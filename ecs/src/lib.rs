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
use component::Component;
use std::any::{Any, TypeId};
use entity::management::EntityIteratorLive;
use entity::management::EntityIterator;
use std::cell::{RefMut, RefCell};
use std::cell::Ref;
use std::collections::HashMap;
use downcast_rs::Downcast;

pub struct ResourceMap{
    map: HashMap<Any, ResourceEntry>
}

pub trait ResourceEntry: Downcast + Sized {}
impl_downcast!(ResourceEntry);

pub struct Resource<T>(RefCell<T>);

impl ResourceMap{
    pub fn get_write_resource<T>(&self) -> RefMut<T>{
        if let Some(x) = self.map.get(&TypeId::of::<T>()){
            if let Some(downcast) = x.downcast_ref::<Resource<T>>(){
                //do something
            }else{
                //err
            }

        }else{
            //err
        }
    }

    pub fn get_read_resource<T>(&self) -> Ref<T>{
        self.map.get(&TypeId::of::<T>()).get()
    }

    pub fn insert_resource<T>(){

    }
}

impl<T> Resource<T> {
    pub fn get_mut(&self) -> RefMut<T> {
        self.0.borrow_mut()
    }

    pub fn get(&self) -> Ref<T> {
        self.0.borrow()
    }
}

//maybe add anymap for shared resources here?
pub struct ECS {
    pub storage: ComponentStorage,
    pub entity_list: EntityAllocator,
    pub resources: ResourceMap,
    pub size: usize
}

impl<'cs> ECS {

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
    pub fn add_component<T: Component>(&mut self, index: EntityIndex, component: T) -> Result<EntityIndex, &str>{
        if index.1 == self.entity_list.entity_list[index.0].generation && self.entity_list.entity_list[index.0].is_live {
            self.storage.add_component(component, index)
        }else{
            Err("incorrect generation")
        }
    }

    /*
    register a new component to the manager
    */
    pub fn register_new_component<T: Component>(&mut self) -> Result<usize, &str> {
        let mut component_storage: Vec<Option<Box<Any>>> = Vec::with_capacity(self.size);
        for _i in 0 .. self.size {
            component_storage.push(None);
        }
        self.storage.register_component::<T>()
    }

    /*
        remove a component from an entity
    */
    pub fn remove_component<T: Component>(&mut self, index: EntityIndex) -> Result<EntityIndex, &str>{
        if index.1 != self.entity_list.entity_list[index.0].generation && !self.entity_list.entity_list[index.0].is_live {
            Err("invalid index")
        }else{
            self.storage.remove_component::<T>(index)
        }
    }

//    pub fn get_component_read_handle<T: 'static + Component>(&self) -> SyncReadHandle<T::ComponentStorage> {
//        let res = self.storage.get::<T>().unwrap();
//        let strg = &res.0;
//        SyncReadHandle(strg)
//    }
//
//    pub fn get_component_write_handle<T: 'static + Component>(&self) -> T::ComponentStorage {
//        let res = self.storage.get::<T>().unwrap();
//        let strg = &res.0;
//        SyncWriteHandle(strg)
//    }

    pub fn get<T: Component>(&self) -> Ref<T::ComponentStorage>{
        let res = self.storage.get::<T>().unwrap();
        res.read()
    }

    pub fn get_mut<T: Component>(&self) -> RefMut<T::ComponentStorage>{
        let res = self.storage.get::<T>().unwrap();
        res.write()
    }

    pub fn get_entity_iterator_live(&self) -> EntityIteratorLive {
        self.entity_list.get_iter_live()
    }

    pub fn get_entity_iterator(&self) -> EntityIterator {
        self.entity_list.get_iter()
    }

    /*
    returns a new empty entity storage
    */
    pub fn new() -> ECS {
        ECS {storage: ComponentStorage::new(), entity_list: EntityAllocator::new(), size: 0}
    }
}
