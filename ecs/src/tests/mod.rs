use component::Component;
use ECS;
use component::Iter;
use component::Storage;
use component::DenseComponentStorage;

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
    assert_eq!(result.is_ok(), true);
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
    entity_manager.register_new_component::<StubComponentA>().expect("unable to register new component");
    entity_manager.add_component((0,0), StubComponentA{ counter: 0 }).expect("not registered");
    entity_manager.add_component((1,0), StubComponentA{ counter: 1 }).expect("not registered");
    entity_manager.add_component((2,0), StubComponentA{ counter: 2 }).expect("not registered");
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
    entity_manager.register_new_component::<StubComponentA>().expect("unable to register new component");
    entity_manager.add_component((0,0), StubComponentA{ counter: 0 }).expect("not registered");
    entity_manager.add_component((1,0), StubComponentA{ counter: 1 }).expect("not registered");
    entity_manager.add_component((2,0), StubComponentA{ counter: 2 }).expect("not registered");
    {
        let comp = entity_manager.get_mut::<StubComponentA>();
        let mut it = comp.get_mut_iter();
        loop{
            if let Some(x) = it.next_element(None) {
                x.0.counter += 1;
            }else{
                break;
            }
        }
    }
    let mut handle = entity_manager.get_component_write_handle::<StubComponentA>();
    let it = handle.get_mut_iter();
    let iw = it.into_iterator_wrapper();
    let res: Vec<&mut Box<StubComponentA>> = iw.collect();
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
    entity_manager.register_new_component::<StubComponentA>().expect("unable to register new component");
    entity_manager.register_new_component::<StubComponentB>().expect("unable to register new component");
    entity_manager.add_component((0,0), StubComponentA{ counter: 0 }).expect("not registered");
    entity_manager.add_component((2,0), StubComponentA{counter: 0}).expect("not registered");
    entity_manager.add_component((1,0), StubComponentA{counter: 0}).expect("not registered");
    entity_manager.add_component((1,0), StubComponentB{counter: 100}).expect("not registered");
    let mut handle = entity_manager.get_component_write_handle::<StubComponentB>();
    let mut itb = handle.get_mut_iter();
    let result = itb.next_element(None);
    let result1 = itb.next_element(None);
    assert_eq!(result.is_none(), false);
    assert_eq!(result.unwrap().0.counter, 100);
    assert_eq!(result1.is_none(), true);
}

#[test]
fn iterator_join_test(){
    let mut entity_manager = ECS::new();
    entity_manager.allocate_new_entity();
    entity_manager.allocate_new_entity();
    entity_manager.allocate_new_entity();
    entity_manager.register_new_component::<StubComponentA>().expect("unable to register new component");
    entity_manager.register_new_component::<StubComponentB>().expect("unable to register new component");
    entity_manager.add_component((0,0), StubComponentA{ counter: 0 }).expect("not registered");
    entity_manager.add_component((1,0), StubComponentA{counter: 0}).expect("not registered");
    entity_manager.add_component((1,0), StubComponentB{counter: 100}).expect("not registered");
    entity_manager.add_component((2,0), StubComponentA{counter: 0}).expect("not registered");
    let mut compha = entity_manager.get_component_write_handle::<StubComponentA>();
    let mut comphb = entity_manager.get_component_write_handle::<StubComponentB>();
    let ita = compha.get_mut_iter();
    let itb = comphb.get_mut_iter();
    let joint = ita.join(itb);
    let mut jit = joint.into_iterator_wrapper();
    let res = jit.next();
    let unwrapped = res.unwrap();
    assert_eq!((unwrapped.0).counter, 0);
    assert_eq!((unwrapped.1).counter, 100);
}


#[test]
fn iterator_vec_test(){
    let mut entity_manager = ECS::new();
    entity_manager.allocate_new_entity();
    entity_manager.allocate_new_entity();
    entity_manager.allocate_new_entity();
    entity_manager.register_new_component::<StubComponentA>().expect("unable to register new component");
    entity_manager.register_new_component::<StubComponentB>().expect("unable to register new component");
    entity_manager.add_component((0,0), StubComponentA{ counter: 0 }).expect("not registered");
    entity_manager.add_component((1,0), StubComponentA{counter: 0}).expect("not registered");
    entity_manager.add_component((1,0), StubComponentB{counter: 100}).expect("not registered");
    entity_manager.add_component((2,0), StubComponentA{counter: 0}).expect("not registered");
    let mut compha = entity_manager.get_component_write_handle::<StubComponentA>();
    let ita = compha.get_mut_iter();
    let jit = ita.into_iterator_wrapper();
    let result: Vec<&mut Box<StubComponentA>> = jit.collect();
    assert_eq!(result.len(), 3);
}

#[test]
fn iterator_joint_vec_test(){
    let mut entity_manager = ECS::new();
    entity_manager.allocate_new_entity();
    entity_manager.allocate_new_entity();
    entity_manager.allocate_new_entity();
    entity_manager.register_new_component::<StubComponentA>().expect("unable to register new component");
    entity_manager.register_new_component::<StubComponentB>().expect("unable to register new component");
    entity_manager.add_component((0,0), StubComponentA{ counter: 0 }).expect("not registered");
    entity_manager.add_component((0,0), StubComponentB{ counter: 0 }).expect("not registered");
    entity_manager.add_component((1,0), StubComponentA{counter: 0}).expect("not registered");
    entity_manager.add_component((1,0), StubComponentB{counter: 100}).expect("not registered");
    entity_manager.add_component((2,0), StubComponentA{counter: 0}).expect("not registered");
    entity_manager.add_component((2,0), StubComponentB{counter: 0}).expect("not registered");
    let mut compha = entity_manager.get_component_write_handle::<StubComponentA>();
    let mut comphb = entity_manager.get_component_write_handle::<StubComponentB>();
    let ita = compha.get_mut_iter();
    let itb = comphb.get_mut_iter();
    let joint = ita.join(itb);
    let jit = joint.into_iterator_wrapper();
    let result: Vec<(&mut Box<StubComponentA>, &mut Box<StubComponentB>)> = jit.collect();
    assert_eq!(result.len(), 3);
}


