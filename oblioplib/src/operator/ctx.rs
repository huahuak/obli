use std::{
  any::Any,
  collections::{hash_map, HashMap},
  string,
};

use lazy_static::lazy_static;
use proto::{protocol::context::Context, sync::UPSafeCell};

lazy_static! {
  pub static ref CTX_MANAGER: UPSafeCell<CtxManager> =
    unsafe { UPSafeCell::new(CtxManager::new()) };
}

pub struct CtxManager {
  ctxs: Vec<Ctx>,
}

impl CtxManager {
  fn new() -> CtxManager {
    CtxManager { ctxs: vec![] }
  }
}

pub struct Ctx {
  id: String,
  pub context: Context,
  pub member: Box<dyn ContinuousCompute>,
  pub stash_values: HashMap<&'static str, Box<dyn Any>>,
}

impl Ctx {
  fn new(context: Context, member: Box<dyn ContinuousCompute>) -> Ctx {
    Ctx {
      id: "1".to_string(),
      context,
      member,
      stash_values: HashMap::new(),
    }
  }

  fn contine_computing(&mut self) {
    // only support context which consist of one operator for now
    // @todo in future, need to choose operator in context, and recover operator compute process 
    self.member.as_mut().contine_compute(&mut self.stash_values);
  }
}

pub trait ContinuousCompute {
  fn contine_compute(&mut self, vals: &mut HashMap<&'static str, Box<dyn Any>>);
}
