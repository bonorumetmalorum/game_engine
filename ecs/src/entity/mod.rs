pub mod management;
///represents an `Entity`
pub type Entity = usize;
///the index generation
pub type Generation = u64;
///the stored index of an `Entity`
pub type EntityIndex = (Entity, Generation);

