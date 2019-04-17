use nphysics3d::object::ColliderHandle;
use ecs::component::Component;
use ecs::component::dense_component_storage::DenseComponentStorage;

#[derive(Clone)]
pub struct Collider(pub ColliderHandle);

impl Component for Collider {
    type ComponentStorage = DenseComponentStorage<Self>;
}