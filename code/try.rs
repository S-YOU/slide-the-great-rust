fn may_fail(in: i32) -> Result<i32, Error> {
  let mut buffer = String::new();
  io::stdin().read_to_string(&mut buffer)?;
  let parsed = buffer.parse()?;
  Ok(parsed + in)
}
