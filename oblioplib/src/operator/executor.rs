use std::sync::Arc;

use proto::protocol::context::{
  Context,
  ExprType::{HASH, SORT},
  Expression, ExtraExprInfo, SortOrderInfo,
};

use crate::data::manager::DATA_MANAGER;

use super::{
  hasher::hash_exec,
  sorter::{sort_exec},
};

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

  if !input.lock().unwrap().prepared {
    return Err("[hasher.rs::hash_exec()] input isn't prepared !!!");
  }
  input.lock().unwrap().decrease_in_use()?;

  // the expr has been executed by other tree
  if output.lock().unwrap().prepared {
    return Ok(());
  }

  match expr.typ {
    // MOD => {}
    HASH => {
      hash_exec(&input.lock().unwrap(), &output.lock().unwrap())?;
    }
    SORT => {
      let sort_order_str = expr.info.get(&ExtraExprInfo::SortOrder).unwrap();
      let ordering: Vec<SortOrderInfo> = serde_json::from_str(&sort_order_str).unwrap();
      sort_exec(&input.lock().unwrap(), &output.lock().unwrap(), ordering)?;
    }
    _ => {
      return Err("[executor.rs::execute()] expr typ of {:#?} is unsupported !!!");
    }
  }

  output.lock().unwrap().prepared = true;

  Ok(())
}

fn prepare_data(expr: &Expression) -> Result<(), &'static str> {
  for child in &expr.children {
    prepare_data(child)?;
  }
  let mut dm = DATA_MANAGER.exclusive_access();
  let input = expr.input.as_ref().borrow();
  match dm.get_data_mut(&input.id) {
    Some(target) => target.description.lock().unwrap().increase_in_use(),
    None => return Err(
      "[TA::executor.rs::prepare_data()] can't find input data, which means need data from transport interface"),
  }
  let output = expr.output.as_ref().borrow();
  match dm.get_data(&output.id) {
    None => {
      dm.insert(&output.id, &output, &vec![], false)?;
    }
    Some(_) => {}
  }
  drop(dm);
  Ok(())
}

pub fn obli_op_ctx_exec(ctx: &mut Context) -> Result<(), &'static str> {
  for expr in &ctx.expressions {
    prepare_data(expr)?;
    execute(expr)?;
  }
  Ok(())
}
