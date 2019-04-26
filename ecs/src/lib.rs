pub mod component;
pub mod entity;
pub mod resource;
#[cfg(test)]
mod tests;

extern crate core;
#[macro_use]
extern crate downcast_rs;
use component::ComponentStorage;
use entity::management::EntityAllocator;
use entity::EntityIndex;
use std::any::Any;
use component::Component;
use component::ComponentReadHandle;
use component::ComponentWriteHandle;
use entity::management::EntityIterator;
use entity::management::EntityIteratorLive;
use resource::ResourceWriteHandle;
use resource::ResourceReadHandle;
use resource::Resource;
use resource::ResourceMap;

//generational data structure
pub struct ECS {
    pub storage: ComponentStorage,
    pub entity_list: EntityAllocator,
    pub resources: ResourceMap,
    pub size: usize
}

impl<'cs> ECS {

    pub fn allocate_new_entity(&mut self) -> EntityIndex {
        self.size += 1;
        let entity = self.entity_list.allocate();
        entity
    }

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

    pub fn add_component<T: Component>(&mut self, index: EntityIndex, component: T) -> Result<EntityIndex, &str>{
        if index.1 == self.entity_list.entity_list[index.0].generation && self.entity_list.entity_list[index.0].is_live {
            self.storage.add_component(component, index)
        }else{
            Err("incorrect generation")
        }
    }

    pub fn register_new_component<T: Component>(&mut self) -> Result<usize, &str> {
        let mut component_storage: Vec<Option<Box<Any>>> = Vec::with_capacity(self.size);
        for _i in 0 .. self.size {
            component_storage.push(None);
        }
        self.storage.register_component::<T>()
    }

    pub fn remove_component<T: Component>(&mut self, index: EntityIndex) -> Result<EntityIndex, &str>{
        if index.1 != self.entity_list.entity_list[index.0].generation && !self.entity_list.entity_list[index.0].is_live {
            Err("invalid index")
        }else{
            self.storage.remove_component::<T>(index)
        }
    }

    pub fn get_component_read_handle<T: 'static + Component>(&self) -> ComponentReadHandle<T::ComponentStorage> {
        let res = self.storage.get::<T>().unwrap();
        let strg = res.0.read().unwrap();
        ComponentReadHandle{ r: strg }
    }

    pub fn get_component_write_handle<T: 'static + Component>(&self) -> ComponentWriteHandle<T::ComponentStorage> {
        let res = self.storage.get::<T>().unwrap();
        let strg = res.0.write().unwrap();
        ComponentWriteHandle{ w: strg }
    }

    pub fn get_mut<T: Component>(&mut self) -> &mut T::ComponentStorage{
        let res = self.storage.get_mut::<T>().unwrap();
        let component = res.0.get_mut().unwrap();
        component
    }

    pub fn get_entity_iterator_live(&self) -> EntityIteratorLive {
        self.entity_list.get_iter_live()
    }

    pub fn get_entity_iterator(&self) -> EntityIterator {
        self.entity_list.get_iter()
    }

    pub fn get_mut_resource<T: 'static>(&self) -> Result<ResourceWriteHandle<T>, &str>{
        match self.resources.get_write_resource::<T>() {
            Ok(x) => Ok(x),
            Err(e) => Err(e)
        }
    }

    pub fn get_resource<T: 'static>(&self) -> Result<ResourceReadHandle<T>, &str>{
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

    pub fn new() -> ECS {
        ECS {storage: ComponentStorage::new(), entity_list: EntityAllocator::new(), resources: ResourceMap::default(), size: 0}
    }
}
