use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

/**
 * @author kahua.li
 * @email moflowerlkh@gmail.com
 * @date 2023/02/04
 **/

#[derive(Deserialize, Serialize, Debug)]
pub struct RetObj {
  // typ: ObliCmdTyp,
  pub obli_op_id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ObliData {
  pub name: String,
  pub id: String,
  pub addr: i64,
  pub length: i64,
  pub prepared: bool,
  pub in_use: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ExprType {
  MOD,
  HASH,
  SORT,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Expression {
  pub id: String,
  pub typ: ExprType,
  // @audit here is bug, input and output is copy because of serialization, need ref instead of copy
  pub input: Rc<RefCell<ObliData>>,
  pub output: Rc<RefCell<ObliData>>,
  pub children: Vec<Expression>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Context {
  pub expressions: Vec<Expression>,
}
