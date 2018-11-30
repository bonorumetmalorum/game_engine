use entity::*;
use entity::management::*;
use component::*;
use std::any::*;
use std::collections::HashMap;

struct StubComponentA {
    pub counter: u8
}

impl Component for StubComponentA {
    fn update(&mut self) {
        self.counter += 1;
    }
}

struct StubComponentB {
    pub counter: u8
}

impl Component for StubComponentB{

    fn update(&mut self) {
        self.counter += 1;
    }
}

 #[test]
 fn initialise_ecs(){
     let entity_manager:EntityStorage<StubComponentA> = EntityStorage::new();
     assert_eq!(entity_manager.generation, 0);
     assert_eq!(entity_manager.storage.len(), 0);
 }

 #[test]
 fn create_new_empty_entity(){
     let mut entity_manager: EntityStorage<StubComponentA> = EntityStorage::new();
     let (entity, generation) = entity_manager.register_new_entity();
     assert_eq!(generation, 1);
     assert_eq!(entity, 0);
 }

 #[test]
 fn add_component_to_entity(){
     let mut entity_manager: EntityStorage<StubComponentA> = EntityStorage::new();
     let index = entity_manager.register_new_entity();
     let (index, gen) = entity_manager.add_component(index, StubComponentA {counter:0}).unwrap();
     assert_eq!(gen, 2);
     assert_eq!(index, 0);
 }

 #[test]
 fn remove_component_from_entity(){
     let mut entity_manager: EntityStorage<StubComponentA> = EntityStorage::new();
     let index = entity_manager.register_new_entity();
     let index = entity_manager.add_component(index, StubComponentA {counter:0}).unwrap();
     let (ind, gen) = entity_manager.remove_component(index).unwrap();
     assert_eq!(gen, 3);
     assert_eq!(ind, 0);
 }

 #[test]
 fn remove_entity_from_ecs(){
     let mut entity_manager: EntityStorage<StubComponentA> = EntityStorage::new();
     let index = entity_manager.register_new_entity();
     let index = entity_manager.add_component(index, StubComponentA {counter:0}).unwrap();
     let result = entity_manager.deregister_entity(index);
     assert!(result.is_ok(), true);
 }

#[test]
fn register_new_component(){
    //issue with dynamic typing.
    //look into any type
    //any type is the solution here, but need to figure out how to implement it into the EntityManager
    let comp = Box::new(StubComponentA{counter: 0});
    let mut map: HashMap<TypeId, Box<Any>> = HashMap::new();
    map.insert(TypeId::of::<StubComponentA>(), comp);
    let typeid = TypeId::of::<StubComponentA>();
    let comp = &map[&typeid];
    let downcast = comp.downcast_ref::<StubComponentA>().unwrap();
    assert_eq!(downcast.counter, 0);
}

