use ecs::component::Component;
use ecs::component::dense_component_storage::DenseComponentStorage;


#[derive(Clone)]
pub enum Node {
    Plane,
    Ball,
    Box,
    Cylinder,
    Cone,
    Capsule,
    Mesh,
    HeightField,
    Convex,
}

impl Component for Node {
    type ComponentStorage = DenseComponentStorage<Self>;
}