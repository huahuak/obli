use std::cmp::Ordering;

use proto::protocol::context::ObliData;

use crate::data::getter::{DataIterator, Record};

pub fn sort_exec(input: &ObliData, output: &ObliData) -> Result<(), &'static str> {
  let iter = &mut DataIterator::new(input);
  let mut data_vec: Vec<Record> = iter.into();
  data_vec.sort_by(|a, b| {
    Ordering::Less
  });
  Ok(())
}
