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
use crate::objects::rigidbody::RigidBodyComponent;
use crate::objects::gfx::Gfx;
use ecs::entity::EntityIndex;
use kiss3d::window::State;
use nalgebra::Vector3;
use core::mem;

pub struct Engine {
    ecs: ECS,
    physicsworld: World<f32>,
    window: Option<Box<Window>>
}

impl Engine {

    pub fn new() -> Engine {
        let mut ecs = ECS::new();
        let world: World<f32> = nphysics3d::world::World::new();
        let mut window = Window::new("phyics window");
        let _ = ecs.register_new_component::<BaseColor>();
        let _ = ecs.register_new_component::<RigidBodyComponent>();
        let _ = ecs.register_new_component::<Gfx>();
        let _ = ecs.register_new_component::<Collider>();
        let _ = ecs.register_new_component::<Color>();
        let _ = ecs.register_new_component::<Delta>();
        let _ = ecs.register_new_component::<Node>();
        Engine { ecs, physicsworld: world, window: Some(Box::new(window)) }
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
        rb_desc.set_position(delta);
        let handle = rb_desc.build(&mut self.physicsworld).handle();
        self.ecs.add_component(new_ent, RigidBodyComponent(handle)).unwrap();
//        let scenehandle = match &self.window {
//            Some(mut w) => Ok(w.add_sphere(rad)),
//            None => Err("unable to add, program running or window is not set")
//        }.unwrap();
        let window = self.window.iter_mut().next().unwrap();
        let mut scenehandle = window.add_sphere(rad);
        scenehandle.set_local_transformation(delta);
        //add the scene node to the window
        self.ecs.add_component(new_ent, Gfx(scenehandle)).unwrap()
    }
    /*
        look up proximity queries and what they are...
        need to add/remove code to do with activation surface and line width depending on findings
    */
    pub fn create_box(&mut self, rad: f32, delta: Isometry3<f32>) -> EntityIndex {
        let cube = ShapeHandle::new(Cuboid::new(Vector3::repeat(rad)));
        let handle = ColliderDesc::new(cube).set_position(delta).build(&mut self.physicsworld).handle();
        let new_entity = self.ecs.allocate_new_entity();
        self.ecs.add_component(new_entity, Collider(handle)).unwrap();
        let mut window = self.window.iter_mut().next().unwrap();
        let mut handle= window.add_cube(rad, rad, rad);
        handle.set_surface_rendering_activation(false);
        handle.set_lines_width(1.0);
        handle.set_local_transformation(delta);
        self.ecs.add_component(new_entity, Gfx(handle)).unwrap()
    }

    pub fn run(mut self) {
        let window = mem::replace(&mut self.window, None).unwrap();
        window.render_loop(self)
    }
}

impl State for Engine {
    fn step(&mut self, window: &mut Window) {
        self.physicsworld.step();
    }
}