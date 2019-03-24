#[macro_use]
extern crate criterion;
extern crate ecs;
extern crate core;

use ecs::entity::EntityIndex;
use ecs::ECS;
use ecs::component::StubVelocity;
use ecs::component::StubPosition;
use criterion::{Criterion, Bencher};
use ecs::component::iter::Iter;
use ecs::component::Component;
use ecs::component::dense_component_storage::DenseComponentStorage;
use ecs::component::dense_component_storage::DenseComponentIteratorMut;
use ecs::component::dense_component_storage::DenseComponentIterator;
use core::borrow::BorrowMut;
use ecs::component::storage::Storage;


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
    c.bench_function("adding entities with position and velocity components", move |b| b.iter(|| {setup_pos_vel();}));
}

fn ecs_deallocate_empty_entity(c: &mut Criterion){
    c.bench_function("deallocate 1 empty entity", move |b| b.iter_with_large_setup( || build(STANDARD), |(entities, mut ecs)|{ecs.deallocate_entity(entities[50]).expect("unable to deallocate entity");}));
}

fn ecs_deallocate_entity_with_component(c: &mut Criterion){
    c.bench_function("deallocate 1 entity with components", move |b| b.iter_with_large_setup(|| setup_pos_vel(), |mut ecs|{ecs.deallocate_entity((50, 0)).expect("unable to deallocate entity");}));
}

fn ecs_register_component(c: &mut Criterion){
    c.bench_function("registering a new component with the ECS", move |b| b.iter_with_large_setup(|| build(STANDARD) , |(_, mut ecs)| {ecs.register_new_component::<StubPosition>().expect("unable to register new component");}));
}

fn ecs_add_new_component(c: &mut Criterion){

    c.bench_function("adding a new component to an entity", move |b| b.iter_with_large_setup(||{
        let (entities, mut ecs) = build(STANDARD);
        ecs.register_new_component::<StubPosition>().expect("unable to register new component"); (entities, ecs)} ,
                                                                               |(entities, mut ecs )| {ecs.add_component(entities[20], StubPosition{x: 0.0, y: 0.0}).expect("not registered");}));
}

fn ecs_remove_component(c: &mut Criterion){
    c.bench_function("removing a component from an entity", move |b| b.iter_with_large_setup(|| setup_pos_vel(), |mut ecs| {ecs.remove_component::<StubPosition>((66, 0)).expect("unable to remove component");}));
}

fn ecs_fetch_component(c: &mut Criterion){
    let ecs = setup_pos_vel();
    c.bench_function("fetching all position components, irrespective of entities", move |b| b.iter(||{
        let mut poshandle = ecs.get_mut::<StubPosition>();
        let iterator = poshandle.borrow_mut();
        let iteratorwrapper = iterator.get_mut_iter().into_iterator_wrapper();
        let _result = iteratorwrapper.collect::<Vec<_>>();
    }));
}

fn ecs_pos_vel_update(c: &mut Criterion){
    let ecs = setup_pos_vel();
    c.bench_function("update position based on velocity for all entities", move |b|b.iter(||{
        let h1 = ecs.get::<StubVelocity>();
        let mut h2 = ecs.get_mut::<StubPosition>();
        let itrr1 = h1.get_iter();
        let itrr2 = h2.get_mut_iter();
        system_movement(itrr1, itrr2);
    }));
}

fn ecs_sequential_systems(c: &mut Criterion) {
    let ecs = setup_parallel();
    c.bench_function("2 systems updating entities sequentially", move |b| b.iter( ||{
        let hr1 = ecs.get::<R>();
        let hr2 = ecs.get::<R>();
        let mut hw1 = ecs.get_mut::<W1>();
        let mut hw2 = ecs.get_mut::<W2>();
        let itrr1 = hr1.get_iter();
        let itrr2 = hr2.get_iter();
        let itrw1 = hw1.get_mut_iter();
        let itrw2 = hw2.get_mut_iter();
        system_w1(itrr1, itrw1);
        system_w2(itrr2, itrw2);
    }
    ));
}

#[derive(Clone)]
struct PositionComponent {
    pub x: f32,
    pub y: f32
}
impl Component for PositionComponent {
    type ComponentStorage = DenseComponentStorage<Self>;
}
#[derive(Clone)]
struct DirectionComponent {
    pub x: f32,
    pub y: f32
}

impl Component for DirectionComponent {
    type ComponentStorage = DenseComponentStorage<Self>;
}

#[derive(Clone)]
struct ComflabulationComponent {
    pub thingy: f32,
    pub dingy: i32,
    pub mingy: bool,
    pub stringy: String
}

impl Component for ComflabulationComponent {
    type ComponentStorage = DenseComponentStorage<Self>;
}

fn setup_for_cpp_comp(noe: usize) -> (ECS, Vec<EntityIndex>) {
    let mut ecs = ECS::new();
    ecs.register_new_component::<DirectionComponent>().is_ok();
    ecs.register_new_component::<PositionComponent>().is_ok();
    ecs.register_new_component::<ComflabulationComponent>().is_ok();
    let mut handles = Vec::new();
    for i in 0..noe{
        let handle = ecs.allocate_new_entity();
        ecs.add_component(handle, PositionComponent{ x: 1.0, y: 1.0 });
        ecs.add_component(handle, DirectionComponent{ x: 1.0, y: 1.0 });
        if i % 2 == 0 {
            ecs.add_component(handle, ComflabulationComponent{
                thingy: 1.0,
                dingy: 1,
                mingy: false,
                stringy: "".to_string()
            });
        }
        handles.push(handle);
    }
    (ecs, handles)
}

