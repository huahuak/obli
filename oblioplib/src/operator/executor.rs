use proto::{
  collection::vector_generated::org::kaihua::obliop::collection::fbs::RowTable,
  context::{
    Context,
    ExprType::{HASH, MOD, SORT},
    Expression,
  },
};

/**
 * @author kahua.li
 * @email moflowerlkh@gmail.com
 * @date 2023/02/04
 **/

pub fn execute(expr: &Expression) {
  for child in &expr.children {
    execute(&child);
  }
  match expr.typ {
    MOD => {}
    HASH => {}
    SORT => {}
    _ => {
      panic!(
        "[executor.rs::execute()] expr typ of {:#?} is unsupported !!!",
        expr.typ
      );
    }
  }
}

pub fn obli_op_ctx_exec(ctx: &mut Context) {
  // trace_println!("[operator::mod.rs] enter fn obli_ob_ctx_exec");
  // trace_println!("[operator::mod.rs] ctx is {:#?}", ctx);

  for expr in &ctx.expressions {}

  // let dm = data::DATA_MANAGER.exclusive_access();
  // let buf = dm.get_data_buffer_by_key(dm.get_map().keys().next().unwrap());
  // let v = flatbuffers::root::<RowTable>(buf).unwrap();
  // unsafe {
  //   trace_println!(
  //     "[TA::operator::mod.rs] v is '{}'",
  //     v.rows()
  //       .unwrap()
  //       .get(0)
  //       .fields()
  //       .unwrap()
  //       .get(0)
  //       .value_as_string_value()
  //       .unwrap()
  //       .value()
  //       .unwrap()
  //   );
  // }
  // drop(dm);
}
