/*
* Variables are immutable by default. However you can use let
* statement to "shadow" variables in Rust.
*/
fn main() {
  let x = 5;
  let x = x + 1;
  let x = x * 5;
  
  println!("This is variable shadowing, x is: {}", x);
}
