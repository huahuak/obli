use std::{cell::RefCell, collections::HashMap, rc::Rc};

use serde::{de::value::BoolDeserializer, Deserialize, Serialize};

use crate::Command;

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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ObliData {
  pub name: String,
  pub id: String,
  pub addr: i64,
  pub length: i64,
  pub prepared: bool,
  pub in_use: i32,
}

impl ObliData {
  pub fn increase_in_use(&mut self) {
    self.in_use += 1;
  }

  pub fn decrease_in_use(&mut self) -> Result<(), &'static str> {
    self.in_use -= 1;
    if self.in_use < 0 {
      return Err("[proto::context.rs::decrease_in_use()] in_use is smaller than 0, it is error!");
    }
    Ok(())
  }
}

impl ObliData {
  pub fn empty_with_uuid(uuid: &String) -> ObliData {
    ObliData {
      name: String::new(),
      id: String::from(uuid),
      addr: 0,
      length: 0,
      prepared: false,
      in_use: 0,
    }
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ExprType {
  MOD,
  HASH,
  SORT,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash)]
pub enum ExtraExprInfo {
  SortOrder,
  ModNumber,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Expression {
  pub id: String,
  pub typ: ExprType,
  // @audit here is bug, input and output is copy because of serialization, need ref instead of copy
  pub input: Rc<RefCell<ObliData>>,
  pub output: Rc<RefCell<ObliData>>,
  pub children: Vec<Expression>,
  pub info: HashMap<ExtraExprInfo, String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Context {
  pub expressions: Vec<Expression>,
}

// ------------------ only use in client and ta ------------------ //
#[derive(Deserialize, Serialize, Debug)]
pub struct TaCallerInfo {
  pub method: Command,
  pub info: String,
  pub ok: bool,
}
