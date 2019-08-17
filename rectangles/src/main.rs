#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle { width: size, height: size }
  }

  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

fn main() {
  let rectangle = Rectangle { width: 20, height: 50 };
  let rect2 = Rectangle { width: 10, height: 40 };
  let rect3 = Rectangle { width: 25, height: 25 };
  let square = Rectangle::square(20);

  println!("The area of the rectangle {:#?} is {}",
           rectangle,
           rectangle.area());
  println!("Can rectangle hold rect2? {}", rectangle.can_hold(&rect2));
  println!("Can rectangle hold rect3? {}", rectangle.can_hold(&rect3));
  println!("The area of the square is {}", square.area());
}

