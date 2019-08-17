fn main() {
  let hello = String::from("hello world");
  let word = first_word(&hello[..]);

  println!("The first word in the string {} is {}", hello, word);
}

fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (index, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..index];
    }
  }

  &s[..]
}
