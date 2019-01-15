use super::*;
use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;
use core::borrow::BorrowMut;
use component::ComponentStorage;

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