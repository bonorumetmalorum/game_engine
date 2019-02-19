#[macro_use]
extern crate criterion;
extern crate ecs;
use ecs::entity::*;
use ecs::component::*;
use criterion::Criterion;
use ecs::ECS;

const NUM_POSITION_ONLY: usize = 9000;
const NUM_POSITION_AND_VELOCITY: usize = 1000;
const STANDARD: usize = 10000;

fn build(number: usize) -> (Vec<EntityIndex>, ECS) {
    let mut entities = vec![];
    let mut ecs = ECS::new();
    for _ in 0..number {
        entities.push(ecs.allocate_new_entity());
    }
    (entities, ecs)
}

fn setup_parallel() -> ECS{
    let (entities, mut ecs) = build(STANDARD);
    ecs.register_new_component::<R>().expect("unable to register new component");
    ecs.register_new_component::<W1>().expect("unable to register new component");
    ecs.register_new_component::<W2>().expect("unable to register new component");
    for ent in entities {
        ecs.add_component(ent, R { x: 32.0 }).expect("not registered");
        ecs.add_component(ent, W1 { x: 0.0 }).expect("not registered");
        ecs.add_component(ent, W2 { x: 0.0 }).expect("not registered");
    }
    ecs
}

fn setup_pos_vel() -> ECS {
    let (entities, mut ecs) = build(NUM_POSITION_ONLY);
    ecs.register_new_component::<StubVelocity>().expect("unable to register new component");
    ecs.register_new_component::<StubPosition>().expect("unable to register new component");
    for ent in entities {
        if ent.0 % NUM_POSITION_AND_VELOCITY == 0 {
            ecs.add_component(ent, StubVelocity { dx: 32.0, dy: 32.0 }).expect("not registered");
        }
        ecs.add_component(ent, StubPosition { x: 1.0, y: 10.0 }).expect("not registered");
    }
    ecs
}

fn ecs_allocate_new_entities_pos_vel(c: &mut Criterion){
    c.bench_function("ecs add  new empty entities", move |b| b.iter(|| {setup_pos_vel();}));
}

fn ecs_deallocate_empty_entity(c: &mut Criterion){
    c.bench_function("ecs deallocate empty entity", move |b| b.iter_with_large_setup( || build(STANDARD), |(entities, mut ecs)|{ecs.deallocate_entity(entities[50]).expect("unable to deallocate entity");}));
}

fn ecs_deallocate_entity_with_component(c: &mut Criterion){
    c.bench_function("ecs deallocate entities with component", move |b| b.iter_with_large_setup(|| setup_pos_vel(), |mut ecs|{ecs.deallocate_entity((50, 0)).expect("unable to deallocate entity");}));
}

fn ecs_register_component(c: &mut Criterion){
    c.bench_function("ecs register new component", move |b| b.iter_with_large_setup(|| build(STANDARD) , |(_, mut ecs)| {ecs.register_new_component::<StubPosition>().expect("unable to register new component");}));
}

fn ecs_add_new_component(c: &mut Criterion){

    c.bench_function("ecs add new component", move |b| b.iter_with_large_setup(||{
        let (entities, mut ecs) = build(STANDARD);
        ecs.register_new_component::<StubPosition>().expect("unable to register new component"); (entities, ecs)} ,
                                                                               |(entities, mut ecs )| {ecs.add_component(entities[20], StubPosition{x: 0.0, y: 0.0}).expect("not registered");}));
}

fn ecs_remove_component(c: &mut Criterion){
    c.bench_function("ecs remove component", move |b| b.iter_with_large_setup(|| setup_pos_vel(), |mut ecs| {ecs.remove_component::<StubPosition>((66, 0)).expect("unable to remove component");}));
}

fn ecs_fetch_component(c: &mut Criterion){
    let ecs = setup_pos_vel();
    c.bench_function("ecs fetch component", move |b| b.iter(||{
        let poshandle = ecs.get_component_read_handle::<StubPosition>();
        let iterator = poshandle.get_iterator();
        let iteratorwrapper = iterator.into_iterator_wrapper();
        let _result = iteratorwrapper.collect::<Vec<_>>();
    }));
}

