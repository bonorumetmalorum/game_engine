pub mod component;
pub mod entity;
pub mod system;
#[cfg(test)]
mod tests;

extern crate core;
#[macro_use]
extern crate downcast_rs;

use component::ComponentStorage;
use entity::management::EntityAllocator;
use entity::EntityIndex;
use std::any::Any;
use component::ComponentIterator;

//generational data structure
pub struct ECS {
    pub storage: ComponentStorage,
    pub entity_list: EntityAllocator,
    pub size: usize
}

impl ECS {

    /*
    allocate a new empty entity
    */
    pub fn allocate_new_entity(&mut self) -> EntityIndex {
        self.size += 1;
        let entity = self.entity_list.allocate();
        entity
    }

    /*
    removes an entity, returning its index to the pool for a new entity to be allocated
    removes all of its components
    */
    pub fn deallocate_entity(&mut self, id: EntityIndex) -> Result<(), &str> {
        self.size -= 1;
        if id.1 == self.entity_list.entity_list[id.0].generation && self.entity_list.entity_list[id.0].is_live{
            let entity = self.entity_list.deallocate(id);
            match entity {
                Ok(_) => {self.storage.clear_entity(id)},
                Err(e) =>  Err(e)
            }
        }else{
            Err("incorrect generation")
        }
    }

    /*
    adds a component to an entity
    if the component has not been registered to the manager, it will panic
    */
    pub fn add_component<T: 'static>(&mut self, index: EntityIndex, component: T) -> Result<EntityIndex, &str>{
        if index.1 == self.entity_list.entity_list[index.0].generation && self.entity_list.entity_list[index.0].is_live {
            self.storage.add_component(component, index)
        }else{
            Err("incorrect generation")
        }
    }

    /*
    register a new component to the manager
    */
    pub fn register_new_component<T: 'static>(&mut self) -> Result<usize, &str> {
        let mut component_storage: Vec<Option<Box<Any>>> = Vec::with_capacity(self.size);
        for _i in 0 .. self.size {
            component_storage.push(None);
        }
        self.storage.register_component::<T>()
    }

    /*
        remove a component from an entity
    */
    pub fn remove_component<T: 'static>(&mut self, index: EntityIndex) -> Result<EntityIndex, &str>{
        if index.1 != self.entity_list.entity_list[index.0].generation && !self.entity_list.entity_list[index.0].is_live {
            Err("invalid index")
        }else{
            self.storage.remove_component::<T>(index)
        }
    }

    /*
    gives a mutable reference to an entities component for updating
    */
//    pub fn fetch<T: 'static>(&mut self, id: EntityIndex) -> Result<Option<&mut T>, &str> {
//        if id.1 != self.entity_list.entity_list[id.0].generation && !self.entity_list.entity_list[id.0].is_live{
//            Err("incorrect generation")
//        }else{
//            let component = self.storage.get_mut(&TypeId::of::<T>()).unwrap();
//            let unwrapped_component = component[id.0].as_mut().unwrap();
//            let downcast: Option<&mut T> = unwrapped_component.downcast_mut::<T>();
//            Ok(downcast)
//        }
//    }

    pub fn iterator<T: 'static>(&mut self) -> ComponentIterator<T>{
        self.storage.get_mut_iterator::<T>().expect("Error creating iterator")
    }

    /*
    returns a new empty entity storage
    */
    pub fn new() -> ECS {
        ECS {storage: ComponentStorage::new(), entity_list: EntityAllocator::new(), size: 0}
    }
}

