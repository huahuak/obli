pub fn sprint_byte(byt: &[u8]) -> String {
  let mut s = String::new();
  unsafe {
    s.push_str(&format!(
      "// ------------------ print byte ------------------ //\n"
    ));
    for i in byt {
      s.push_str(&format!("{:?} ", i));
    }
    s.push('\n');
    for i in byt {
      s.push_str(&format!("{}", char::from(*i)));
    }
    s.push('\n');
    s.push_str(&format!("// ------------------ print byte ------------------ //\n"));
  }
  s
}