fn ecs_pos_vel_update(c: &mut Criterion){
    let ecs = setup_pos_vel();
    c.bench_function("ecs_pos_vel_update", move |b|b.iter(||{
        let h1 = ecs.get_component_read_handle::<StubVelocity>();
        let mut h2 = ecs.get_component_write_handle::<StubPosition>();
        let itrr1 = h1.get_iterator();
        let itrr2 = h2.get_mut_iter();
        system_movement(itrr1, itrr2);
    }));
}

fn ecs_sequential_systems(c: &mut Criterion) {
    let ecs = setup_parallel();
    c.bench_function("ecs sequential systems", move |b| b.iter( ||{
        let hr1 = ecs.get_component_read_handle::<R>();
        let hr2 = ecs.get_component_read_handle::<R>();
        let mut hw1 = ecs.get_component_write_handle::<W1>();
        let mut hw2 = ecs.get_component_write_handle::<W2>();
        let itrr1 = hr1.get_iterator();
        let itrr2 = hr2.get_iterator();
        let itrw1 = hw1.get_mut_iter();
        let itrw2 = hw2.get_mut_iter();
        system_w1(itrr1, itrw1);
        system_w2(itrr2, itrw2);
    }
    ));
}

//parallel benchmark currently not working due to issues with lifetimes
//fn ecs_parallel_systems(c: &mut Criterion){
//    c.bench_function("ecs parallel systems", move |b| {
//        b.iter_with_large_setup(|| setup_parallel(), |ecs: ECS|{
//            let hr1 = ecs.get_component_read_handle::<R>();
//            let mut hw1 = ecs.get_component_write_handle::<W1>();
//
//            let hr2 = ecs.get_component_read_handle::<R>();
//            let mut hw2 = ecs.get_component_write_handle::<W2>();
//
//            let mut handle1 = thread::spawn(move ||{
//                let mut itrr1 = hr1.get_iterator();
//                let itrw1 = hw1.get_mut_iter();
//                systemW1(itrr1, itrw1);
//            });
//
//            let mut handle2 = thread::spawn( move ||{
//                let mut itrr2 = hr2.get_iterator();
//                let itrw2 = hw2.get_mut_iter();
//                systemW2(itrr2, itrw2);
//            });
//            handle1.join();
//            handle2.join();
//        })
//    });
//}



fn system_w1(read_r: ComponentIterator<R>, write_w1: ComponentIteratorMut<W1>) {
    let joint = read_r.join(write_w1);
    let iterator = joint.into_iterator_wrapper();
    for (r, w1) in iterator {
        w1.x = r.x;
    }
}

fn system_w2(read_r: ComponentIterator<R>, write_w2: ComponentIteratorMut<W2>){
    let joint = read_r.join(write_w2);
    let iterator = joint.into_iterator_wrapper();
    for (r, w2) in iterator {
        w2.x = r.x;
    }
}

fn system_movement(read: ComponentIterator<StubVelocity>, writer: ComponentIteratorMut<StubPosition>) {
    let joint = read.join(writer);
    let iter = joint.into_iterator_wrapper();
    for (v, p) in iter {
        p.x += v.dx;
        p.y += v.dy;
    }
}

#[derive(Clone)]
struct R{
    pub x: f32
}

impl Component for R{
    type ComponentStorage = DenseComponentStorage<Self>;

    fn update(&mut self) {
        unimplemented!()
    }
}

#[derive(Clone)]
struct W1{
    pub x: f32
}

impl Component for W1{
    type ComponentStorage = DenseComponentStorage<Self>;

    fn update(&mut self) {
        unimplemented!()
    }
}

#[derive(Clone)]
struct W2{
    pub x: f32
}

impl Component for W2{
    type ComponentStorage = DenseComponentStorage<Self>;

    fn update(&mut self) {
    }
}

criterion_group!(benches, ecs_allocate_new_entities_pos_vel, ecs_deallocate_empty_entity, ecs_deallocate_entity_with_component, ecs_register_component, ecs_add_new_component, ecs_remove_component, ecs_fetch_component, ecs_pos_vel_update, ecs_sequential_systems);
criterion_main!(benches);