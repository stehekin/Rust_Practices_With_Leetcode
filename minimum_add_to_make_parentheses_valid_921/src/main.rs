pub fn min_add_to_make_valid(s: String) -> i32 {
  let mut r = 0;
  let mut b = 0;
  for c in s.chars() {
    match (c) {
      '(' => {
        b += 1;
      }
      ')' => {
        b -= 1;
        if b < 0 {
          b = 0;
          r += 1;
        }
      }
      _ => {}
    }
  }

  r + b
}

fn main() {
    println!("Hello, world!");
}
