use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;
use entity::EntityIndex;
use core::borrow::Borrow;
use core::borrow::BorrowMut;

pub trait Component{
    fn update(&mut self);
}

type Storage = Vec<Option<Box<Any>>>;

pub struct ComponentStorage(HashMap<TypeId, Vec<Option<Box<Any>>>>);

impl ComponentStorage {

    pub fn new() -> ComponentStorage {
        ComponentStorage(HashMap::new())
    }

    pub fn register_component<T:'static>(&mut self) -> Result<(usize), &str>{
        let component_storage: Vec<Option<Box<Any>>> = Vec::new();
        if let None = self.0.insert(TypeId::of::<T>(), component_storage) {
            Ok((component_storage.len()))
        }else{
            Err("overwritten existing component storage")
        }
    }

    pub fn add_component<T:'static>(&mut self, component: T, id: EntityIndex) -> Result<EntityIndex, &str> {
        if let Some(storage) = self.0.get_mut(&TypeId::of::<T>()){
            while id.0 >= storage.len() {
                storage.push(None);
            }
            storage[id.0] = Some(Box::new(component));
            Ok(id)
        }else{
            Err("component is not registered")
        }
    }

    pub fn remove_component<T:'static>(&mut self, id: EntityIndex) -> Result<EntityIndex, &str>{
        if let Some(storage) = self.0.get_mut(&TypeId::of::<T>()){
            if id.0 >= storage.len() {
                Err("entity does not have component")
            }else{
                storage[id.0] = None;
                Ok(id)
            }
        }else{
            Err("component is not registered")
        }
    }

    pub fn clear_entity(&mut self, id: EntityIndex) -> Result<(), &str> {
        for (_, cs) in self.0.borrow_mut() {
            if id.0 > cs.len() {
                continue;
            }else{
                cs[id.0] = None;
            }
        }
        Ok(())
    }

    pub fn get<T>(&self) -> &Vec<Option<Box<T>>> {
        self.0.get(&TypeId::of::<T>())
    }

    pub fn get_mut<T>(&mut self) -> &mut Vec<Option<Box<T>>> {
        &self.0[&TypeId::of::<T>()]
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

//pub struct ComponentIterator<'cs, T>{st: &'cs Storage, current_index: usize}
//
//impl<'cs, T> Iterator for ComponentIterator<'cs, T> {
//    type Item = T;
//
//    fn next(&mut self) -> Option<&mut Self::Item> {
//        self.st.get_mut(self.current_index)
//    }
//}