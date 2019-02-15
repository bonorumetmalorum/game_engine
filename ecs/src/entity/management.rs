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

    pub fn get_iter_live(&self) -> EntityIteratorLive{
        EntityIteratorLive{
            st: self.entity_list.iter(),
            current_index: 0
        }
    }

    pub fn get_iter(&self) -> EntityIterator{
        EntityIterator{
            st: self.entity_list.iter(),
            current_index: 0
        }
    }

    pub fn set_live(&mut self, id: EntityIndex) -> bool {
        unimplemented!()
    }

    pub fn set_dead(&mut self, id: EntityIndex) -> bool {
        unimplemented!()
    }
}

pub struct EntityIteratorLive<'cs>{
    st: slice::Iter<'cs, Entry>,
    current_index: usize
}

impl<'cs> Iter for EntityIteratorLive<'cs> {
    type Item = &'cs Entry;
    //gets the next live entry
    fn next_element(&mut self, until: Option<usize>) -> Option<(Self::Item, usize)> {
        let next = until.unwrap_or(0);
        loop{
            let r;
            let i;
            if next > self.current_index {
                r = self.st.nth(next - self.current_index);
                i = next;
                self.current_index = next + 1;
            }else{
                r = self.st.next();
                i = self.current_index;
                self.current_index += 1;
            }

            match r {
                Some(entry) => if entry.is_live {return Some((entry, self.current_index))} else {continue}
                None => return None
            }
        }
    }
}

pub struct EntityIterator<'cs>{
    st: slice::Iter<'cs, Entry>,
    current_index: usize
}

impl<'cs> Iter for EntityIterator<'cs> {
    type Item = &'cs Entry;
    //gets the next live entry
    fn next_element(&mut self, until: Option<usize>) -> Option<(Self::Item, usize)> {
        let next = until.unwrap_or(0);
        loop{
            let r;
            let i;
            if next > self.current_index {
                r = self.st.nth(next - self.current_index);
                i = next;
                self.current_index = next + 1;
            }else{
                r = self.st.next();
                i = self.current_index;
                self.current_index += 1;
            }

            match r {
                Some(entry) => {return Some((entry, self.current_index))}
                None => return None
            }
        }
    }
}

