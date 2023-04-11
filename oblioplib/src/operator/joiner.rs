use core::str;
use std::vec;

use proto::protocol::context::{JoinKeyInfo, ObliData};

use crate::data::{
  manager::{Record, DATA_MANAGER},
  reader::DataIterator,
  writer::DataWriter,
};

/**
 * @author kahua.li
 * @email moflowerlkh@gmail.com
 * @date 2023/03/13
 **/

pub struct JoinManager {
  output_size: usize,
  buffer: Vec<Record>,
  right_iter: Option<DataIterator>,
  join_key: Vec<JoinKeyInfo>,
}

impl JoinManager {
  pub fn new(join_key: Vec<JoinKeyInfo>) -> Self {
    Self {
      output_size: 1000,
      buffer: vec![],
      right_iter: None,
      join_key,
    }
  }
  /// Join two table and return fixed size
  /// This function will return an error if need to change next block.
  /// left is normal table, and right is secure table !!!
  pub fn join_exec(&mut self, left: &ObliData, output: &ObliData) -> Result<(), &'static str> {
    let mut left_iter = DataIterator::new(left);
    let mut ret: Vec<Record> = Vec::with_capacity(self.output_size);

    fn merge_join_scan(jm: &mut JoinManager, left_record: &Record) -> Result<Vec<Record>, String> {
      let mut ans: Vec<Record> = vec![];
      loop {
        if let Some(right_iter) = &mut jm.right_iter {
          match right_iter.next() {
            Some(right_record) => {
              // @todo now only support one join key
              let join_key = jm.join_key[0].position as usize;
              if right_record.items[join_key] == left_record.items[join_key] {
                ans.push(right_record.union(left_record));
              }
            }
            None => {
              // right iterator is empty, return error
              // change new right block or end join operation
              return Err(String::from("empty right iterator"));
            }
          }
        } else {
          break;
        }
      }
      Ok(ans)
    }

    loop {
      if let Some(record) = left_iter.next() {
        match merge_join_scan(self, &record) {
          Ok(mut result) => {
            while ret.len() < ret.capacity() && result.len() > 0 {
              ret.push(result.pop().unwrap());
            }
            while result.len() > 0 {
              self.buffer.push(result.pop().unwrap());
            }
            while ret.len() < ret.capacity() && self.buffer.len() > 0 {
              ret.push(self.buffer.pop().unwrap());
            }
            while ret.len() > 0 && ret.len() < ret.capacity() {
              ret.push(ret.get(0).unwrap().gen_dummy_with_same_schema());
            }
          }
          Err(_e) => {
            return Err("right iterator is empty, need new right block");
            // right block is empty, need new right block
          }
        };
      } else {
        break;
      }
    }

    let mut writer = DataWriter::new();
    ret.iter().for_each(|record| writer.write(record));
    let byt_buf = writer.finish();

    let mut dm = DATA_MANAGER.exclusive_access();
    // push fbs buf_result to output data
    if let Some(data) = dm.get_data_mut(&output.id) {
      byt_buf
        .iter()
        .for_each(|item| data.buffer.lock().unwrap().push(*item));
    } else {
      return Err("[operator::hasher::sort_exec()] output data don't exist");
    }
    drop(dm);

    Ok(())
  }

  pub fn registry_right_block(&mut self, right: &ObliData) -> Result<(), &'static str> {
    Ok(())
  }
}
