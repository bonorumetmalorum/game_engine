use nphysics3d::object::BodyHandle;
use ecs::component::Component;
use ecs::component::dense_component_storage::DenseComponentStorage;
use nphysics3d::object::RigidBody;


#[derive(Clone)]
pub struct RigidBodyComponent(pub BodyHandle);

impl Component for RigidBodyComponent{
    type ComponentStorage = DenseComponentStorage<Self>;
}