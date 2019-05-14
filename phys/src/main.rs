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
pub mod systems;

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
    let num= 10;
    let rad = 0.1;
    let shift = rad * 2.0 + 0.002;
    let centerx = shift * (num as f32) / 2.0;
    let centery = shift / 2.0;
    let centerz = shift * (num as f32) / 2.0;
    let height = 3.0;
    let mut engine = Engine::new();
    for i in 0usize..num {
        for j in 0usize..num {
            for k in 0usize..num {
                let x = i as f32 * shift - centerx;
                let y = j as f32 * shift + centery + height;
                let z = k as f32 * shift - centerz;

                // Build the rigid body and its collider.
                engine.create_ball(
                    rad,
                    Isometry3::new(Vector3::new(x, y , z), Vector3::new(0.0, 0.0, 0.0))
                );
            }
        }
    }
    engine.create_box(50.0, Isometry3::new(Vector3::new(0.0, 0.0 , 0.0), Vector3::new(0.0, 0.0, 0.0)));
    engine.run();
}
