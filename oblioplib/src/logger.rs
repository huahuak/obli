use std::sync::Mutex;

use lazy_static::lazy_static;

pub struct Logger {
  log_buf: Vec<String>,
}

lazy_static! {
  pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
}

impl Logger {
  fn new() -> Logger {
    Logger { log_buf: vec![] }
  }

  pub fn log(&mut self, info: String) {
    self.log_buf.push(info);
  }

  pub fn next(&mut self) -> Option<String> {
    if self.log_buf.len() > 0 {
      Some(self.log_buf.swap_remove(0))
    } else {
      None
    }
  }
}
