use nphysics3d::object::ColliderHandle;
use ecs::component::Component;
use ecs::component::dense_component_storage::DenseComponentStorage;

pub struct Collider(ColliderHandle);

impl Component for Collider {
    type ComponentStorage = DenseComponentStorage<Self>;
}