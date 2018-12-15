use super::*;
use component::*;

pub struct EntityStorage<T: Component> {
    pub storage: Vec<Option<T>>,
    pub generation: u64
}

impl<T: Component> EntityStorage<T>{
    pub fn register_new_entity(&mut self) -> EntityIndex {
        self.storage.push(None);
        self.generation += 1;
        (self.storage.len() - 1, self.generation)
    }

    pub fn deregister_entity(&mut self, id: EntityIndex) -> Result<T, &str> {
        if id.1 != self.generation {
            Err("incorrect generation")
        } else{
            self.generation += 1;
            Ok(self.storage[id.0].take().unwrap())
        }
    }

    pub fn add_component(&mut self, index: EntityIndex, component: T) -> Result<EntityIndex, &str>{
        if index.1 != self.generation{
            Err("incorrect generation")
        }else{
            self.generation += 1;
            self.storage[index.0] = Some(component);
            Ok((index.0, self.generation))
        }
    }

    pub fn remove_component(&mut self, index: EntityIndex) -> Result<EntityIndex, &str>{
        if index.1 != self.generation {
            Err("incorrect generation")
        }else{
            self.generation += 1;
            self.storage[index.0] = None;
            Ok((index.0, self.generation))
        }
    }

    pub fn fetch(&mut self, id: EntityIndex) -> Result<&Option<T>, &str> {
        if id.1 != self.generation{
            Err("incorrect generation")
        }else{
            Ok(&self.storage[id.0])
        }
    }

    pub fn new_with_components(storage: Vec<Option<T>>) -> EntityStorage<T>{
         EntityStorage{storage, generation: 0}
    }

    pub fn new() -> EntityStorage<T> {
        EntityStorage{storage: Vec::new(), generation: 0}
    }
}

/*
notes:
make all data structure generational
*/