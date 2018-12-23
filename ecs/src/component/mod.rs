pub trait Component{
    fn update(&mut self);
}

pub struct StubPosition{
    pub x: f32,
    pub y: f32,
}