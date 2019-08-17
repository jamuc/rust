fn main() {
  let s1 = String::from("hello"); // String object where the value is stored on the heap

  let length = calculate_length(&s1);
  println!("The string {} has a length of {}", s1, length);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}
