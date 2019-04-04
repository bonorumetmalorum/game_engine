pub mod component;
pub mod entity;
pub mod system;
#[cfg(test)]
mod tests;

extern crate core;
#[macro_use]
extern crate downcast_rs;
extern crate fnv;

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
    map: HashMap<TypeId, Box<ResourceEntry>>
}

pub trait ResourceEntry: Downcast {}
impl_downcast!(ResourceEntry);

pub struct Resource<T>(RefCell<T>);

impl ResourceMap{

    pub fn get_write_resource<T:'static>(&self) -> Result<RefMut<T>, &str>{
        if let Some(x) = self.map.get(&TypeId::of::<T>()){
            if let Some(downcast) = x.downcast_ref::<Resource<T>>(){
                Ok(downcast.get_mut())
            }else{
                Err("unable to downcast")
            }

        }else{
            Err("resource does not exist")
        }
    }

    pub fn get_read_resource<T:'static>(&self) -> Result<Ref<T>, &str>{
        if let Some(entry) = self.map.get(&TypeId::of::<T>()) {
            if let Some(t) = entry.downcast_ref::<Resource<T>>() {
                Ok(t.get())
            }else{
                Err("unable to downcast")
            }
        }else{
            Err("resource does not exist")
        }
    }

    pub fn insert_resource<T:'static>(&mut self, resource: T){
        self.map.insert(TypeId::of::<T>(), Box::new(Resource(RefCell::new(resource))));
    }

    pub fn remove_resource<T:'static>(&mut self) -> Result<Resource<T>, &str> {
        match self.map.remove(&TypeId::of::<T>()) {
            Some(x) => {
                match x.downcast::<Resource<T>>() {
                    Ok(x) => Ok(*x),
                    Err(s) => Err("error downcasting removed type")
                }
            },
            None => Err("resource does not exist")
        }
    }
}

impl Default for ResourceMap {
    fn default() -> Self {
        ResourceMap{
            map: HashMap::new()
        }
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

impl<T:'static> ResourceEntry for Resource<T> {}

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

    pub fn get_mut_resource<T: 'static>(&self) -> Result<RefMut<T>, &str>{
        match self.resources.get_write_resource::<T>() {
            Ok(x) => Ok(x),
            Err(e) => Err(e)
        }
    }

    pub fn get_resource<T: 'static>(&self) -> Result<Ref<T>, &str>{
        match self.resources.get_read_resource::<T>() {
            Ok(x) => Ok(x),
            Err(e) => Err(e)
        }
    }

    pub fn remove_resource<T:'static>(&mut self) -> Result<Resource<T>, &str>{
        match self.resources.remove_resource::<T>() {
            Err(e) => Err(e),
            Ok(x) => Ok(x)
        }
    }

    pub fn insert_new_resource<T:'static>(&mut self, resource: T){
        self.resources.insert_resource::<T>(resource);
    }

    /*
    returns a new empty entity storage
    */
    pub fn new() -> ECS {
        ECS {storage: ComponentStorage::new(), entity_list: EntityAllocator::new(), size: 0, resources: ResourceMap::default()}
    }
}
