use std::sync::RwLockWriteGuard;
use component::storage::Storage;
use entity::EntityIndex;
use component::ComponentEntry;
use std::sync::RwLockReadGuard;
use std::ops::Deref;
use std::ops::DerefMut;


pub struct ComponentWriteHandle<'l, T>{
    pub w: RwLockWriteGuard<'l, T>
}

impl<'a, 'b, S: Storage<'b>> ComponentWriteHandle<'a, S>{
    pub fn get(&'b self, id: EntityIndex) -> &ComponentEntry<S::Component> {
        self.w.deref().get(id)
    }

    pub fn get_mut_iter(&'b mut self) -> S::ComponentIteratorMut {
        self.w.deref_mut().get_mut_iter()
    }
}

pub struct ComponentReadHandle<'l, T> {
    pub r: RwLockReadGuard<'l, T>
}

impl<'a, 'b, S:Storage<'b>> ComponentReadHandle<'a, S>{
    pub fn get(&'b self, id: EntityIndex) -> &ComponentEntry<S::Component> {
        self.r.deref().get(id)
    }

    pub fn get_iterator(&'b self) -> S::ComponentIterator {
        self.r.deref().get_iter()
    }
}