pub mod component;
pub mod entity;
pub mod system;
#[cfg(test)]
mod tests;

extern crate core;

use component::ComponentStorage;
use entity::management::EntityAllocator;
use entity::EntityIndex;
use std::any::Any;
use core::borrow::BorrowMut;

//generational data structure
pub struct EntityStorage {
    pub storage: ComponentStorage,
    pub entity_list: EntityAllocator,
    pub size: usize
}

impl EntityStorage{

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
                Ok(_) => {for (_, comp) in self.storage.0.borrow_mut() {comp[id.0] = None}; Ok(())},
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
            if let Some(comp) = self.storage.0.get_mut(&TypeId::of::<T>()) {
                if let Some(None) = comp.get_mut(index.0) {
                    comp[index.0] = Some(Box::new(component));
                    Ok(index)
                } else {
                    Err("entity does not exist")
                }
            } else {
                Err("unregistered component, please register before adding")
            }
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
        let size = component_storage.len();
        if let None = self.storage.0.insert(TypeId::of::<T>(), component_storage) {
            Ok(size)
        }else{
            Err("overwritten existing component storage")
        }
    }

    /*
        remove a component from an entity
    */
    pub fn remove_component<T: 'static>(&mut self, index: EntityIndex) -> Result<EntityIndex, &str>{
        if index.1 != self.entity_list.entity_list[index.0].generation && !self.entity_list.entity_list[index.0].is_live {
            Err("invalid index")
        }else{
            if let Some(x) = self.storage.0.get_mut(&TypeId::of::<T>()){
                x[index.0] = None;
            }
            Ok(index)
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

    pub fn find_entities()  {
        /*
        this method needs to return an iterator which will go through each entity that meets the condition (has the components)
        */
        unimplemented!()
    }

    /*
    returns a new empty entity storage
    */
    pub fn new() -> EntityStorage {
        EntityStorage{storage: ComponentStorage::new(), entity_list: EntityAllocator::new(), size: 0}
    }
}

