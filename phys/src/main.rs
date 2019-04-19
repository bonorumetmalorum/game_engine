extern crate kiss3d;
extern crate ncollide3d;
extern crate nphysics3d;
extern crate nalgebra;

use kiss3d::window::Window;
use kiss3d::light::Light;
use nalgebra::{Vector3, UnitQuaternion};
use ecs::ECS;
use nphysics3d::object::{RigidBody, RigidBodyDesc, ColliderDesc, BodyHandle};
use ncollide3d::shape::{ShapeHandle, Ball, Plane, Cuboid};
use nphysics3d::world::World;
use ecs::component::Component;
use ecs::component::dense_component_storage::DenseComponentStorage;
use kiss3d::scene::SceneNode;
use crate::engine::Engine;
use nalgebra::geometry::{Isometry3, Translation, Translation3, Rotation3};

pub mod objects;
pub mod engine;

#[derive(Clone)]
pub struct PhysicsComponent {
    handle: BodyHandle
}

impl Component for PhysicsComponent {
    type ComponentStorage = DenseComponentStorage<Self>;
}

#[derive(Clone)]
pub struct RenderComponent {
    node: SceneNode
}

impl Component for RenderComponent {
    type ComponentStorage = DenseComponentStorage<Self>;
}

fn main() {
//    let mut ecs = setup_ecs();
//    let mut window = Window::new("physics demo");
//    let mut c = window.add_cube(1.0,1.0,1.0);
//    window.set_light(Light::StickToCamera);
//    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
//
//    while window.render() {
//        c.prepend_to_local_rotation(&rot);
//    }
    let mut engine = Engine::new();
    engine.create_ball(
        0.1,
        Isometry3::identity()
    );
    engine.run();
}

fn setup_ecs() -> ECS {
    let mut world = World::new();
    world.set_gravity(Vector3::new(0.0, -9.81, 0.0)); //physics world
    let mut ecs = ECS::new();

    //ball entity
    let ball = ShapeHandle::new(Ball::new(0.1));
    let collider_desc = ColliderDesc::new(ball).density(1.0);
    let mut rb_desc = RigidBodyDesc::new().collider(&collider_desc).build(&mut world).handle();

    //ground -- figure out how ground body handle words and add collider to it.
    let ground = BodyHandle::ground();
    let ground_size = 50.0;
    let ground_shape = ShapeHandle::new(Cuboid::new(Vector3::repeat(ground_size)));
    let physhandle = ColliderDesc::new(ground_shape).translation(Vector3::y() * -ground_size).build(&mut world).handle();


    let res = ecs.register_new_component::<PhysicsComponent>();
    let res = ecs.register_new_component::<RenderComponent>();
    ecs.insert_new_resource(world); //store physics world in resources
    ecs
}
