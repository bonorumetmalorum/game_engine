use std::collections::HashMap;
use std::any::TypeId;
use entity::EntityIndex;
use core::borrow::BorrowMut;
use std::slice;
use downcast_rs::Downcast;
use std::sync::RwLock;
use std::sync::RwLockWriteGuard;
use std::sync::RwLockReadGuard;
use std::ops::Deref;
use std::ops::DerefMut;

pub struct ComponentWriteHandle<'l, T>{
    pub w: RwLockWriteGuard<'l, T>
}

impl<'a, 'b, S: Storage<'b>> ComponentWriteHandle<'a, S>{
    pub fn get(&'b self, id: EntityIndex) -> &ComponentEntry<S::Component> {
        self.w.deref().get(id)
    }

    pub fn get_mut_iter(&'b mut self) -> S::ComponentIteratorMut {
        self.w.deref_mut().get_mut_iter()
    }
}

pub struct ComponentReadHandle<'l, T> {
    pub r: RwLockReadGuard<'l, T>
}

impl<'a, 'b, S:Storage<'b>> ComponentReadHandle<'a, S>{
    pub fn get(&'b self, id: EntityIndex) -> &ComponentEntry<S::Component> {
        self.r.deref().get(id)
    }

    pub fn get_iterator(&'b self) -> S::ComponentIterator {
        self.r.deref().get_iter()
    }
}

pub trait Component: 'static + Sized + Send + Sync + Clone{
    type ComponentStorage: for<'st> Storage<'st, Component = Self>;
    fn update(&mut self);
}

#[derive(Clone)]
pub enum ComponentEntry<T: Sized + Send + Sync + Clone>{
    Empty,
    Entry(Box<T>)
}

impl<T: Sized + Send + Sync + Clone> ComponentEntry<T> {
    pub fn borrow_mut(&mut self) -> Option<&mut Box<T>> {
        match self {
            ComponentEntry::Entry(val) => Some(val),
            _ => None
        }
    }
}

pub trait Storage<'st>: 'static + Send + Sync + Clone + Default {
    type Component: 'static + Send + Sync + Sized + Clone;
    type ComponentIteratorMut: Iter<Item = &'st mut Box<Self::Component>>;
    type ComponentIterator: Iter<Item = &'st Box<Self::Component>>;
    fn get(&self, id: EntityIndex) -> &ComponentEntry<Self::Component>;
    fn remove(&mut self, EntityIndex) -> Result<EntityIndex, &str>;
    fn get_mut_iter(&'st mut self) -> Self::ComponentIteratorMut;
    fn get_iter(&'st self) -> Self::ComponentIterator;

    fn insert(&mut self, index: EntityIndex, component: Self::Component) -> Result<EntityIndex, &str>;
    fn len(&self) -> usize;
}

pub trait GenericComponentStorage: Send + Sync + Downcast{
    fn remove(&mut self, index: EntityIndex) -> Result<EntityIndex, &str>;
}
impl_downcast!(GenericComponentStorage);

pub struct ComponentStore<T>(pub RwLock<T>);

//switch RwLockWriteGuard to ComponentWrite/Read Handle.
impl<'st, T: Storage<'st>> ComponentStore<T> {
    pub fn write_handle(&self) -> ComponentWriteHandle<T>{
        let result = self.0.write().unwrap();
        ComponentWriteHandle{ w: result }
    }

    pub fn read_handle(&self) -> ComponentReadHandle<T>{
        let result = self.0.read().unwrap();
        ComponentReadHandle{ r: result }
    }

    pub fn get_mut_handle(&mut self) -> &mut T {
        self.0.get_mut().unwrap()
    }
}

impl<'cs, T: 'static + Storage<'cs>> GenericComponentStorage for ComponentStore<T> {
    fn remove(&mut self, index: (usize, u64)) -> Result<(usize, u64), &str> {
        self.0.get_mut().expect("poisoned lock").remove(index)
    }
}

#[derive(Clone)]
pub struct DenseComponentStorage<T: Send + Sync + Clone>(Vec<ComponentEntry<T>>);

impl<T: Component> Default for DenseComponentStorage<T>{
    fn default() -> Self {
        DenseComponentStorage(Vec::new())
    }
}

impl<'it, T: Component> Storage<'it> for DenseComponentStorage<T> {
    type Component = T;
    type ComponentIteratorMut = ComponentIteratorMut<'it, T>;
    type ComponentIterator = ComponentIterator<'it, T>;

