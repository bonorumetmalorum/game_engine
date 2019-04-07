use nphysics3d::object::BodyHandle;
use ecs::component::Component;
use ecs::component::dense_component_storage::DenseComponentStorage;
use nphysics3d::object::RigidBody;

pub struct RigidBodyComponent<'a>(&'a mut RigidBody<f32>);

impl Component for RigidBodyComponent{
    type ComponentStorage = DenseComponentStorage<Self>;
}

