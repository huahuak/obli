use std::sync::Arc;

use proto::context::{
  Context,
  ExprType::{HASH, MOD, SORT},
  Expression, ObliData,
};

use crate::data::manager::DATA_MANAGER;

use super::hasher::hash_exec;

/**
 * @author kahua.li
 * @email moflowerlkh@gmail.com
 * @date 2023/02/04
 **/

pub fn execute(expr: &Expression) -> Result<(), &'static str> {
  for child in &expr.children {
    execute(&child)?;
  }
  let mut dm = DATA_MANAGER.exclusive_access();
  let input = Arc::clone(
    &dm
      .get_data(&expr.input.as_ref().borrow().id)
      .unwrap()
      .description,
  );
  let output = Arc::clone(
    &dm
      .get_data_mut(&expr.output.as_ref().borrow().id)
      .unwrap()
      .description,
  );
  drop(dm);
  match expr.typ {
    // MOD => {}
    HASH => {
      hash_exec(&input.lock().unwrap(), &output.lock().unwrap())?;
    }
    // SORT => {}
    _ => {
      // panic!(
      //   "[executor.rs::execute()] expr typ of {:#?} is unsupported !!!",
      //   expr.typ
      // );
      return Err("[executor.rs::execute()] expr typ of {:#?} is unsupported !!!");
    }
  }
  Ok(())
}

fn prepare_data(expr: &Expression) -> Result<(), &'static str> {
  let mut dm = DATA_MANAGER.exclusive_access();
  let input = expr.input.as_ref().borrow();
  match dm.get_data_mut(&input.id) {
    Some(target) => target.description.lock().unwrap().in_use += 1,
    None => dm.insert(&input.id, &input, &[])?,
  }
  let output = expr.output.as_ref().borrow();
  dm.insert(&output.id, &output, &[])?;
  drop(dm);
  // for child must be exec after insert output operation
  for child in &expr.children {
    prepare_data(child)?;
  }
  Ok(())
}

pub fn obli_op_ctx_exec(ctx: &mut Context) -> Result<(), &'static str> {
  for expr in &ctx.expressions {
    prepare_data(expr)?;
    execute(expr)?;
  }
  Ok(())
}
