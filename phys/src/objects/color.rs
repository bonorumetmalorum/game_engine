use nalgebra::geometry::Point3;
use ecs::component::Component;
use ecs::component::dense_component_storage::DenseComponentStorage;

#[derive(Clone)]
pub struct Color(Point3<f32>);

impl Component for Color {
    type ComponentStorage = DenseComponentStorage<Self>;
}