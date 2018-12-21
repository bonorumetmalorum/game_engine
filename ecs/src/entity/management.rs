use super::*;
use component::*;
use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;

//entry to define an allocation into a generational data structure
pub struct Entry {
    pub is_live: bool,
    pub generation: u64
}

//generational data structure
pub struct EntityStorage {
    pub storage: HashMap<TypeId, Vec<Option<Box<Any>>>>,
    pub entity_list: Vec<Entry>,
    pub free_list: Vec<usize>,
    pub size: usize
}

impl EntityStorage{

    /*
    allocate a new empty entity
    creates a vec full of nones
    */
    pub fn allocate_new_entity(&mut self) -> EntityIndex {
        self.size += 1;
        if let Some(x) = self.free_list.pop() {
            self.entity_list[x] = Entry{is_live: true, generation: 0};
            (x, 0)
        }else{
            self.entity_list.push(Entry { is_live: true, generation: 0 });
            let entity = (self.entity_list.len() - 1, 0);
            for (_, component) in self.storage.iter_mut(){
                component.push(None);
            }
            entity
        }
    }

    /*
    removes an entity, returning its index to the pool for a new entity to be allocated
    removes all of its components
    */
    pub fn deallocate_entity(&mut self, id: EntityIndex) -> Result<(), &str> {
        self.size -= 1;
        if id.1 == self.entity_list[id.0].generation {
            self.entity_list[id.0].is_live = false;
            self.free_list.push(id.0);
            for (_, component) in self.storage.iter_mut(){
                component[id.0] = None;
            }
            Ok(())
        }else{
            Err("incorrect generation")
        }
    }

    /*
    adds a component to an entity
    if the component has not been registered to the manager, it will panic
    */
    pub fn add_component<T: 'static>(&mut self, index: EntityIndex, component: T) -> Result<EntityIndex, &str>{
        if index.1 == self.entity_list[index.0].generation {
            if let Some(comp) = self.storage.get_mut(&TypeId::of::<T>()) {
                if let Some(None) = comp.get_mut(index.0) {
                    comp[index.0] = Some(Box::new(component));
                    self.entity_list[index.0].generation += 1;
                    Ok((index.0, self.entity_list[index.0].generation))
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
        if index.1 != self.entity_list[index.0].generation {
            Err("incorrect generation")
        }else{
            if let Some(x) = self.storage.get_mut(&TypeId::of::<T>()){
                x[index.0] = None;
                self.entity_list[index.0].generation += 1;
            }
            Ok((index.0, self.entity_list[index.0].generation))
        }
    }

    /*
    gives a mutable reference to an entities component for updating
    */
    pub fn fetch<T: 'static>(&mut self, id: EntityIndex) -> Result<Option<&mut T>, &str> {
        if id.1 != self.entity_list[id.0].generation{
            Err("incorrect generation")
        }else{
            let component = self.storage.get_mut(&TypeId::of::<T>()).unwrap();
            let unwrapped_component = component[id.0].as_mut().unwrap();
            let downcast: Option<&mut T> = unwrapped_component.downcast_mut::<T>();
            Ok(downcast)
        }
    }

    /*
    returns a new empty entity storage
    */
    pub fn new() -> EntityStorage {
        EntityStorage{storage: HashMap::new(), free_list: Vec::new(), entity_list: Vec::new(), size: 0}
    }
}