use component::ComponentEntry;
use component::Component;
use component::storage::Storage;
use entity::EntityIndex;
use core::slice;
use component::iter::Iter;

#[derive(Clone)]
pub struct DenseComponentStorage<T: Clone>(pub Vec<ComponentEntry<T>>);

impl<T: Component> Default for DenseComponentStorage<T>{
    fn default() -> Self {
        DenseComponentStorage(Vec::new())
    }
}

impl<'it, T: Component> Storage<'it> for DenseComponentStorage<T> {
    type Component = T;
    type ComponentIteratorMut = DenseComponentIteratorMut<'it, T>;
    type ComponentIterator = DenseComponentIterator<'it, T>;

    //potentially make this return a result type
    fn get(&self, id: (usize, u64)) -> &ComponentEntry<Self::Component> {
        if let Some(x) = self.0.get(id.0) {
            x
        }else{
            &ComponentEntry::Empty
        }
    }

    fn remove(&mut self, index: EntityIndex) -> Result<(usize, u64), &str> {
        if let Some(reference) = self.0.get_mut(index.0){
            *reference = ComponentEntry::Empty;
            Ok(index)
        }else{
            Err("index out of bounds")
        }
    }

    fn get_mut_iter(&'it mut self) -> Self::ComponentIteratorMut {
        DenseComponentIteratorMut{current_index: 0, st: self.0.iter_mut()}
    }

    fn get_iter(&'it self) -> Self::ComponentIterator {
        DenseComponentIterator{ st: self.0.iter(), current_index: 0 }
    }

    fn insert(&mut self, index: (usize, u64), component: Self::Component) -> Result<EntityIndex, &str>{
        if index.0 > self.len(){
            while index.0 > self.len() {
                self.0.push(ComponentEntry::Empty);
            }
            self.0.push(ComponentEntry::Entry(Box::new(component)));
        }else{
            self.0.insert(index.0, ComponentEntry::Entry(Box::new(component)));
        }
        Ok(index)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'it, T: Clone> DenseComponentStorage<T> {
    pub fn new() -> DenseComponentStorage<T>{
        DenseComponentStorage(Vec::new())
    }
}

pub struct DenseComponentIteratorMut<'cs, T: 'cs + Clone>{
    st: slice::IterMut<'cs, ComponentEntry<T>>,
    current_index: usize
}

impl<'it, T: Component> Iter for DenseComponentIteratorMut<'it, T>{
    type Item = &'it mut Box<T>;

    fn next_element(&mut self, until: Option<usize>) -> Option<(Self::Item, usize)> {
        let lim = until.unwrap_or(0);
        loop{
            let r;
            let i;
            if lim > self.current_index {
                r = self.st.nth(lim - self.current_index);
                i = lim;
                self.current_index = lim + 1;
            }else{
                r = self.st.next();
                i = self.current_index;
                self.current_index += 1;
            }

            match r {
                Some(ComponentEntry::Entry(ref mut v)) => {return Some((v, i))},
                Some(_) => continue,
                None => {return None}
            }

        }
    }
}

impl<'it, T: 'static + Clone> DenseComponentIteratorMut<'it, T> {

    pub fn new(it: slice::Iter<'it, ComponentEntry<T>>) -> DenseComponentIterator<'it, T> {
        DenseComponentIterator{
            st: it,
            current_index: 0
        }
    }

    pub fn index(&mut self) -> usize {
        self.current_index
    }
}

pub struct DenseComponentIterator<'cs, T: 'cs + Clone>{
    st: slice::Iter<'cs, ComponentEntry<T>>,
    current_index: usize
}

impl<'cs, T: Component> Iter for DenseComponentIterator<'cs, T> {
    type Item = &'cs Box<T>;

    fn next_element(&mut self, until: Option<usize>) -> Option<(Self::Item, usize)> {
        let lim = until.unwrap_or(0);
        loop{
            let r;
            let i;
            if lim > self.current_index {
                r = self.st.nth(lim - self.current_index);
                i = lim;
                self.current_index = lim + 1;
            }else{
                r = self.st.next();
                i = self.current_index;
                self.current_index += 1;
            }

            match r {
                Some(ComponentEntry::Entry(ref v)) => {return Some((v, i))},
                Some(_) => continue,
                None => {return None}
            }
        }
    }
}