///currently unsupported but will be beneficial in preventing deadlocks.
///This interface can be used to reliably and predictably acquire locks/handles.
trait System{
    fn update(&mut self){unimplemented!()}
}