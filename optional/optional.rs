// Use of Optional and pattern matching in Rust
fn main() {
  let some_value = Some(0u8);

  match some_value {
    Some(0) => println!("the value is zero"),
    _ => (), // Every other cause just return the value
  }

  if let Some(0) = some_value {
    println!("The value is zero again");
  }
}
