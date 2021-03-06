use std::cell::RefMut;
use std::cell::Ref;
use std::cell::RefCell;
use downcast_rs::Downcast;
use std::any::TypeId;
use std::sync::RwLock;
use std::collections::HashMap;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

///centralized easily accessible storage for shared resources
pub struct ResourceMap{
    map: HashMap<TypeId, Box<ResourceEntry>>
}
///convenience trait allowing for casting to appropriate type
pub trait ResourceEntry: Downcast {}
impl_downcast!(ResourceEntry);
///Entry type for the resource map
pub struct Resource<T>(RwLock<T>);

impl ResourceMap{
    ///get a mutable reference to the stored resource
    pub fn get_write_resource<T:'static>(&self) -> Result<ResourceWriteHandle<T>, &str>{
        if let Some(x) = self.map.get(&TypeId::of::<T>()){
            if let Some(downcast) = x.downcast_ref::<Resource<T>>(){
                Ok(downcast.get_mut())
            }else{
                Err("unable to downcast")
            }

        }else{
            Err("resource does not exist")
        }
    }
    ///get an immutable reference to the stored resource
    pub fn get_read_resource<T:'static>(&self) -> Result<ResourceReadHandle<T>, &str>{
        if let Some(entry) = self.map.get(&TypeId::of::<T>()) {
            if let Some(t) = entry.downcast_ref::<Resource<T>>() {
                Ok(t.get())
            }else{
                Err("unable to downcast")
            }
        }else{
            Err("resource does not exist")
        }
    }
    ///insert a new resource into the resource map
    pub fn insert_resource<T:'static>(&mut self, resource: T){
        self.map.insert(TypeId::of::<T>(), Box::new(Resource(RwLock::new(resource))));
    }
    ///remove a resource from the resource map
    pub fn remove_resource<T:'static>(&mut self) -> Result<Resource<T>, &str> {
        match self.map.remove(&TypeId::of::<T>()) {
            Some(x) => {
                match x.downcast::<Resource<T>>() {
                    Ok(x) => Ok(*x),
                    Err(s) => Err("error downcasting removed type")
                }
            },
            None => Err("resource does not exist")
        }
    }
}

impl Default for ResourceMap {
    fn default() -> Self {
        ResourceMap{
            map: HashMap::new()
        }
    }
}

impl<T> Resource<T> {
    ///get a mutable reference to the stored resource
    pub fn get_mut(&self) -> ResourceWriteHandle<T> {
        ResourceWriteHandle{r: self.0.write().unwrap()}
    }
    ///get a immutable reference to the stored resource
    pub fn get(&self) -> ResourceReadHandle<T> {
        ResourceReadHandle{r: self.0.read().unwrap()}
    }
}

impl<T:'static> ResourceEntry for Resource<T> {}

pub struct ResourceReadHandle<'l, T> {
    pub r: RwLockReadGuard<'l, T>
}

pub struct ResourceWriteHandle<'l, T> {
    pub r: RwLockWriteGuard<'l, T>
}