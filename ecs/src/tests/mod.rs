use component::Component;
use ECS;
use component::ComponentIterator;
use component::Iter;
use component::ComponentEntry::*;
use component::Storage;
use component::DenseComponentStorage;
use std::clone::Clone;

#[derive(Clone)]
struct StubComponentA {
    pub counter: u8
}

impl Component for StubComponentA {
    type ComponentStorage = DenseComponentStorage<Self>;

    fn update(&mut self) {
        self.counter += 1;
    }
}
#[derive(Clone)]
struct StubComponentB {
    pub counter: u8
}

impl Component for StubComponentB{
    type ComponentStorage = DenseComponentStorage<Self>;

    fn update(&mut self) {
        self.counter += 1;
    }
}

#[test]
fn initialise_ecs(){
    let entity_manager:ECS = ECS::new();
    assert_eq!(entity_manager.entity_list.entity_list.len(), 0);
    assert_eq!(entity_manager.entity_list.free_list.len(), 0);
    assert_eq!(entity_manager.storage.len() , 0);
}

#[test]
fn create_new_empty_entity(){
    let mut entity_manager: ECS = ECS::new();
    let (entity, generation) = entity_manager.allocate_new_entity();
    assert_eq!(generation, 0);
    assert_eq!(entity, 0);
    assert_eq!(entity_manager.size, 1);
}

#[test]
fn add_component_to_entity(){
    let mut entity_manager: ECS = ECS::new();
    let index = entity_manager.allocate_new_entity();
    let _ok = entity_manager.register_new_component::<StubComponentA>().is_ok();
    let (index1, gen) = entity_manager.add_component(index, StubComponentA {counter:0}).unwrap();
    assert_eq!(gen, 0);
    assert_eq!(index1, 0);
}

#[test]
fn remove_component_from_entity(){
    let mut entity_manager: ECS = ECS::new();
    let index = entity_manager.allocate_new_entity();
    let _ok = entity_manager.register_new_component::<StubComponentA>().is_ok();
    let index = entity_manager.add_component(index, StubComponentA {counter:0}).unwrap();
    let (ind, gen) = entity_manager.remove_component::<StubComponentA>(index).unwrap();
    assert_eq!(gen, 0);
    assert_eq!(ind, 0);
}

#[test]
fn remove_entity_from_ecs(){
    let mut entity_manager: ECS = ECS::new();
    let index = entity_manager.allocate_new_entity();
    let _ok = entity_manager.register_new_component::<StubComponentA>().is_ok();
    let index = entity_manager.add_component(index, StubComponentA {counter:0}).unwrap();
    let result = entity_manager.deallocate_entity(index);
    assert!(result.is_ok(), true);
}

#[test]
fn register_new_component(){
    //
    let mut entity_manager = ECS::new();
    entity_manager.allocate_new_entity();
    let res2 = entity_manager.register_new_component::<StubComponentA>().unwrap();
    assert_eq!(res2, 0)
}

#[test]
fn get_component_iterator(){
    let mut entity_manager = ECS::new();
    entity_manager.allocate_new_entity();
    entity_manager.allocate_new_entity();
    entity_manager.allocate_new_entity();
    entity_manager.register_new_component::<StubComponentA>();
    entity_manager.add_component((0,0), StubComponentA{ counter: 0 });
    entity_manager.add_component((1,0), StubComponentA{ counter: 1 });
    entity_manager.add_component((2,0), StubComponentA{ counter: 2 });
    let mut handle = entity_manager.get_component_write_handle::<StubComponentA>();
    let mut it = handle.get_mut_iter();
    assert_eq!(it.index(), 0)
}

#[test]
fn get_component_test(){
    let mut entity_manager = ECS::new();
    entity_manager.allocate_new_entity();
    entity_manager.allocate_new_entity();
    entity_manager.allocate_new_entity();
    entity_manager.register_new_component::<StubComponentA>();
    entity_manager.add_component((0,0), StubComponentA{ counter: 0 });
    entity_manager.add_component((1,0), StubComponentA{ counter: 1 });
    entity_manager.add_component((2,0), StubComponentA{ counter: 2 });
    {
        let mut comp = entity_manager.get_mut::<StubComponentA>();
        let mut it = comp.get_mut_iter();
        loop{
            if let Some(x) = it.next(None) {
                x.0.counter += 1;
            }else{
                break;
            }
        }
    }
    let mut handle = entity_manager.get_component_write_handle::<StubComponentA>();
    let mut it = handle.get_mut_iter();
    let mut res = it.into_vec();
    assert_eq!(res[0].counter, 1);
    assert_eq!(res[1].counter, 2);
    assert_eq!(res[2].counter, 3);
}

#[test]
fn iterator_test_singular(){
    let mut entity_manager = ECS::new();
    entity_manager.allocate_new_entity();
    entity_manager.allocate_new_entity();
    entity_manager.allocate_new_entity();
    entity_manager.register_new_component::<StubComponentA>();
    entity_manager.register_new_component::<StubComponentB>();
    entity_manager.add_component((0,0), StubComponentA{ counter: 0 });
    entity_manager.add_component((2,0), StubComponentA{counter: 0});
    entity_manager.add_component((1,0), StubComponentA{counter: 0});
    entity_manager.add_component((1,0), StubComponentB{counter: 100});
    let mut handle = entity_manager.get_component_write_handle::<StubComponentB>();
    let mut itb = handle.get_mut_iter();
    let mut result = itb.next(None);
    let mut result1 = itb.next(None);
    assert_eq!(result.is_none(), true);
    assert_eq!(result1.unwrap().0.counter, 100);
}

#[test]
fn iterator_join_test(){
    let mut entity_manager = ECS::new();
    entity_manager.allocate_new_entity();
    entity_manager.allocate_new_entity();
    entity_manager.allocate_new_entity();
    entity_manager.register_new_component::<StubComponentA>();
    entity_manager.register_new_component::<StubComponentB>();
    entity_manager.add_component((0,0), StubComponentA{ counter: 0 });
    entity_manager.add_component((1,0), StubComponentA{counter: 0});
    entity_manager.add_component((1,0), StubComponentB{counter: 100});
    entity_manager.add_component((2,0), StubComponentA{counter: 0});
    let mut compha = entity_manager.get_component_write_handle::<StubComponentA>();
    let mut comphb = entity_manager.get_component_write_handle::<StubComponentB>();
    let ita = compha.get_mut_iter();
    let itb = comphb.get_mut_iter();
    let mut joint = ita.join(itb);
    let mut res = joint.next(None);
    let mut unwrapped = res.unwrap();
    assert_eq!((unwrapped.0).0.counter, 0);
    assert_eq!((unwrapped.0).1.counter, 100);
}

