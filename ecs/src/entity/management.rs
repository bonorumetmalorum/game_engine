use super::*;
use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;
use core::borrow::BorrowMut;

//entry to define an allocation into a generational data structure
pub struct Entry {
    pub is_live: bool,
    pub generation: u64
}

//the reason for this abstraction is to allow for the Iterator trait to be implemented on this data structure. easily...
pub struct EntityAllocator {
    pub entity_list: Vec<Entry>,
    pub free_list: Vec<usize>,
}

impl EntityAllocator {

    pub fn new() -> EntityAllocator {
        EntityAllocator{
            entity_list: Vec::new(),
            free_list: Vec::new()
        }
    }

    pub fn allocate(&mut self) -> EntityIndex {
        if let Some(x) = self.free_list.pop() {
            let mut index = &mut self.entity_list[x];
            index.is_live = true;
            index.generation += 1;
            (x, index.generation)
        }else{
            self.entity_list.push(Entry { is_live: true, generation: 0 });
            (self.entity_list.len() - 1, 0)
        }
    }

    pub fn deallocate(&mut self, id: EntityIndex) -> Result<(), &str> {
        if id.1 == self.entity_list[id.0].generation {
            self.entity_list[id.0].is_live = false;
            self.free_list.push(id.0);
            Ok(())
        }else{
            Err("incorrect generation")
        }
    }
}

pub struct ComponentStorage(HashMap<TypeId, Vec<Option<Box<Any>>>>);

impl ComponentStorage {

    pub fn register_component<T:'static>(&mut self) -> Result<usize, &str>{
        let mut component_storage: Vec<Option<Box<Any>>> = Vec::new();
        if let None = self.storage.insert(TypeId::of::<T>(), component_storage) {
            Ok(size)
        }else{
            Err("overwritten existing component storage")
        }
    }

    pub fn add_component<T:'static>(&mut self, component: T, id: EntityIndex) -> Result<EntityIndex, &str> {
        if let Some(storage) = self.0.get_mut(&TypeId::of::<T>()){
            while id.0 >= storage.len() {
                storage.push(None);
            }
            storage[id.0] = component;
            Ok(id)
        }else{
            Err("component is not registered")
        }
    }

    pub fn remove_component<T:'static>(&mut self, id: EntityIndex) -> Result<EntityIndex, &str>{
        if let Some(storage) = self.0.get_mut(&TypeId::of::<T>()){
            if id.0 >= storage.length {
                Err("entity does not have component")
            }else{
                storage[id.0] = None;
                Ok(id)
            }
        }else{
            Err("component is not registered")
        }
    }
}

//generational data structure
pub struct EntityStorage {
    pub storage: HashMap<TypeId, Vec<Option<Box<Any>>>>,
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
        if entity.1 == 0 {
            for (_, val) in self.storage.borrow_mut() {
                val.push(None);
            }
        }
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
                Ok(_) => {for (_, comp) in self.storage.borrow_mut() {comp[id.0] = None}; Ok(())},
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
            if let Some(comp) = self.storage.get_mut(&TypeId::of::<T>()) {
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
        if let None = self.storage.insert(TypeId::of::<T>(), component_storage) {
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
            if let Some(x) = self.storage.get_mut(&TypeId::of::<T>()){
                x[index.0] = None;
            }
            Ok(index)
        }
    }

    /*
    gives a mutable reference to an entities component for updating
    */
    pub fn fetch<T: 'static>(&mut self, id: EntityIndex) -> Result<Option<&mut T>, &str> {
        if id.1 != self.entity_list.entity_list[id.0].generation && !self.entity_list.entity_list[id.0].is_live{
            Err("incorrect generation")
        }else{
            let component = self.storage.get_mut(&TypeId::of::<T>()).unwrap();
            let unwrapped_component = component[id.0].as_mut().unwrap();
            let downcast: Option<&mut T> = unwrapped_component.downcast_mut::<T>();
            Ok(downcast)
        }
    }

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
        EntityStorage{storage: HashMap::new(), entity_list: EntityAllocator::new(), size: 0}
    }
}