use super::Entity;

trait StorageType{

}

struct EntityStorage<T> where T: StorageType {

}

impl EntityStorage<T>{
    fn register_new_entity(&mut self){unimplemented!()}

    fn deregister_entity(&mut self, id: Entity){unimplemented!()}

    fn fetch(&self, id: Entity){unimplemented!()}
}