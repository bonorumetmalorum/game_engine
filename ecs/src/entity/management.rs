use super::*;
use core::slice;
use component::iter::Iter;


//entry to define an allocation into a generational data structure
pub struct Entry {
    pub is_live: bool,
    pub generation: u64
}

///allocator that returns unused or new `EntityIndex`es
pub struct EntityAllocator {
    pub entity_list: Vec<Entry>,
    pub free_list: Vec<usize>,
}

impl EntityAllocator {
    ///creates a new `EntityAllocator`
    pub fn new() -> EntityAllocator {
        EntityAllocator{
            entity_list: Vec::new(),
            free_list: Vec::new()
        }
    }
    ///allocates a new Entity and returns its `EntityIndex`
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
    ///Deallocates the given `EntityIndex` returning it to a pool of unused indexes
    pub fn deallocate(&mut self, id: EntityIndex) -> Result<(), &str> {
        if id.1 == self.entity_list[id.0].generation {
            if self.entity_list[id.0].is_live{
                self.entity_list[id.0].is_live = false;
                self.free_list.push(id.0);
                Ok(())
            }else{
                Err("already deallocated")
            }
        }else{
            Err("incorrect generation")
        }
    }
    ///returns an immutable iterator over all live `EntityIndexes`
    pub fn get_iter_live(&self) -> EntityIteratorLive{
        EntityIteratorLive{
            st: self.entity_list.iter(),
            current_index: 0
        }
    }
    ///returns an immutable iterator over all live and dead `EntityIndexes`
    pub fn get_iter(&self) -> EntityIterator{
        EntityIterator{
            st: self.entity_list.iter(),
            current_index: 0
        }
    }
}
///Entity Iterator over live indexes
pub struct EntityIteratorLive<'cs>{
    st: slice::Iter<'cs, Entry>,
    current_index: usize
}

impl<'cs> Iter for EntityIteratorLive<'cs> {
    type Item = &'cs Entry;
    ///gets the next live entry
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
                Some(entry) => if entry.is_live {return Some((entry, i))} else {continue}
                None => return None
            }
        }
    }
}
///Entity iterator over all entities, dead or alive
pub struct EntityIterator<'cs>{
    st: slice::Iter<'cs, Entry>,
    current_index: usize
}

impl<'cs> Iter for EntityIterator<'cs> {
    type Item = &'cs Entry;
    ///gets the next entry
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
                Some(entry) => {return Some((entry, i))}
                None => return None
            }
        }
    }
}

