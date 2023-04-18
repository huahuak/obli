use core::str;
use std::{any::Any, collections::HashMap, vec};

use proto::protocol::context::{JoinKeyInfo, ObliData};

use crate::data::{
  manager::{Record, DATA_MANAGER},
  reader::DataIterator,
  writer::DataWriter,
};

use super::ctx::ContinuousCompute;

const KEY: &str = "testkey";

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

  stash_map: HashMap<&'static str, Box<dyn Any>>, // stash some value in context
}

impl ContinuousCompute for JoinManager {
  fn contine_compute(&mut self, vals: &mut HashMap<&'static str, Box<dyn Any>>) {
    // @todo handle the error.
    vals.get(KEY).unwrap();
  }
}

impl JoinManager {
  pub fn new(join_key: Vec<JoinKeyInfo>, stash_map: HashMap<&'static str, Box<dyn Any>>) -> Self {
    Self {
      output_size: 5,
      buffer: vec![],
      right_iter: None,
      join_key,
      stash_map,
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
      if let Some(mut right_iter) = jm.right_iter.clone() {
        loop {
          match right_iter.next() {
            Some(right_record) => {
              // @todo now only support one join key
              let lpos = jm.join_key[0].lpos as usize;
              let rpos = jm.join_key[0].rpos as usize;
              let cmp = left_record.items[lpos]
                .partial_cmp(&right_record.items[rpos])
                .unwrap();
              match cmp {
                std::cmp::Ordering::Equal => {
                  ans.push(left_record.union(&right_record));
                }
                std::cmp::Ordering::Less => {
                  jm.right_iter.as_mut().unwrap().cur = right_iter.cur - 1;
                  return Ok(ans);
                }
                std::cmp::Ordering::Greater => {
                  continue;
                }
              }
            }
            None => {
              // right iterator is empty, return error
              // change new right block or end join operation
              jm.stash_map.insert(KEY, Box::new("test ok"));
              break;
              // @todo here need to get new right block from spark
              // return Err(String::from("empty right iterator"));
            }
          }
        }
      };

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
          }
          Err(_e) => {
            return Err("right iterator is empty, need new right block");
            // right block is empty, need new right block
          }
        };
      } else {
        while ret.len() < ret.capacity() && self.buffer.len() > 0 {
          ret.push(self.buffer.pop().unwrap());
        }
        while ret.len() > 0 && ret.len() < ret.capacity() {
          ret.push(ret.get(0).unwrap().gen_dummy_with_same_schema());
        }
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
    self.right_iter = Some(DataIterator::new(right));
    Ok(())
  }
}
