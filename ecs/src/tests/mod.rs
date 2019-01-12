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
    let entity_manager:EntityStorage = EntityStorage::new();
    assert_eq!(entity_manager.entity_list.entity_list.len(), 0);
    assert_eq!(entity_manager.entity_list.free_list.len(), 0);
    assert_eq!(entity_manager.storage.len(), 0);
}

#[test]
fn create_new_empty_entity(){
    let mut entity_manager: EntityStorage = EntityStorage::new();
    let (entity, generation) = entity_manager.allocate_new_entity();
    assert_eq!(generation, 0);
    assert_eq!(entity, 0);
    assert_eq!(entity_manager.size, 1);
}

#[test]
fn add_component_to_entity(){
    let mut entity_manager: EntityStorage = EntityStorage::new();
    let index = entity_manager.allocate_new_entity();
    entity_manager.register_new_component::<StubComponentA>();
    let (index1, gen) = entity_manager.add_component(index, StubComponentA {counter:0}).unwrap();
    assert_eq!(gen, 0);
    assert_eq!(index1, 0);
}

#[test]
fn remove_component_from_entity(){
    let mut entity_manager: EntityStorage = EntityStorage::new();
    let index = entity_manager.allocate_new_entity();
    entity_manager.register_new_component::<StubComponentA>();
    let index = entity_manager.add_component(index, StubComponentA {counter:0}).unwrap();
    let (ind, gen) = entity_manager.remove_component::<StubComponentA>(index).unwrap();
    assert_eq!(gen, 0);
    assert_eq!(ind, 0);
}

#[test]
fn remove_entity_from_ecs(){
    let mut entity_manager: EntityStorage = EntityStorage::new();
    let index = entity_manager.allocate_new_entity();
    entity_manager.register_new_component::<StubComponentA>();
    let index = entity_manager.add_component(index, StubComponentA {counter:0}).unwrap();
    let result = entity_manager.deallocate_entity(index);
    assert!(result.is_ok(), true);
}

#[test]
fn register_new_component(){
    //
    let mut entity_manager = EntityStorage::new();
    let res1 = entity_manager.allocate_new_entity();
    let res2 = entity_manager.register_new_component::<StubComponentA>().unwrap();
    assert_eq!(res2, 1)
}

#[test]
fn get_component_test(){
    //
    let mut entity_manager = EntityStorage::new();
    let res1 = entity_manager.allocate_new_entity();
    entity_manager.register_new_component::<StubComponentA>();
    let res3 = entity_manager.add_component(res1, StubComponentA{counter: 0}).unwrap();
    let mut comp = entity_manager.fetch::<StubComponentA>(res3).unwrap();
    assert_eq!(comp.unwrap().counter, 0);
}

