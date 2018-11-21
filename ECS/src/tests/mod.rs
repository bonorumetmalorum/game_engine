use entity::*;
use entity::management::*;
use component::*;

struct StubComponent;

impl Component for StubComponent{
    fn update(&mut self) {
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
     assert_eq!(true, false);
 }

 #[test]
 fn add_component_to_entity(){
     assert_eq!(true, false);
 }

 #[test]
 fn remove_component_from_entity(){
     assert_eq!(true, false);
 }

 #[test]
 fn remove_entity_from_ecs(){
     assert_eq!(true, false);
 }

 #[test]
 fn fetch_range_of_entities(){
     assert_eq!(true, false);
 }