use super::*;
use component::*;

trait StorageType {
    //methods to store things
    fn put();
    fn fetch();
    fn delete();
}

struct EntityStorage<T> where T: StorageType {
    storage: T
}

impl<T: StorageType> EntityStorage<T>{
    fn register_new_entity(&mut self){unimplemented!()}

    fn deregister_entity(&mut self, id: Entity){unimplemented!()}

    fn fetch(&self, id: Entity){unimplemented!()}
}

//allows for storing of anyting implements the component trait
struct GenerationalVec<T> where T: Component{
    current_generation: Generation,
    vec: Vec<T>
}

impl<T: Component> StorageType for GenerationalVec<T>{
    fn put() {
        unimplemented!()
    }

    fn fetch() {
        unimplemented!()
    }

    fn delete() {
        unimplemented!()
    }
}