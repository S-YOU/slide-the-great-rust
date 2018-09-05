struct BigStruct {
  data: Vec<String>
}

impl BigStruct {
  fn new() -> Self {
    let mut inst = BigStruct {
      data: Vec::new()
    };
    inst.data.push("Hello!".to_string());
    inst
  }
}
