use entity::*;
use entity::management::*;
use component::*;

struct StubComponent{
    pub counter: u8
}

impl Component for StubComponent{
    fn update(&mut self) {
        self.counter += 1;
    }
}

 #[test]
 fn initialise_ecs(){
     let entity_manager:EntityStorage<StubComponent> = EntityStorage::new();
     assert_eq!(entity_manager.generation, 0);
     assert_eq!(entity_manager.storage.len(), 0);
 }

 #[test]
 fn create_new_empty_entity(){
     let mut entity_manager: EntityStorage<StubComponent> = EntityStorage::new();
     let (entity, generation) = entity_manager.register_new_entity();
     assert_eq!(generation, 1);
     assert_eq!(entity, 0);
 }

 #[test]
 fn add_component_to_entity(){
     let mut entity_manager: EntityStorage<StubComponent> = EntityStorage::new();
     let index = entity_manager.register_new_entity();
     let (index, gen) = entity_manager.add_component(index, StubComponent{counter:0}).unwrap();
     assert_eq!(gen, 2);
     assert_eq!(index, 0);
 }

 #[test]
 fn remove_component_from_entity(){
     let mut entity_manager: EntityStorage<StubComponent> = EntityStorage::new();
     let index = entity_manager.register_new_entity();
     let index = entity_manager.add_component(index, StubComponent{counter:0}).unwrap();
     let (ind, gen) = entity_manager.remove_component(index).unwrap();
     assert_eq!(gen, 3);
     assert_eq!(ind, 0);
 }

 #[test]
 fn remove_entity_from_ecs(){
     let mut entity_manager: EntityStorage<StubComponent> = EntityStorage::new();
     let index = entity_manager.register_new_entity();
     let index = entity_manager.add_component(index, StubComponent{counter:0}).unwrap();
     let result = entity_manager.deregister_entity(index);
     assert!(result.is_ok(), true);
 }