use nphysics3d::object::BodyHandle;
use ecs::component::Component;
use ecs::component::dense_component_storage::DenseComponentStorage;
use nphysics3d::object::RigidBody;


#[derive(Clone)]
pub struct RigidBodyComponent<'a>(pub &'a mut RigidBody<f32>);

impl Component for RigidBodyComponent<'static>{
    type ComponentStorage = DenseComponentStorage<Self>;
}

