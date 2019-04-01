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

pub struct Engine {
    ecs: ECS
}

impl Engine {

    pub fn create_plane(
        &mut self,
        object: ColliderHandle,
        color: Point3<f32>,
    ) {
        let pos = world.collider(object).unwrap().position();
        let position = Point3::from(pos.translation.vector);
        let normal = pos * shape.normal();
    }

    fn create_heightfield(
        &mut self,
        object: ColliderHandle,
        delta: Isometry3<f32>,
        color: Point3<f32>,
    ) {

    }

    fn create_capsule(
        &mut self,
        object: ColliderHandle,
        delta: Isometry3<f32>,
        color: Point3<f32>,
    ) {
    }

    fn create_ball(
        &mut self,
        object: ColliderHandle,
        delta: Isometry3<f32>,
        color: Point3<f32>)
    {
        //shared resources
        let mut world = self.ecs.get_resource::<World<f32>>().unwrap();
        let mut window = self.ecs.get_resource::<Window>().unwrap();
        let mut ex: SceneNode;
        //allocate a new entity
        let new_ent = self.ecs.allocate_new_entity();
        //create ball
        let ball = ShapeHandle::new(Ball::new(rad));
        let collider_desc = ColliderDesc::new(ball)
            .density(1.0);
        //rigid body creation
        let mut rb_desc = RigidBodyDesc::new()
            .collider(&collider_desc);
        //build the ball in the phys world
        rb_desc.build(world);
        self.ecs.add_component(new_ent, Collider(object));
    }

    /*
        creates a box at position x y z with a specified color
    */
    fn create_box(
        &mut self,
        object: ColliderHandle,
        delta: Isometry3<f32>,
        color: Point3<f32>,
    ) {
        let margin = world.collider(object).unwrap().margin();
        let rx = shape.half_extents().x + margin;
        let ry = shape.half_extents().y + margin;
        let rz = shape.half_extents().z + margin;
        //add to ecs
    }

    fn create_convex(
        &mut self,
        object: ColliderHandle,
        delta: Isometry3<f32>,
        color: Point3<f32>,
    ) {
        let mut chull = transformation::convex_hull(shape.points());
        chull.replicate_vertices();
        chull.recompute_normals();
        //add to ecs
    }
}