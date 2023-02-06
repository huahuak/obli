use flatbuffers::{FlatBufferBuilder, WIPOffset};

use super::vector_generated::org::kaihua::obliop::collection::fbs::Field;

pub struct FbsRowTable<'fbb> {
  fbb: FlatBufferBuilder<'fbb>,
  // rows: Vec<WIPOffset<Field<'fbb>>>,
}

impl<'fbb> FbsRowTable<'fbb> {
  pub fn new() -> FbsRowTable<'fbb> {
    FbsRowTable {
      fbb: FlatBufferBuilder::with_capacity(1024),
    }
  }

  pub fn create_fields_cell() {}
}
