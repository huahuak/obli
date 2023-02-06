use proto::{
  collection::vector_generated::org::kaihua::obliop,
  context::{Expression, ObliData},
};

pub fn hash_exec(input: ObliData, output: ObliData) {
  assert!(input.prepared, "[hasher.rs::hash_exec()] input isn't prepared !!!");
  
}
