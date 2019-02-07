use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;
use entity::EntityIndex;
use core::borrow::BorrowMut;
use std::slice;

pub trait Component: Any{
    fn update(&mut self);
}

pub enum ComponentEntry<T: ?Sized>{
    Empty,
    Entry(Box<T>)
}

pub struct ComponentStorage(HashMap<TypeId, Box<Any>>); //I think here i need to store a Box any and store vectors in the any
//this will allow to downcast to a Vec<T> and subsequently get the appropriate iterator.

impl ComponentStorage {

    pub fn new() -> ComponentStorage {
        ComponentStorage(HashMap::new())
    }

    pub fn register_component<T:'static>(&mut self) -> Result<(usize), &str>{
        let component_storage: Vec<ComponentEntry<Any>> = Vec::new();
        let len = component_storage.len();
        if let None = self.0.insert(TypeId::of::<T>(), Box::new(component_storage)) {
            Ok(len)
        }else{
            Err("overwritten existing component storage")
        }
    }

    pub fn add_component<T:'static>(&mut self, component: T, id: EntityIndex) -> Result<EntityIndex, &str> {
        if let Ok(storage) = self.get_mut::<T>(){
            while id.0 >= storage.len() {
                storage.push(ComponentEntry::Empty);
            }
            storage[id.0] = ComponentEntry::Entry(Box::new(component));
            Ok(id)
        }else{
            Err("component is not registered")
        }
    }

    pub fn remove_component<T:'static>(&mut self, id: EntityIndex) -> Result<EntityIndex, &str>{
        if let Ok(storage) = self.get_mut::<T>(){
            if id.0 >= storage.len() {
                Err("entity does not have component")
            }else{
                storage[id.0] = ComponentEntry::Empty;
                Ok(id)
            }
        }else{
            Err("component is not registered")
        }
    }

    pub fn clear_entity(&mut self, id: EntityIndex) -> Result<(), &str> {
        for (t, cs) in self.0.borrow_mut() {
            let mut dc = cs.downcast_mut::<Vec<ComponentEntry<Any>>>().unwrap();
            if id.0 > dc.len() {
                continue;
            }else{
                dc[id.0] = ComponentEntry::Empty;
            }
        }
        Ok(())
    }

    pub fn get<T: 'static>(&self) -> Result<&Vec<ComponentEntry<T>>, &str> {
        if let Some(x) = self.0.get(&TypeId::of::<T>()){
            if let Some(dc) = x.downcast_ref::<Vec<ComponentEntry<T>>>() {
                Ok(dc)
            }else{
                Err("downcast failed, type error")
            }
        }else{
            Err("unregistered type")
        }
    }

    pub fn get_mut<T: 'static>(&mut self) -> Result<&mut Vec<ComponentEntry<T>>, &str> {
        if let Some(x) = self.0.get_mut(&TypeId::of::<T>()){
            if let Some(dc) = x.downcast_mut::<Vec<ComponentEntry<T>>>() {
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
            let it = entry.iter_mut();
            Ok(ComponentIterator{st: it, current_index: 0})
        }else{
            Err("Unregistered component")
        }
    }

}

pub struct ComponentIteratorJoin<'it, H: 'it, T: 'it>(ComponentIterator<'it, H>, ComponentIterator<'it, T>);

pub struct ComponentIterator<'cs, T: 'cs>{
    st: slice::IterMut<'cs, ComponentEntry<T>>,
    current_index: usize
}

impl<'it, T: 'static> ComponentIterator<'it, T> {

    pub fn new(it: slice::IterMut<'it, ComponentEntry<T>>) -> ComponentIterator<'it, T> {
        ComponentIterator{
            st: it,
            current_index: 0
        }
    }

    pub fn next(&mut self) -> Option<&mut T> {
        unimplemented!()
    }

    pub fn join<H>(&mut self, other: ComponentIterator<H>) -> ComponentIteratorJoin<T, H> {
        unimplemented!()
    }

//    pub fn into_vec(self) -> Vec<Option<&mut T>> {
//        if self.current_index > self.st.col {
//            self.cache
//        }else{
//            loop{
//                self.next
//            }
//        }
//    }

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