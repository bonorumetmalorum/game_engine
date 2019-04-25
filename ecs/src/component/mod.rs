use std::sync::RwLockWriteGuard;
use component::storage::Storage;
use entity::EntityIndex;
use std::sync::RwLockReadGuard;
use downcast_rs::Downcast;
use std::sync::RwLock;
use std::any::TypeId;
use component::dense_component_storage::DenseComponentStorage;
use std::collections::HashMap;
use core::borrow::BorrowMut;
use std::cell::RefCell;
use std::cell::Ref;
use std::cell::RefMut;
use fnv::FnvHashMap;

pub mod dense_component_storage;
pub mod storage;
pub mod iter;
//pub mod handles;
/// must be implemented for all components to be stored in the `ECS`.
/// Associated type defines the method of storage in the `ECS`
pub trait Component: 'static + Sized + Clone{
    type ComponentStorage: for<'st> Storage<'st, Component = Self>;
}
///Entry in the component storage
#[derive(Clone)]
pub enum ComponentEntry<T: Sized + Clone>{
    Empty,
    Entry(Box<T>)
}

impl<T: Sized + Clone> ComponentEntry<T> {
    ///Borrow the internally held component mutably
    pub fn borrow_mut(&mut self) -> Option<&mut Box<T>> {
        match self {
            ComponentEntry::Entry(val) => Some(val),
            _ => None
        }
    }
}
///A generic storage type which implements `downcast`, can be casted to the appropriate storage type
pub trait GenericComponentStorage: Downcast{
    ///remove the component at the given `EntityIndex`
    fn remove(&mut self, index: EntityIndex) -> Result<EntityIndex, &str>;
}
impl_downcast!(GenericComponentStorage);

pub struct ComponentStore<T>(pub RefCell<T>);

//switch RwLockWriteGuard to ComponentWrite/Read Handle.
impl<'st, T: Storage<'st>> ComponentStore<T> {

    pub fn write(&self) -> RefMut<T>{
        self.0.borrow_mut()
    }

    pub fn read(&self) -> Ref<T>{
        self.0.borrow()
    }

    pub fn get_mut_handle(&mut self) -> &mut T {
        self.0.get_mut()
    }
}

impl<'cs, T: 'static + Storage<'cs>> GenericComponentStorage for ComponentStore<T> {
    fn remove(&mut self, index: (usize, u64)) -> Result<(usize, u64), &str> {
        self.0.get_mut().remove(index)
    }
}
///component storage
pub struct ComponentStorage(
    FnvHashMap<TypeId, Box<GenericComponentStorage>>
);

impl<'st> ComponentStorage {
    ///creates a new empty `ComponentStorage`
    pub fn new() -> ComponentStorage {
        ComponentStorage(FnvHashMap::default())
    }
    /// Registers a new component with the `ECS`, allowing it to be added to entities
    pub fn register_component<T: Component>(&mut self) -> Result<(usize), &str>{
        let compstrg: DenseComponentStorage<T> = DenseComponentStorage::new();
        let len = compstrg.0.len();
        let componentstore = ComponentStore(RefCell::new(compstrg));
        if let None = self.0.insert(TypeId::of::<T>(), Box::new(componentstore)) {
            Ok(len)
        }else{
            Err("overwritten existing component storage")
        }
    }
    ///adds the component to the given `EntityIndex` provided that the entity exists and the component is registered with the `ECS`
    pub fn add_component<T: Component>(&mut self, component: T, id: EntityIndex) -> Result<EntityIndex, &str> {
        if let Ok(storage) = self.get_mut::<T>(){
            let mut store = storage.0.get_mut();
            store.insert(id, component).expect("unable to insert component");
            Ok(id)
        }else{
            Err("component is not registered")
        }
    }
    ///removes a component from the given `EntityIndex` provided that the entity exists and the component is registered with the `ECS`
    pub fn remove_component<T: Component>(&mut self, id: EntityIndex) -> Result<EntityIndex, &str>{
        if let Ok(storage) = self.get_mut::<T>(){
            let mut store = storage.0.get_mut();
            if id.0 >= store.len() {
                Err("entity does not have component")
            }else{
                store.remove(id).expect("unable to remove component");
                Ok(id)
            }
        }else{
            Err("component is not registered")
        }
    }
    ///removes all components from a given `EntityIndex`, returning `Ok` on success and 'Err' otherwise
    pub fn clear_entity(&mut self, id: EntityIndex) -> Result<(), &str> {
        let mut status = Ok(());
        for (_, cs) in self.0.borrow_mut() {
            if let Ok(_) = cs.remove(id) {
                continue;
            }else{
                status = Err("Entity does not exist");
                break;
            }
        }
        status
    }
    /// returns an immutable reference to the component for the given `EntityIndex` should it exist and have one
    pub fn get<T: Component>(&self) -> Result<&ComponentStore<T::ComponentStorage>, &str> {
        if let Some(x) = self.0.get(&TypeId::of::<T>()){
            if let Some(dc) = x.downcast_ref::<ComponentStore<T::ComponentStorage>>() {
                Ok(dc)
            }else{
                Err("downcast failed, type error")
            }
        }else{
            Err("unregistered type")
        }
    }
    /// returns a mutable reference to the component for the given `EntityIndex` should it exist and have one
    pub fn get_mut<T: Component>(&mut self) -> Result<&mut ComponentStore<T::ComponentStorage>, &str> {
        if let Some(x) = self.0.get_mut(&TypeId::of::<T>()){
            if let Some(dc) = x.downcast_mut::<ComponentStore<T::ComponentStorage>>() {
                Ok(dc)
            }else{
                Err("downcast failed, type error")
            }
        }else{
            Err("unregistered type")
        }
    }
    /// returns the maximum number of entities allocated
    pub fn len(&self) -> usize {
        self.0.len()
    }
}