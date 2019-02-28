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
use component::handles::ComponentWriteHandle;
use component::handles::ComponentReadHandle;

pub mod dense_component_storage;
pub mod storage;
pub mod iter;
pub mod handles;

pub trait Component: 'static + Sized + Send + Sync + Clone{
    type ComponentStorage: for<'st> Storage<'st, Component = Self>;
    fn update(&mut self);
}

#[derive(Clone)]
pub enum ComponentEntry<T: Sized + Send + Sync + Clone>{
    Empty,
    Entry(Box<T>)
}

impl<T: Sized + Send + Sync + Clone> ComponentEntry<T> {
    pub fn borrow_mut(&mut self) -> Option<&mut Box<T>> {
        match self {
            ComponentEntry::Entry(val) => Some(val),
            _ => None
        }
    }
}

pub trait GenericComponentStorage: Send + Sync + Downcast{
    fn remove(&mut self, index: EntityIndex) -> Result<EntityIndex, &str>;
}
impl_downcast!(GenericComponentStorage);

pub struct ComponentStore<T>(pub RwLock<T>);

//switch RwLockWriteGuard to ComponentWrite/Read Handle.
impl<'st, T: Storage<'st>> ComponentStore<T> {
    pub fn write_handle(&self) -> ComponentWriteHandle<T>{
        let result = self.0.write().unwrap();
        ComponentWriteHandle{ w: result }
    }

    pub fn read_handle(&self) -> ComponentReadHandle<T>{
        let result = self.0.read().unwrap();
        ComponentReadHandle{ r: result }
    }

    pub fn get_mut_handle(&mut self) -> &mut T {
        self.0.get_mut().unwrap()
    }
}

impl<'cs, T: 'static + Storage<'cs>> GenericComponentStorage for ComponentStore<T> {
    fn remove(&mut self, index: (usize, u64)) -> Result<(usize, u64), &str> {
        self.0.get_mut().expect("poisoned lock").remove(index)
    }
}

pub struct ComponentStorage(
    HashMap<TypeId, Box<GenericComponentStorage>>
);
//I think here i need to store a Box any and store vectors in the any
//this will allow to downcast to a Vec<T> and subsequently get the appropriate iterator.

impl<'st> ComponentStorage {

    pub fn new() -> ComponentStorage {
        ComponentStorage(HashMap::new())
    }

    pub fn register_component<T: Component>(&mut self) -> Result<(usize), &str>{
        let compstrg: DenseComponentStorage<T> = DenseComponentStorage::new();
        let len = compstrg.0.len();
        let componentstore = ComponentStore(RwLock::new(compstrg));
        if let None = self.0.insert(TypeId::of::<T>(), Box::new(componentstore)) {
            Ok(len)
        }else{
            Err("overwritten existing component storage")
        }
    }

    pub fn add_component<T: Component>(&mut self, component: T, id: EntityIndex) -> Result<EntityIndex, &str> {
        if let Ok(storage) = self.get_mut::<T>(){
            let mut store = storage.0.get_mut().unwrap();
            store.insert(id, component).expect("unable to insert component");
            Ok(id)
        }else{
            Err("component is not registered")
        }
    }

    pub fn remove_component<T: Component>(&mut self, id: EntityIndex) -> Result<EntityIndex, &str>{
        if let Ok(storage) = self.get_mut::<T>(){
            let mut store = storage.0.get_mut().unwrap();
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

    pub fn len(&self) -> usize {
        self.0.len()
    }

//    pub fn get_mut_iterator<T: Component>(&mut self) -> Result<T, &str>{
//        if let Ok(entry) = self.get_mut::<T>(){
//            let mut storage = entry.write_handle();
//            let it = storage.get_mut_iter();
//            Ok(it)
//        }else{
//            Err("Unregistered component")
//        }
//    }

}

#[derive(Clone)]
pub struct StubPosition{
    pub x: f32,
    pub y: f32,
}

impl Component for StubPosition{
    type ComponentStorage = DenseComponentStorage<Self>;

    fn update(&mut self) {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct StubVelocity{
    pub dx: f32,
    pub dy: f32,
}

impl Component for StubVelocity{
    type ComponentStorage = DenseComponentStorage<Self>;

    fn update(&mut self) {
        unimplemented!()
    }
}