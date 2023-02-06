use std::{
  option::IntoIter,
  sync::{Arc, Mutex},
};

use flatbuffers::{ForwardsUOffset, Vector, VectorIter};
use proto::{
  collection::vector_generated::org::kaihua::obliop::collection::fbs::{Field, Row, RowTable},
  context::ObliData,
};

use super::manager::DATA_MANAGER;

pub struct DataIterator<'a> {
  data: &'a ObliData,
  buf: Arc<Mutex<Vec<u8>>>,
  row_iter: Option<IntoIter<Vector<'a, ForwardsUOffset<Row<'a>>>>>,
}

impl<'a> DataIterator<'a> {
  fn new(data: &'a ObliData) -> DataIterator {
    let dm = DATA_MANAGER.exclusive_access();
    let buf = Arc::clone(&dm.get_data(&data.id).unwrap().buffer);
    let iter = DataIterator {
      data,
      buf,
      row_iter: None,
    };
    iter
  }
}

impl<'a> Iterator for DataIterator<'a> {
  type Item = Field<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    None
  }
}
