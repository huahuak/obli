use std::{
  borrow::Borrow,
  collections::HashMap,
  result,
  sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
use proto::{context::ObliData, sync::UPSafeCell, util};

use crate::logger::{Logger, LOGGER};

#[derive(Debug)]
pub struct Data {
  pub description: Arc<Mutex<ObliData>>,
  pub buffer: Arc<Mutex<Vec<u8>>>,
}

pub struct DataManager {
  map: HashMap<String, Data>,
}

impl DataManager {
  pub fn new() -> DataManager {
    DataManager {
      map: HashMap::new(),
    }
  }

  /// insert will clone data
  pub fn insert(&mut self, key: &str, data: &ObliData, buffer: &[u8]) -> Result<(), &'static str> {
    // assert!(
    //   self.get_data_mut(key).is_none(),
    //   "[data::manager::inser()] DATA_MANAGER already had the data with key '{}'",
    //   key
    // );

    // prevent the data of same key
    if !self.get_data_mut(key).is_none() {
      return Err("[data::manager::inser()] DATA_MANAGER already had the data");
    }

    let key = String::from(key);
    let mut data = data.clone();
    data.prepared = true;
    self.map.insert(
      key,
      Data {
        description: Arc::new(Mutex::new(data)),
        buffer: Arc::new(Mutex::new(buffer.to_vec())),
      },
    );
    Ok(())
  }

  pub fn get_data_mut(&mut self, key: &str) -> Option<&mut Data> {
    self.map.get_mut(key)
  }

  pub fn get_data(&self, key: &str) -> Option<&Data> {
    self.map.get(key)
  }
}

lazy_static! {
  pub static ref DATA_MANAGER: UPSafeCell<DataManager> =
    unsafe { UPSafeCell::new(DataManager::new()) };
}

pub fn push_data_handler(data: &ObliData, buffer: &[u8]) -> Result<(), &'static str> {
  // trace_println!("[data::mod.rs] enter fn data_handler");
  DATA_MANAGER
    .exclusive_access()
    .insert(&data.id, data, buffer)?;
  Ok(())
}

pub fn get_data_handle(target: &ObliData, output: &mut [u8]) -> Result<(), &'static str> {
  let dm = DATA_MANAGER.exclusive_access();
  let buf = &dm.get_data(&target.id).unwrap().buffer;
  let buf_len = buf.lock().unwrap().len();
  output[..buf_len].copy_from_slice(&buf.lock().unwrap());

  LOGGER.lock().unwrap().log(format!("len is {}", buf_len));
  LOGGER
    .lock()
    .unwrap()
    .log(proto::collection::fbs_vec::FbsRowTable::sprint_row_table_fbs(&buf.lock().unwrap()));
  LOGGER
    .lock()
    .unwrap()
    .log(util::sprint_byte(&buf.lock().unwrap()));

  drop(dm);
  Ok(())
}
