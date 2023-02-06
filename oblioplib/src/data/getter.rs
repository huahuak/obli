use std::sync::Arc;

use flatbuffers::{ForwardsUOffset, VectorIter};
use proto::{
  collection::vector_generated::org::kaihua::obliop::collection::fbs::{Field, Row, RowTable},
  context::ObliData,
};

use super::manager::{DataManager, DATA_MANAGER};

pub struct DataIterator<'a> {
  data: &'a ObliData,
  buf: Arc<Vec<u8>>,
  row_iter: Option<VectorIter<'a, ForwardsUOffset<Row<'a>>>>,
}

impl<'a> DataIterator<'a> {
  pub fn new(data: &'a ObliData) -> DataIterator {
    let dm = DATA_MANAGER.exclusive_access();
    let buf = Arc::clone(&dm.get_data_buffer_by_key(&data.id));
    let r_table = flatbuffers::root::<RowTable>(&buf[..]).unwrap();
    let row_iter = r_table.rows().unwrap().into_iter();
    DataIterator {
      data,
      buf,
      row_iter: None,
    }
  }
}

impl<'a> Iterator for DataIterator<'a> {
  type Item = Field<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    None
  }
}
