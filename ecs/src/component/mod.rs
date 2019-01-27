pub trait Component{
    fn update(&mut self);
}

pub struct StubPosition{
    pub x: f32,
    pub y: f32,
}

pub struct StubVelocity{
    dx: f32,
    dy: f32,
}