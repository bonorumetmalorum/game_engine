use ecs::ECS;
use kiss3d::window::Window;
use nphysics3d::object::{ColliderHandle, ColliderDesc, RigidBodyDesc};
use nphysics3d::world::World;
use ncollide3d::shape::*;
use ecs::component::Component;
use std::any::TypeId;
use nalgebra::Isometry3;
use nalgebra::Point3;
use nalgebra::geometry::Isometry;
use crate::objects::collider::Collider;
use kiss3d::scene::SceneNode;
use crate::objects::base_color::BaseColor;
use crate::objects::color::Color;
use crate::objects::delta::Delta;
use crate::objects::node::Node;
use crate::objects::rigidbody::RigidBody;
use crate::objects::rigidbody::RigidBodyComponent;
use crate::objects::gfx::Gfx;
use ecs::entity::EntityIndex;

pub struct Engine {
    ecs: ECS,
    physicsworld: World<f23> //has to be kept here instead of in the resources due to limitation of nphysics library
}

impl Engine {

    pub fn new() -> Engine{
        let mut ecs = ECS::new();
        let world: World<f32> = nphysics3d::world::World::new();
        let _ = ecs.register_new_component::<BaseColor>();
        let _ = ecs.register_new_component::<Collider>();
        let _ = ecs.register_new_component::<Color>();
        let _ = ecs.register_new_component::<Delta>();
        let _ = ecs.register_new_component::<Node>();
        Engine{ ecs , physicsworld: world}
    }

    pub fn create_plane(&mut self, position: Isometry3<f32>) -> EntityIndex {

    }

    pub fn create_heightfield(&mut self, delta: Isometry3<f32>) -> EntityIndex {

    }

    pub fn create_capsule(&mut self, delta: Isometry3<f32>) -> EntityIndex {
    }

    pub fn create_ball(&mut self, rad: f32, delta: Isometry3<f32>) -> EntityIndex {
        //allocate a new entity
        let new_ent = self.ecs.allocate_new_entity();
        //create ball
        let ball = ShapeHandle::new(Ball::new(rad));
        let collider_desc = ColliderDesc::new(ball).density(1.0);
        //rigid body creation
        let mut rb_desc = RigidBodyDesc::new()
            .collider(&collider_desc);
        //build the ball in the phys world and add the component to the entity
        self.physicsworld.add_rigid_body();
        let handle = rb_desc.build(&self.physicsworld);
        self.ecs.add_component(new_ent, RigidBodyComponent(handle));
        //add the scene node to the window
        let mut window = self.ecs.get_mut_resource::<Window>().unwrap();
        let node = window.add_sphere(rad);
        let _ = self.ecs.add_component(new_ent, Gfx(node));
    }

    pub fn create_box(&mut self, delta: Isometry3<f32>) -> EntityIndex {
        let margin = world.collider(object).unwrap().margin();
        let rx = shape.half_extents().x + margin;
        let ry = shape.half_extents().y + margin;
        let rz = shape.half_extents().z + margin;
        //add to ecs
    }

    pub fn create_convex(&mut self, delta: Isometry3<f32>) -> EntityIndex {
        let mut chull = transformation::convex_hull(shape.points());
        chull.replicate_vertices();
        chull.recompute_normals();
        //add to ecs
    }
}