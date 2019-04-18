use kiss3d::scene::SceneNode;
use ecs::component::Component;
use ecs::component::dense_component_storage::DenseComponentStorage;

#[derive(Clone)]
pub struct Gfx(pub SceneNode);

impl Component for Gfx {
    type ComponentStorage = DenseComponentStorage<Self>;
}