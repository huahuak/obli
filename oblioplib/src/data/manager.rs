use std::{
  borrow::Borrow,
  collections::HashMap,
  result,
  sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
use proto::{
  context::{ObliData, TaCallerInfo},
  sync::UPSafeCell,
  util,
};

use crate::logger::{self, Logger, LOGGER};

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

pub fn get_data_handle(
  target: &ObliData,
  info: &mut [u8],
  output: &mut [u8],
) -> Result<(), &'static str> {
  let dm = DATA_MANAGER.exclusive_access();
  // let buf = &dm.get_data(&target.id).unwrap().buffer;
  let mut v = TaCallerInfo {
    method: proto::Command::DataGet,
    info: String::new(),
    ok: false,
  };

  if let Some(data) = dm.get_data(&target.id) {
    let buf = &data.buffer;
    let buf_len = buf.lock().unwrap().len();
    output[..buf_len].copy_from_slice(&buf.lock().unwrap());
    v.info.insert_str(0, "data ready".to_string().as_ref());
    v.ok = true;
  } else {
    LOGGER.lock().unwrap().log(String::from(
      "[ta::manager.rs::get_data_handle()] data not ready",
    ));
    v.info.insert_str(0, "data not ready".to_string().as_ref());
    v.ok = false;
  }

  let info_json = serde_json::to_string(&v).unwrap();
  let info_json_byte = info_json.as_bytes();
  info[..info_json_byte.len()].copy_from_slice(info_json_byte);
  drop(dm);
  Ok(())
}
