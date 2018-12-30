#[macro_use]
extern crate criterion;
extern crate ecs;
use ecs::entity::*;
use ecs::component::*;
use criterion::Criterion;

const NUM_ENTITIES: u32 = 100;

fn build() -> (Vec<EntityIndex>, management::EntityStorage) {
    let mut entities = vec![];
    let mut ecs = management::EntityStorage::new();
    ecs.register_new_component::<StubPosition>();
    ecs.register_new_component::<StubVelocity>();
    for _ in 0..NUM_ENTITIES {
        entities.push(ecs.allocate_new_entity());
    }
    (entities, ecs)
}

fn ecs_allocate_new_entities(c: &mut Criterion){
    c.bench_function("ecs add  new empty entities", move |b| b.iter(|| build()));
}
fn ecs_deallocate_empty_entity(c: &mut Criterion){
    let (entities, mut ecs) = build();
    c.bench_function("ecs deallocate empty entity", move |b| b.iter( || ecs.deallocate_entity(entities[50])));
}
fn ecs_deallocate_entity_with_component(c: &mut Criterion){
    let (entities, mut ecs) = build();
    ecs.register_new_component::<StubPosition>();
    let res2 = ecs.add_component(entities[99], StubPosition{x: 0.0, y: 0.0}).unwrap();
    c.bench_function("ecs deallocate entities with component", move |b| b.iter(||     ecs.deallocate_entity(res2)));
}

fn ecs_register_component(c: &mut Criterion){
    let (entities, mut ecs) = build();
    c.bench_function("ecs register new component", move |b| b.iter(|| ecs.register_new_component::<StubPosition>()));
}

fn ecs_add_new_component(c: &mut Criterion){
    let (entities, mut ecs) = build();
    ecs.register_new_component::<StubPosition>();
    c.bench_function("ecs add new component", move |b| b.iter(|| ecs.add_component(entities[20], StubPosition{x: 0.0, y: 0.0})));
}

fn ecs_remove_component(c: &mut Criterion){
    let (entities, mut ecs) = build();
    ecs.register_new_component::<StubPosition>();
    let res2 = ecs.add_component(entities[66], StubPosition{x: 0.0, y: 0.0}).unwrap();
    c.bench_function("ecs remove component", move |b| b.iter(|| ecs.remove_component::<StubPosition>(res2)));
}

fn ecs_fetch_component(c: &mut Criterion){
    let (entities, mut ecs) = build();
    ecs.register_new_component::<StubPosition>();
    let res2 = ecs.add_component(entities[99], StubPosition{x: 0.0, y: 0.0}).unwrap();
    c.bench_function("ecs fetch component", move |b| b.iter(|| ecs.fetch::<StubPosition>(res2)));
}

criterion_group!(benches, ecs_allocate_new_entities, ecs_deallocate_empty_entity, ecs_deallocate_entity_with_component, ecs_register_component, ecs_add_new_component, ecs_remove_component, ecs_fetch_component);
criterion_main!(benches);