    //potentially make this return a result type
    fn get(&self, id: (usize, u64)) -> &ComponentEntry<Self::Component> {
        if let Some(x) = self.0.get(id.0) {
            x
        }else{
            &ComponentEntry::Empty
        }
    }

    fn remove(&mut self, index: EntityIndex) -> Result<(usize, u64), &str> {
        if let Some(reference) = self.0.get_mut(index.0){
            *reference = ComponentEntry::Empty;
            Ok(index)
        }else{
            Err("index out of bounds")
        }
    }

    fn get_mut_iter(&'it mut self) -> Self::ComponentIteratorMut {
        ComponentIteratorMut{current_index: 0, st: self.0.iter_mut()}
    }

    fn get_iter(&'it self) -> Self::ComponentIterator {
        ComponentIterator{ st: self.0.iter(), current_index: 0 }
    }

    fn insert(&mut self, index: (usize, u64), component: Self::Component) -> Result<EntityIndex, &str>{
        if index.0 > self.len(){
            while index.0 > self.len() {
                self.0.push(ComponentEntry::Empty);
            }
            self.0.push(ComponentEntry::Entry(Box::new(component)));
        }else{
            self.0.insert(index.0, ComponentEntry::Entry(Box::new(component)));
        }
        Ok(index)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'it, T: Send + Sync + Clone> DenseComponentStorage<T> {
    pub fn new() -> DenseComponentStorage<T>{
        DenseComponentStorage(Vec::new())
    }
}

pub struct ComponentStorage(
    HashMap<TypeId, Box<GenericComponentStorage>>
);
//I think here i need to store a Box any and store vectors in the any
//this will allow to downcast to a Vec<T> and subsequently get the appropriate iterator.

impl<'st> ComponentStorage {

    pub fn new() -> ComponentStorage {
        ComponentStorage(HashMap::new())
    }

    pub fn register_component<T: Component>(&mut self) -> Result<(usize), &str>{
        let compstrg: DenseComponentStorage<T> = DenseComponentStorage::new();
        let len = compstrg.0.len();
        let componentstore = ComponentStore(RwLock::new(compstrg));
        if let None = self.0.insert(TypeId::of::<T>(), Box::new(componentstore)) {
            Ok(len)
        }else{
            Err("overwritten existing component storage")
        }
    }

    pub fn add_component<T: Component>(&mut self, component: T, id: EntityIndex) -> Result<EntityIndex, &str> {
        if let Ok(storage) = self.get_mut::<T>(){
            let mut store = storage.0.get_mut().unwrap();
            store.insert(id, component);
            Ok(id)
        }else{
            Err("component is not registered")
        }
    }

