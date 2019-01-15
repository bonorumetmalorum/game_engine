use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;
use entity::EntityIndex;

pub trait Component{
    fn update(&mut self);
}

pub struct ComponentStorage(HashMap<TypeId, Vec<Option<Box<Any>>>>);

impl ComponentStorage {

    pub fn new() -> ComponentStorage {
        ComponentStorage(HashMap::new())
    }

    pub fn register_component<T:'static>(&mut self) -> Result<(), &str>{
        let component_storage: Vec<Option<Box<Any>>> = Vec::new();
        if let None = self.0.insert(TypeId::of::<T>(), component_storage) {
            Ok(())
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

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Iterator for ComponentStorage {
    type Item = //need to add abstraction here, a component scanner type which is instantiated with a type
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unimplemented!()
    }
}