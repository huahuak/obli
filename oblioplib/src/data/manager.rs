use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
use proto::{
  collection::vector_generated::org::kaihua::obliop::collection::fbs::RowTable, context::ObliData,
  sync::UPSafeCell,
};

pub struct Data {
  pub obli_data: Arc<Mutex<ObliData>>,
  buffer: Arc<Vec<u8>>,
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

  pub fn insert(&mut self, key: &str, data: Arc<Mutex<ObliData>>, buffer: &[u8]) {
    let mut k = String::new();
    k.push_str(key);
    self.map.insert(
      k,
      Data {
        obli_data: data,
        buffer: Arc::new(buffer.to_vec()),
      },
    );
  }

  pub fn get_data_buffer_by_key(&self, key: &str) -> Arc<Vec<u8>> {
    Arc::clone(&self.map.get(key).unwrap().buffer)
  }

  pub fn get_map(&self) -> &HashMap<String, Data> {
    &self.map
  }
}

lazy_static! {
  pub static ref DATA_MANAGER: UPSafeCell<DataManager> =
    unsafe { UPSafeCell::new(DataManager::new()) };
}

pub fn data_handler(data: Arc<Mutex<ObliData>>, buffer: &[u8]) {
  // trace_println!("[data::mod.rs] enter fn data_handler");
  let key = String::from(&data.lock().unwrap().id);
  DATA_MANAGER
    .exclusive_access()
    .insert(key.as_str(), data, buffer);
}
