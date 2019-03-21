use ecs::ECS;
use kiss3d::window::Window;
use nphysics3d::object::ColliderHandle;
use nphysics3d::world::World;
use ncollide3d::shape::*;
use ecs::component::Component;
use std::any::TypeId;
use nalgebra::Isometry3;
use nalgebra::Point3;

pub struct Engine {
    ecs: ECS
}

impl Engine {

    pub fn add_shape(
    &mut self,
    window: &mut Window,
    object: ColliderHandle,
    world: &World<f32>,
    delta: Isometry3<f32>,
    shape: &Shape<f32>,
    color: Point3<f32>,
    ) {
        if let Some(s) = shape.as_shape::<Plane<f32>>() {
            self.add_plane(window, object, world, s, color, out)
        } else if let Some(s) = shape.as_shape::<Ball<f32>>() {
            self.add_ball(window, object, world, delta, s, color, out)
        } else if let Some(s) = shape.as_shape::<Cuboid<f32>>() {
            self.add_box(window, object, world, delta, s, color, out)
        } else if let Some(s) = shape.as_shape::<ConvexHull<f32>>() {
            self.add_convex(window, object, world, delta, s, color, out)
        } else if let Some(s) = shape.as_shape::<shape::Capsule<f32>>() {
            self.add_capsule(window, object, world, delta, s, color, out)
        } else if let Some(s) = shape.as_shape::<Compound<f32>>() {
            for &(t, ref s) in s.shapes().iter() {
            self.add_shape(window, object, world, delta * t, s.as_ref(), color)
            }
        } else if let Some(s) = shape.as_shape::<TriMesh<f32>>() {
            self.add_mesh(window, object, world, delta, s, color, out);
        } else if let Some(s) = shape.as_shape::<shape::HeightField<f32>>() {
            self.add_heightfield(window, object, world, delta, s, color, out);
        }
    }


}