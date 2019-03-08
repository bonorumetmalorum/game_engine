extern crate kiss3d;
extern crate ncollide3d;
extern crate nphysics3d;
extern crate nalgebra;

use kiss3d::window::Window;
use kiss3d::light::Light;
use nalgebra::{Vector3, UnitQuaternion};
use ecs::ECS;

fn main() {
    let mut ecs = setup_entities();
    let mut window = Window::new("physics demo");
    let mut c = window.add_cube(1.0,1.0,1.0);
    window.set_light(Light::StickToCamera);
    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() {
        c.prepend_to_local_rotation(&rot);
    }
}

fn setup_entities() -> ECS {
    let mut ecs = ECS::new();
    for i in 0..10000 {
        let handle = ecs.allocate_new_entity();
        //register components for NPhysics and Ncollide
        //register components for Kiss3D
        //add entities to Kiss3d scene
    }
    ecs
}
