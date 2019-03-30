use kiss3d::scene::SceneNode;
use ecs::component::Component;
use ecs::component::dense_component_storage::DenseComponentStorage;

pub struct Gfx(SceneNode);

impl Component for Gfx {
    type ComponentStorage = DenseComponentStorage<Self>;
}