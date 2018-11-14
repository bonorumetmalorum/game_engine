use super::Entity;

trait StorageType {
    //methods to store things
    fn put();
    fn fetch();
}

struct EntityStorage<T> where T: StorageType {

}

impl EntityStorage<T>{
    fn register_new_entity(&mut self){unimplemented!()}

    fn deregister_entity(&mut self, id: Entity){unimplemented!()}

    fn fetch(&self, id: Entity){unimplemented!()}
}

//think about adding a grouping of common entities: Group(*Entity)
struct GenerationalVec{

}