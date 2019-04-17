use nalgebra::geometry::Isometry3;
use ecs::component::Component;
use ecs::component::dense_component_storage::DenseComponentStorage;

#[derive(Clone)]
pub struct Delta(Isometry3<f32>);

impl Component for Delta {
    type ComponentStorage = DenseComponentStorage<Self>;
}