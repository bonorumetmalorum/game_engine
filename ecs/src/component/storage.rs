use component::iter::Iter;
use entity::EntityIndex;
use component::ComponentEntry;

pub trait Storage<'st>: 'static + Clone + Default {
    type Component: 'static + Sized + Clone;
    type ComponentIteratorMut: Iter<Item = &'st mut Box<Self::Component>>;
    type ComponentIterator: Iter<Item = &'st Box<Self::Component>>;
    ///returns the appropriate component storage
    fn get(&self, id: EntityIndex) -> &ComponentEntry<Self::Component>;
    ///removes the component at `EntityIndex`
    fn remove(&mut self, EntityIndex) -> Result<EntityIndex, &str>;
    ///returns a mutable reference to its mutable `Iter`
    fn get_mut_iter(&'st mut self) -> Self::ComponentIteratorMut;
    ///returns a mutable reference to its immutable `Iter`
    fn get_iter(&'st self) -> Self::ComponentIterator;
    ///adds a component to the given `EntityIndex`
    fn insert(&mut self, index: EntityIndex, component: Self::Component) -> Result<EntityIndex, &str>;
    ///returns the length of the `Storage`
    fn len(&self) -> usize;
}

