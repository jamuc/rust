//An example on how to use enums and pattern matching in Rust
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

impl Coin {
  fn to_cents(&self) -> u8 {
    match self {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter => 25,
    }
  }
}

fn main() {
  let coin = Coin::Nickel;
  println!("Coin in cents {}", coin.to_cents());
}