fn update_system(read_r: DenseComponentIterator<DirectionComponent>, write_w1: DenseComponentIteratorMut<PositionComponent>) {
    let joint = read_r.join(write_w1);
    let iterator = joint.into_iterator_wrapper();
    let dt = 1.0 / 60.0;
    for (d, p) in iterator {
        p.x = d.x * dt;
        p.y += d.y * dt;
    }
}

fn comflab_system(comflabiter: DenseComponentIteratorMut<ComflabulationComponent>) {
    let iterator = comflabiter.into_iterator_wrapper();
    for comflab in iterator {
        comflab.thingy *= 1.000001;
        comflab.mingy = !comflab.mingy;
        comflab.dingy += 1;
    }
}

fn unpack_2_components(c: &mut Criterion) {
    c.bench_function_over_inputs("unpacking 2 components for all entities", |b:&mut Bencher, sample_size: &usize|{
        b.iter_with_large_setup(move ||{setup_for_cpp_comp(*sample_size)}, |(mut ecs, _)|{
            let directioncom = ecs.get::<DirectionComponent>();
            let mut poscomp = ecs.get_mut::<PositionComponent>();
            let directioniter = directioncom.get_iter();
            let positer = poscomp.get_mut_iter();
        })
    },
vec![25, 50, 100, 200, 400, 800, 1600, 3200, 5000, 10000, 30000, 100000]);
}

fn unpack_1_component(c: &mut Criterion) {
    c.bench_function_over_inputs("unpacking 1 component for all entities", |b: &mut Bencher, sample_size: &usize|{
        b.iter_with_large_setup(move ||{setup_for_cpp_comp(*sample_size)}, |(mut ecs, _)|{
            let directioncom = ecs.get::<DirectionComponent>();
            let directioniter = directioncom.get_iter();
        })
    },
vec![25, 50, 100, 200, 400, 800, 1600, 3200, 5000, 10000, 30000, 100000]);
}

fn creating_entities(c: &mut Criterion) {
    c.bench_function_over_inputs("allocating entities", |b: &mut Bencher, sample_size: &usize|{
        b.iter(move ||{
            setup_for_cpp_comp(*sample_size);
        })
    },
vec![25, 50, 100, 200, 400, 800, 1600, 3200, 5000, 10000, 30000, 100000]);
}

fn removing_entities(c: &mut Criterion) {
    c.bench_function_over_inputs("deallocating entities", |b: &mut Bencher, sample_size: &usize|{
        b.iter_with_large_setup(move ||{setup_for_cpp_comp(*sample_size)}, |(mut ecs, handles)|{
            for gen in handles.iter() {
                ecs.deallocate_entity(*gen);
            }
        })
    },
vec![25, 50, 100, 200, 400, 800, 1600, 3200, 5000, 10000, 30000, 100000]);
}

fn update_with_2_systems(c: &mut Criterion){
    c.bench_function_over_inputs("update all entities with 2 systems", |b: &mut Bencher, sample_size: &usize|{
        b.iter_with_large_setup( ||{setup_for_cpp_comp(*sample_size)}, |(ecs, _)|{
            let directioncom = ecs.get::<DirectionComponent>();
            let mut poscomp = ecs.get_mut::<PositionComponent>();
            let mut comflabcomp = ecs.get_mut::<ComflabulationComponent>();
            let directioniter = directioncom.get_iter();
            let positer = poscomp.get_mut_iter();
            let comflabiter = comflabcomp.get_mut_iter();
            update_system(directioniter, positer);
            comflab_system(comflabiter);
        })
    },
     vec![25, 50, 100, 200, 400, 800, 1600, 3200, 5000, 10000, 30000, 100000]);
}


fn system_w1(read_r: DenseComponentIterator<R>, write_w1: DenseComponentIteratorMut<W1>) {
    let joint = read_r.join(write_w1);
    let iterator = joint.into_iterator_wrapper();
    for (r, w1) in iterator {
        w1.x = r.x;
    }
}

fn system_w2(read_r: DenseComponentIterator<R>, write_w2: DenseComponentIteratorMut<W2>){
    let joint = read_r.join(write_w2);
    let iterator = joint.into_iterator_wrapper();
    for (r, w2) in iterator {
        w2.x = r.x;
    }
}

fn system_movement(read: DenseComponentIterator<StubVelocity>, writer: DenseComponentIteratorMut<StubPosition>) {
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
}

#[derive(Clone)]
struct W1{
    pub x: f32
}

impl Component for W1{
    type ComponentStorage = DenseComponentStorage<Self>;

}

#[derive(Clone)]
struct W2{
    pub x: f32
}

impl Component for W2{
    type ComponentStorage = DenseComponentStorage<Self>;
}

criterion_group!(benches,
                ecs_allocate_new_entities_pos_vel,
                ecs_deallocate_empty_entity,
                ecs_deallocate_entity_with_component,
                ecs_register_component, ecs_add_new_component,
                ecs_remove_component,
                ecs_fetch_component,
                ecs_pos_vel_update,
                ecs_sequential_systems,
                unpack_2_components,
                unpack_1_component,
                creating_entities,
                removing_entities,
                update_with_2_systems);

criterion_main!(benches);