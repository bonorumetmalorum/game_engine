use downcast_rs::Downcast;

trait System{
//    type query;
    fn update(&mut self){unimplemented!()}
}
////query -> translated into iterators. Can be used to prevent deadlocks by ensuring predictable lock acquisition.
//trait Query: Downcast{}
//
////struct to hold a query and execute it
//pub struct SearchQuery{
//    search: Vec<Query>
//}


