use nalgebra::geometry::Point3;
use ecs::component::Component;
use ecs::component::dense_component_storage::DenseComponentStorage;

#[derive(Clone)]
pub struct BaseColor(Point3<f32>);

impl Component for BaseColor {
    type ComponentStorage = DenseComponentStorage<Self>;
}