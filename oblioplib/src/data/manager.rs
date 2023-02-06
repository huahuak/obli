use std::{
  collections::HashMap,
  option,
  sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
use proto::{context::ObliData, sync::UPSafeCell};

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
  pub fn insert(&mut self, key: &str, data: &ObliData, buffer: &[u8]) {
    assert!(
      self.get_data_mut(key).is_none(),
      "[data::manager::inser()] DATA_MANAGER already had the data with key '{}'",
      key
    );

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

pub fn data_handler(data: &ObliData, buffer: &[u8]) {
  // trace_println!("[data::mod.rs] enter fn data_handler");
  DATA_MANAGER
    .exclusive_access()
    .insert(&data.id, data, buffer);
}
