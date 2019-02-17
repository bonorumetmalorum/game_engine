#[macro_use]
extern crate criterion;
extern crate ecs;
use ecs::entity::*;
use ecs::component::*;
use criterion::Criterion;
use ecs::ECS;
use std::thread;

const NUM_ENTITIES: u32 = 10000;

fn build() -> (Vec<EntityIndex>, ECS) {
    let mut entities = vec![];
    let mut ecs = ECS::new();
    ecs.register_new_component::<StubPosition>();
    ecs.register_new_component::<StubVelocity>();
    for _ in 0..NUM_ENTITIES {
        entities.push(ecs.allocate_new_entity());
    }
    (entities, ecs)
}

fn ecs_allocate_new_entities(c: &mut Criterion){
    c.bench_function("ecs add  new empty entities", move |b| b.iter(|| {build();}));
}
fn ecs_deallocate_empty_entity(c: &mut Criterion){
    let (entities, mut ecs) = build();
    c.bench_function("ecs deallocate empty entity", move |b| b.iter( ||{ecs.deallocate_entity(entities[50]);}));
}
fn ecs_deallocate_entity_with_component(c: &mut Criterion){
    let (entities, mut ecs) = build();
    ecs.register_new_component::<StubPosition>();
    let res2 = ecs.add_component(entities[99], StubPosition{x: 0.0, y: 0.0}).unwrap();
    c.bench_function("ecs deallocate entities with component", move |b| b.iter(||{ecs.deallocate_entity(res2);}));
}

fn ecs_register_component(c: &mut Criterion){
    let (entities, mut ecs) = build();
    c.bench_function("ecs register new component", move |b| b.iter(|| {ecs.register_new_component::<StubPosition>();}));
}

fn ecs_add_new_component(c: &mut Criterion){
    let (entities, mut ecs) = build();
    ecs.register_new_component::<StubPosition>();
    c.bench_function("ecs add new component", move |b| b.iter(|| {ecs.add_component(entities[20], StubPosition{x: 0.0, y: 0.0});}));
}

fn ecs_remove_component(c: &mut Criterion){
    let (entities, mut ecs) = build();
    ecs.register_new_component::<StubPosition>();
    let res2 = ecs.add_component(entities[66], StubPosition{x: 0.0, y: 0.0}).unwrap();
    c.bench_function("ecs remove component", move |b| b.iter(|| {ecs.remove_component::<StubPosition>(res2);}));
}

fn ecs_fetch_component(c: &mut Criterion){
    let (entities, mut ecs) = build();
    ecs.register_new_component::<StubPosition>();
    let res2 = ecs.add_component(entities[99], StubPosition{x: 0.0, y: 0.0}).unwrap();
    c.bench_function("ecs fetch component", move |b| b.iter(||{
        let poshandle = ecs.get_component_read_handle::<StubPosition>();
        let mut iterator = poshandle.get_iterator();
        let mut iteratorwrapper = iterator.into_iterator_wrapper();
        let result = iteratorwrapper.collect::<Vec<_>>();
    }));
}

pub fn setup_parallel() -> ECS{
    let (entities, mut ecs) = build();
    ecs.register_new_component::<R>();
    ecs.register_new_component::<W1>();
    ecs.register_new_component::<W2>();
    for ent in entities {
        ecs.add_component(ent, R { x: 32.0 });
        ecs.add_component(ent, W1 { x: 0.0 });
        ecs.add_component(ent, W2 { x: 0.0 });
    }
    ecs
}

fn ecs_sequential_systems(c: &mut Criterion) {
    let mut ecs = setup_parallel();
    c.bench_function("ecs sequential systems", move |b| b.iter( ||{
        let hr1 = ecs.get_component_read_handle::<R>();
        let hr2 = ecs.get_component_read_handle::<R>();
        let mut hw1 = ecs.get_component_write_handle::<W1>();
        let mut hw2 = ecs.get_component_write_handle::<W2>();
        let mut itrr1 = hr1.get_iterator();
        let mut itrr2 = hr2.get_iterator();
        let itrw1 = hw1.get_mut_iter();
        let itrw2 = hw2.get_mut_iter();
        systemW1(itrr1, itrw1);
        systemW2(itrr2, itrw2);
    }
    ));
}

fn systemW1(mut readR: ComponentIterator<R>, mut rightW1: ComponentIteratorMut<W1>) {
    let mut joint = readR.join(rightW1);
    let mut iterator = joint.into_iterator_wrapper();
    for (r, w1) in iterator {
        w1.x = r.x;
    }
}

fn systemW2(mut readR: ComponentIterator<R>, mut rightW2: ComponentIteratorMut<W2>){
    let mut joint = readR.join(rightW2);
    let mut iterator = joint.into_iterator_wrapper();
    for (r, w2) in iterator {
        w2.x = r.x;
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

criterion_group!(benches, ecs_allocate_new_entities, ecs_deallocate_empty_entity, ecs_deallocate_entity_with_component, ecs_register_component, ecs_add_new_component, ecs_remove_component, ecs_fetch_component, ecs_sequential_systems);
criterion_main!(benches);