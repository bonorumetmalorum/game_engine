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

    pub fn deregister_entity(&mut self, id: Entity) -> T {
        self.generation += 1;
        self.storage[id].take().unwrap()
    }

    pub fn fetch(&self, id: Entity){unimplemented!()}

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