use downcast_rs::Downcast;

trait System: Downcast{
    fn update(&mut self){unimplemented!()}
}
impl_downcast!(System);