#[test]
fn iterator_joint_uneven_vec_test(){
    let mut entity_manager = ECS::new();
    entity_manager.allocate_new_entity();
    entity_manager.allocate_new_entity();
    entity_manager.allocate_new_entity();
    entity_manager.register_new_component::<StubComponentA>().expect("unable to register new component");
    entity_manager.register_new_component::<StubComponentB>().expect("unable to register new component");
    entity_manager.add_component((0,0), StubComponentA{ counter: 0 }).expect("not registered");
    entity_manager.add_component((1,0), StubComponentA{counter: 0}).expect("not registered");
    entity_manager.add_component((1,0), StubComponentB{counter: 100}).expect("not registered");
    entity_manager.add_component((2,0), StubComponentA{counter: 0}).expect("not registered");
    let mut compha = entity_manager.get_component_write_handle::<StubComponentA>();
    let mut comphb = entity_manager.get_component_write_handle::<StubComponentB>();
    let ita = compha.get_mut_iter();
    let itb = comphb.get_mut_iter();
    let joint = ita.join(itb);
    let jit = joint.into_iterator_wrapper();
    let result = jit.collect::<Vec<_>>();
    assert_eq!(result.len(), 1);
}

#[test]
fn entity_iterator_test() {
    let mut entity_manager = ECS::new();
    entity_manager.allocate_new_entity();
    entity_manager.allocate_new_entity();
    entity_manager.allocate_new_entity();
    entity_manager.register_new_component::<StubComponentA>().expect("unable to register new component");
    entity_manager.register_new_component::<StubComponentB>().expect("unable to register new component");
    entity_manager.add_component((0, 0), StubComponentA { counter: 0 }).expect("not registered");
    entity_manager.add_component((1, 0), StubComponentA { counter: 0 }).expect("not registered");
    entity_manager.add_component((1, 0), StubComponentB { counter: 100 }).expect("not registered");
    entity_manager.add_component((2, 0), StubComponentA { counter: 0 }).expect("not registered");
    let ent_it = entity_manager.get_entity_iterator_live();
    let mut compha = entity_manager.get_component_write_handle::<StubComponentA>();
    let ita = compha.get_mut_iter();
    let jointea = ent_it.join(ita);
    let jointwrapper = jointea.into_iterator_wrapper();
    let collect = jointwrapper.collect::<Vec<_>>();
    assert_eq!(collect.len(), 3);
}

#[test]
fn entity_iterator_live_only_test(){
    let mut entity_manager = ECS::new();
    let _entity1 = entity_manager.allocate_new_entity();
    let entity2 = entity_manager.allocate_new_entity();
    let _entity3 = entity_manager.allocate_new_entity();
    let _res = entity_manager.deallocate_entity(entity2).expect("Error when: ");
    let it = entity_manager.get_entity_iterator_live();
    let wrapper = it.into_iterator_wrapper();
    let result = wrapper.collect::<Vec<_>>();
    assert_eq!(result.len(), 2);
}
#[test]
fn entity_iterator_joint_test(){
    let mut entity_manager = ECS::new();
    let entity1 = entity_manager.allocate_new_entity();
    let entity2= entity_manager.allocate_new_entity();
    let entity3 =entity_manager.allocate_new_entity();
    entity_manager.register_new_component::<StubComponentA>().expect("unable to register new component");
    entity_manager.register_new_component::<StubComponentB>().expect("unable to register new component");
    entity_manager.add_component(entity1, StubComponentA { counter: 0 }).expect("not registered");
    entity_manager.add_component(entity2, StubComponentA { counter: 0 }).expect("not registered");
    entity_manager.add_component(entity2, StubComponentB { counter: 100 }).expect("not registered");
    entity_manager.add_component(entity3, StubComponentA { counter: 0 }).expect("not registered");
    let _res = entity_manager.deallocate_entity(entity2).expect("Error when: ");
    let it = entity_manager.get_entity_iterator_live();
    let ah = entity_manager.get_component_read_handle::<StubComponentA>();
    let ita = ah.get_iterator();
    let jointiter = it.join(ita);
    let jiter = jointiter.into_iterator_wrapper();
    let result = jiter.collect::<Vec<_>>();
    assert_eq!(result.len(), 2);
}

#[derive(Debug)]
struct DeltaTime(f32);

#[test]
fn ecs_add_resource(){
    let mut ecs = ECS::new();
    let resource = DeltaTime(32.5);
    ecs.insert_new_resource(resource);
    let resource_handle = ecs.get_resource::<DeltaTime>().unwrap();
    assert_eq!(32.5, resource_handle.r.0);
}

#[test]
fn ecs_remove_resource(){
    let mut ecs = ECS::new();
    let resource = DeltaTime(32.5);
    ecs.insert_new_resource(resource);
    let result = ecs.remove_resource::<DeltaTime>().unwrap();
    assert_eq!(32.5, (result.get().r.0));
}

#[test]
fn ecs_get_mut_resource(){
    let mut ecs = ECS::new();
    let resource = DeltaTime(32.5);
    ecs.insert_new_resource(resource);
    {
        let mut resource_handle = ecs.get_mut_resource::<DeltaTime>().unwrap();
        resource_handle.r.0 += 12.5;
    }
    let resource_handle = ecs.get_resource::<DeltaTime>().unwrap();
    assert_eq!(45.0, resource_handle.r.0)
}

