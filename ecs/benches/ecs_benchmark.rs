#[macro_use]
extern crate criterion;
extern crate ecs;
use ecs::entity::*;
use ecs::component::*;
use criterion::Criterion;

fn ecs_allocate_new_entity(){
    let mut ecs = management::EntityStorage::new();
    ecs.allocate_new_entity();
}
fn ecs_deallocate_empty_entity(){
    let mut ecs = management::EntityStorage::new();
    let res = ecs.allocate_new_entity();
    ecs.deallocate_entity(res);
}
fn ecs_deallocate_entity_with_component(){
    let mut ecs = management::EntityStorage::new();
    let res = ecs.allocate_new_entity();
    ecs.register_new_component::<StubPosition>();
    let res2 = ecs.add_component(res, StubPosition{x: 0.0, y: 0.0}).unwrap();
    ecs.deallocate_entity(res2);
}

fn ecs_register_component(){
    let mut ecs = management::EntityStorage::new();
    let res = ecs.register_new_component::<StubPosition>();
}

fn ecs_add_new_component(){
    let mut ecs = management::EntityStorage::new();
    let idx = ecs.allocate_new_entity();
    ecs.register_new_component::<StubPosition>();
    let res2 = ecs.add_component(idx, StubPosition{x: 0.0, y: 0.0});
}

fn ecs_remove_component(){
    let mut ecs = management::EntityStorage::new();
    let idx = ecs.allocate_new_entity();
    ecs.register_new_component::<StubPosition>();
    let res2 = ecs.add_component(idx, StubPosition{x: 0.0, y: 0.0}).unwrap();
    ecs.remove_component::<StubPosition>(res2);
}

fn ecs_fetch_component(){
    let mut ecs = management::EntityStorage::new();
    let idx = ecs.allocate_new_entity();
    ecs.register_new_component::<StubPosition>();
    let res2 = ecs.add_component(idx, StubPosition{x: 0.0, y: 0.0}).unwrap();
    let result = ecs.fetch::<StubPosition>(res2);
}

fn ecs_bench(c: &mut Criterion){
    c.bench_function("ecs add  new empty entity", move |b| b.iter(|| ecs_allocate_new_entity()));
    c.bench_function("ecs deallocate empty entity", move |b| b.iter(|| ecs_deallocate_empty_entity()));
    c.bench_function("ecs deallocate entity with component", move |b| b.iter(|| ecs_deallocate_entity_with_component()));
    c.bench_function("ecs register new component", move |b| b.iter(|| ecs_register_component()));
    c.bench_function("ecs add new component", move |b| b.iter(|| ecs_add_new_component()));
    c.bench_function("ecs remove component", move |b| b.iter(|| ecs_remove_component()));
    c.bench_function("ecs fetch component", move |b| b.iter(|| ecs_fetch_component()));
}
criterion_group!(benches, ecs_bench);
criterion_main!(benches);