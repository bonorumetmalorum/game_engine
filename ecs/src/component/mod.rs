use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;
use entity::EntityIndex;
use core::borrow::BorrowMut;
use std::slice;

pub trait Component: Any{
    fn update(&mut self);
}

pub struct StubPosition{
    pub x: f32,
    pub y: f32,
}

pub struct StubVelocity {
    dx: f32,
    dy: f32,
}
pub enum ComponentEntry{
    Empty,
    Entry(Box<Any>)
}

pub struct ComponentStorage(HashMap<TypeId, Vec<ComponentEntry>>);

impl ComponentStorage {

    pub fn new() -> ComponentStorage {
        ComponentStorage(HashMap::new())
    }

    pub fn register_component<T:'static>(&mut self) -> Result<(usize), &str>{
        let component_storage: Vec<ComponentEntry> = Vec::new();
        let len = component_storage.len();
        if let None = self.0.insert(TypeId::of::<T>(), component_storage) {
            Ok(len)
        }else{
            Err("overwritten existing component storage")
        }
    }

    pub fn add_component<T:'static>(&mut self, component: T, id: EntityIndex) -> Result<EntityIndex, &str> {
        if let Some(storage) = self.0.get_mut(&TypeId::of::<T>()){
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
        if let Some(storage) = self.0.get_mut(&TypeId::of::<T>()){
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
        for (_, cs) in self.0.borrow_mut() {
            if id.0 > cs.len() {
                continue;
            }else{
                cs[id.0] = ComponentEntry::Empty;
            }
        }
        Ok(())
    }

    pub fn get<T: 'static>(&self) -> Result<&Vec<ComponentEntry>, &str> {
        if let Some(x) = self.0.get(&TypeId::of::<T>()){
            Ok(x)
        }else{
            Err("unregistered type")
        }
    }

    pub fn get_mut<T: 'static>(&mut self) -> Result<&mut Vec<ComponentEntry>, &str> {
        if let Some(x) = self.0.get_mut(&TypeId::of::<T>()){

            Ok(x)
        }else{
            Err("unregistered type")
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_mut_iterator<T: 'static>(&mut self) -> Result<ComponentIterator, &str>{
        if let Some(entry) = self.0.get_mut(&TypeId::of::<T>()){
            let it = entry.iter_mut();
            Ok(ComponentIterator{st: it, current_index: 0})
        }else{
            Err("Unregistered component")
        }
    }

}

pub struct ComponentIterator<'cs>{
    st: slice::IterMut<'cs, ComponentEntry>,
    current_index: usize
}

impl<'it> ComponentIterator<'it> {

    fn new(it: slice::IterMut<'it, ComponentEntry>) -> ComponentIterator<'it> {
        ComponentIterator{
            st: it,
            current_index: 0
        }
    }

    fn next(&mut self) -> Option<&mut ComponentEntry> {
        self.current_index += 1;
        self.st.next()
    }
}