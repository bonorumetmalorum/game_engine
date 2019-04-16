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
use kiss3d::window::State;

pub struct Engine {
    ecs: ECS,
    physicsworld: World<f23> //has to be kept here instead of in the resources due to limitation of nphysics library
}

impl Engine {

    pub fn new() -> Engine {
        let mut ecs = ECS::new();
        let world: World<f32> = nphysics3d::world::World::new();
        let _ = ecs.register_new_component::<BaseColor>();
        let _ = ecs.register_new_component::<Collider>();
        let _ = ecs.register_new_component::<Color>();
        let _ = ecs.register_new_component::<Delta>();
        let _ = ecs.register_new_component::<Node>();
        Engine { ecs, physicsworld: world }
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
        self.physicsworld.add_rigid_body();
        let handle = rb_desc.build(&self.physicsworld);
        self.ecs.add_component(new_ent, RigidBodyComponent(handle));
        //add the scene node to the window
        let mut window = self.ecs.get_mut_resource::<Window>().unwrap();
        let node = window.add_sphere(rad);
        let _ = self.ecs.add_component(new_ent, Gfx(node));
    }
    /*
        look up proximity queries and what they are...
        need to add/remove code to do with activation surface and line width depending on findings
    */
    pub fn create_box(&mut self, rad: f32, delta: Isometry3<f32>) -> EntityIndex {
        let cube = ShapeHandle::new(Cuboid::new(Vector3::repeat(rad)));
        let handle = ColliderDesc::new(cube).set_position(delta).build(&self.physicsworld);
        let new_entity = self.ecs.allocate_new_entity();
        let _ = self.ecs.add_component(new_entity, Collider(handle));
        let mut window = self.ecs.get_resource::<Window>().unwrap();
        let mut scenenode = window.add_cube(rad, rad, rad);
        scenenode.set_surface_rendering_activation(false);
        scenenode.set_lines_width(1.0);
        let _ = self.ecs.add_component(new_entity, Gfx(scenenode));
    }
}

impl State for Engine {
    fn step(&mut self, window: &mut Window) {
        self.physicsworld.step();
        unimplemented!()
    }
}