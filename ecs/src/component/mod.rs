use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;
use entity::EntityIndex;
use core::borrow::BorrowMut;
use std::slice;
use downcast_rs::Downcast;

pub trait Component: Downcast{
    fn update(&mut self);
}

impl_downcast!(Component);

#[derive(PartialEq, Eq, Debug)]
pub enum ComponentEntry<T: ?Sized>{
    Empty,
    Entry(Box<T>)
}

impl<T> ComponentEntry<T> {
    pub fn borrow_mut(&mut self) -> Option<&mut Box<T>> {
        match self {
            ComponentEntry::Entry(val) => Some(val),
            _ => None
        }
    }
}

trait Storage: Downcast {
    fn remove(&mut self, EntityIndex) -> Result<EntityIndex, &str>;
}
impl_downcast!(Storage);

pub struct DenseComponentStorage<T>(Vec<ComponentEntry<T>>);

impl<T: 'static> Storage for DenseComponentStorage<T> {

    fn remove(&mut self, index: EntityIndex) -> Result<(usize, u64), &str> {
        if let Some(reference) = self.0.get_mut(index.0){
            *reference = ComponentEntry::Empty;
            Ok(index)
        }else{
            Err("index out of bounds")
        }
    }
}

impl<T> DenseComponentStorage<T> {
    pub fn new() -> DenseComponentStorage<T>{
        DenseComponentStorage(Vec::new())
    }
}

pub struct ComponentStorage(HashMap<TypeId, Box<Storage>>); //I think here i need to store a Box any and store vectors in the any
//this will allow to downcast to a Vec<T> and subsequently get the appropriate iterator.

impl ComponentStorage {

    pub fn new() -> ComponentStorage {
        ComponentStorage(HashMap::new())
    }

    pub fn register_component<T:'static>(&mut self) -> Result<(usize), &str>{
        let compstrg: DenseComponentStorage<T> = DenseComponentStorage::new();
        let len = compstrg.0.len();
        if let None = self.0.insert(TypeId::of::<T>(), Box::new(compstrg)) {
            Ok(len)
        }else{
            Err("overwritten existing component storage")
        }
    }

    pub fn add_component<T:'static>(&mut self, component: T, id: EntityIndex) -> Result<EntityIndex, &str> {
        if let Ok(storage) = self.get_mut::<T>(){
            while id.0 > storage.0.len() {
                storage.0.push(ComponentEntry::Empty);
            }
            storage.0.push( ComponentEntry::Entry(Box::new(component)));
            Ok(id)
        }else{
            Err("component is not registered")
        }
    }

    pub fn remove_component<T:'static>(&mut self, id: EntityIndex) -> Result<EntityIndex, &str>{
        if let Ok(storage) = self.get_mut::<T>(){
            if id.0 >= storage.0.len() {
                Err("entity does not have component")
            }else{
                storage.0[id.0] = ComponentEntry::Empty;
                Ok(id)
            }
        }else{
            Err("component is not registered")
        }
    }

    pub fn clear_entity(&mut self, id: EntityIndex) -> Result<(), &str> {
        let mut status = Ok(());
        for (t, cs) in self.0.borrow_mut() {
            if let Ok(res) = cs.remove(id) {
                continue;
            }else{
                status = Err("Entity does not exist");
                break;
            }
        }
        status
    }

    pub fn get<T: 'static>(&self) -> Result<&DenseComponentStorage<T>, &str> {
        if let Some(x) = self.0.get(&TypeId::of::<T>()){
            if let Some(dc) = x.downcast_ref::<DenseComponentStorage<T>>() {
                Ok(dc)
            }else{
                Err("downcast failed, type error")
            }
        }else{
            Err("unregistered type")
        }
    }

    pub fn get_mut<T: 'static>(&mut self) -> Result<&mut DenseComponentStorage<T>, &str> {
        if let Some(x) = self.0.get_mut(&TypeId::of::<T>()){
            if let Some(dc) = x.downcast_mut::<DenseComponentStorage<T>>() {
                Ok(dc)
            }else{
                Err("downcast failed, type error")
            }
        }else{
            Err("unregistered type")
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_mut_iterator<T: 'static>(&mut self) -> Result<ComponentIterator<T>, &str>{
        if let Ok(entry) = self.get_mut::<T>(){
            let it = entry.0.iter_mut();
            Ok(ComponentIterator{st: it, current_index: 0})
        }else{
            Err("Unregistered component")
        }
    }

}
//I may be able to get away without implementing a trait for the iterators depending on how I tackle the system impls
//if a system is written using n different iterators then it is up to the user to correctly implement the search used within
pub trait Iter{
    type Item;

    fn next(&mut self, until: Option<usize>) -> Option<(Self::Item, usize)>;
    fn into_vec(mut self) -> Vec<Self::Item>;
}

pub struct ComponentIteratorJoin<H, T>(H, T);

impl<H: Iter, T: Iter> Iter for ComponentIteratorJoin<H, T> {
    type Item = (H::Item, T::Item);

    fn next(&mut self, until: Option<usize>) -> Option<(Self::Item, usize)> {
        match (self.0.next(until), self.1.next(until)){
            (Some((mut i1, mut ind1)), Some((mut i2, mut ind2))) => loop {
                if ind1 < ind2 {
                    if let Some(res) = self.0.next(Some(ind2)) {
                        i1 = res.0;
                        ind1 = res.1;
                    }else{
                        return None;
                    }
                } else if ind1 > ind2 {
                    if let Some(res) = self.1.next(Some(ind1)){
                        i2 = res.0;
                        ind2 = res.1;
                    }else{
                        return None;
                    }
                }else{
                    return Some(((i1, i2), ind1))
                }
            },
            _ => None
        }
    }

    fn into_vec(mut self) -> Vec<Self::Item> {
        unimplemented!()
    }
}

pub struct ComponentIterator<'cs, T: 'cs>{
    st: slice::IterMut<'cs, ComponentEntry<T>>,
    current_index: usize
}

//maybe implement Iterator trait for ComponentIterator to allow for a better interface

impl<'it, T> Iter for ComponentIterator<'it, T>{
    type Item = &'it mut ComponentEntry<T>;

    fn next(&mut self, until: Option<usize>) -> Option<(Self::Item, usize)> {
        let mut lim = until.unwrap_or(0);
        loop{
            let r;
            let i;
            if lim > self.current_index {
                r = self.st.nth(lim - self.current_index);
                i = lim;
                self.current_index = lim + 1;
            }else{
                r = self.st.next();
                i = self.current_index;
                self.current_index += 1;
            }

            match r {
                Some(ComponentEntry::Entry(_)) => {return Some((r.unwrap(), i))},
                _ => {return None}
            }

        }
    }

    fn into_vec(mut self) -> Vec<Self::Item> {
        let mut result: Vec<&mut ComponentEntry<T>> = vec![];
        loop{
            match self.st.next() {
                Some(val) => result.push(val),
                None => break
            }
        }
        result
    }
}

impl<'it, T: 'static> ComponentIterator<'it, T> {

    pub fn new(it: slice::IterMut<'it, ComponentEntry<T>>) -> ComponentIterator<'it, T> {
        ComponentIterator{
            st: it,
            current_index: 0
        }
    }

    pub fn join<H>(&mut self, other: ComponentIterator<H>) -> ComponentIteratorJoin<T, H> {
        unimplemented!()
    }

    pub fn index(&mut self) -> usize {
        self.current_index
    }
}

pub struct StubPosition{
    pub x: f32,
    pub y: f32,
}

pub struct StubVelocity{
    dx: f32,
    dy: f32,
}