    pub fn remove_component<T: Component>(&mut self, id: EntityIndex) -> Result<EntityIndex, &str>{
        if let Ok(storage) = self.get_mut::<T>(){
            let mut store = storage.0.get_mut().unwrap();
            if id.0 >= store.len() {
                Err("entity does not have component")
            }else{
                store.remove(id);
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

    pub fn get<T: Component>(&self) -> Result<&ComponentStore<T::ComponentStorage>, &str> {
        if let Some(x) = self.0.get(&TypeId::of::<T>()){
            if let Some(dc) = x.downcast_ref::<ComponentStore<T::ComponentStorage>>() {
                Ok(dc)
            }else{
                Err("downcast failed, type error")
            }
        }else{
            Err("unregistered type")
        }
    }

    pub fn get_mut<T: Component>(&mut self) -> Result<&mut ComponentStore<T::ComponentStorage>, &str> {
        if let Some(x) = self.0.get_mut(&TypeId::of::<T>()){
            if let Some(dc) = x.downcast_mut::<ComponentStore<T::ComponentStorage>>() {
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

//    pub fn get_mut_iterator<T: Component>(&mut self) -> Result<T, &str>{
//        if let Ok(entry) = self.get_mut::<T>(){
//            let mut storage = entry.write_handle();
//            let it = storage.get_mut_iter();
//            Ok(it)
//        }else{
//            Err("Unregistered component")
//        }
//    }

}
//I may be able to get away without implementing a trait for the iterators depending on how I tackle the system impls
//if a system is written using n different iterators then it is up to the user to correctly implement the search used within
pub trait Iter{
    type Item;

    fn next_element(&mut self, until: Option<usize>) -> Option<(Self::Item, usize)>;
    fn join<H: Iter>(mut self, other: H) -> ComponentIteratorJoin<Self, H> where Self: Sized{
        ComponentIteratorJoin(self, other)
    }
    fn into_iterator_wrapper(self) -> IteratorWrapper<Self> where Self: Sized {
        IteratorWrapper(self)
    }
}

pub struct ComponentIteratorJoin<H, T>(H, T);

impl<H: Iter, T: Iter> Iter for ComponentIteratorJoin<H, T> {
    type Item = (H::Item, T::Item);

    fn next_element(&mut self, until: Option<usize>) -> Option<(Self::Item, usize)> {
        match (self.0.next_element(until), self.1.next_element(until)){
            (Some((mut i1, mut ind1)), Some((mut i2, mut ind2))) => loop {
                if ind1 < ind2 {
                    if let Some(res) = self.0.next_element(Some(ind2)) {
                        i1 = res.0;
                        ind1 = res.1;
                    }else{
                        return None;
                    }
                } else if ind1 > ind2 {
                    if let Some(res) = self.1.next_element(Some(ind1)){
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
}

pub struct ComponentIteratorMut<'cs, T: 'cs + Send + Sync + Clone>{
    st: slice::IterMut<'cs, ComponentEntry<T>>,
    current_index: usize
}

impl<'it, T: Component> Iter for ComponentIteratorMut<'it, T>{
    type Item = &'it mut Box<T>;

    fn next_element(&mut self, until: Option<usize>) -> Option<(Self::Item, usize)> {
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
                Some(ComponentEntry::Entry(ref mut v)) => {return Some((v, i))},
                Some(_) => continue,
                None => {return None}
            }

        }
    }
}

impl<'it, T: 'static + Send + Sync + Clone> ComponentIteratorMut<'it, T> {

    pub fn new(it: slice::Iter<'it, ComponentEntry<T>>) -> ComponentIterator<'it, T> {
        ComponentIterator{
            st: it,
            current_index: 0
        }
    }

    pub fn index(&mut self) -> usize {
        self.current_index
    }
}

pub struct ComponentIterator<'cs, T: 'cs + Send + Sync + Clone>{
    st: slice::Iter<'cs, ComponentEntry<T>>,
    current_index: usize
}

impl<'cs, T: Component> Iter for ComponentIterator<'cs, T> {
    type Item = &'cs Box<T>;

    fn next_element(&mut self, until: Option<usize>) -> Option<(Self::Item, usize)> {
        let lim = until.unwrap_or(0);
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
                Some(ComponentEntry::Entry(ref v)) => {return Some((v, i))},
                Some(_) => continue,
                None => {return None}
            }
        }
    }
}

pub struct IteratorWrapper<H>(H);

impl<H> Iterator for IteratorWrapper<H> where H: Iter{
    type Item = H::Item;

    fn next(&mut self) -> Option<Self::Item> {
            if let Some(val) = self.0.next_element(None){
            Some(val.0)
        }else{
            None
        }
    }
}

#[derive(Clone)]
pub struct StubPosition{
    pub x: f32,
    pub y: f32,
}

impl Component for StubPosition{
    type ComponentStorage = DenseComponentStorage<Self>;

    fn update(&mut self) {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct StubVelocity{
    pub dx: f32,
    pub dy: f32,
}

impl Component for StubVelocity{
    type ComponentStorage = DenseComponentStorage<Self>;

    fn update(&mut self) {
        unimplemented!()
    }
}