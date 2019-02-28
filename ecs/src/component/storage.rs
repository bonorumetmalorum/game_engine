use component::iter::Iter;
use entity::EntityIndex;
use component::ComponentEntry;

pub trait Storage<'st>: 'static + Send + Sync + Clone + Default {
    type Component: 'static + Send + Sync + Sized + Clone;
    type ComponentIteratorMut: Iter<Item = &'st mut Box<Self::Component>>;
    type ComponentIterator: Iter<Item = &'st Box<Self::Component>>;
    fn get(&self, id: EntityIndex) -> &ComponentEntry<Self::Component>;
    fn remove(&mut self, EntityIndex) -> Result<EntityIndex, &str>;
    fn get_mut_iter(&'st mut self) -> Self::ComponentIteratorMut;
    fn get_iter(&'st self) -> Self::ComponentIterator;

    fn insert(&mut self, index: EntityIndex, component: Self::Component) -> Result<EntityIndex, &str>;
    fn len(&self) -> usize;
}

