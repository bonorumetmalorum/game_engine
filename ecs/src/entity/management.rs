use super::*;
use core::slice;
use component::Iter;

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

pub struct EntityIterator<'cs>{
    st: slice::IterMut<'cs, Entry>,
    current_index: usize
}

impl<'cs> Iter for EntityIterator<'cs> {
    type Item = &'cs mut Entry;

    fn next(&mut self, until: Option<usize>) -> Option<(Self::Item, usize)> {
        if let Some(x) = self.st.next(){
            if x.is_live {
                Some((x, self.current_index))
            }else {
                None
            }
        }else{
            None
        }
    }

    fn into_vec(mut self) -> Vec<Self::Item> {
        let mut res = vec![];
        loop{
            match self.next() {
                Some(x) => res.push(x.0),
                None => break,
            }    
        }
        res
    }
}
