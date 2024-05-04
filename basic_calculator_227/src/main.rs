pub fn calculate(s: String) -> i32 {
  let s = s.as_bytes();
  let mut first = 0;
  let mut second = 0;

  let mut i = 0 as usize;
  let mut op = b'+';

  while i < s.len() {
      if s[i] >= b'0' && s[i] <= b'9' {
          let mut num = 0;
          while i < s.len() && s[i] >= b'0' && s[i] <= b'9' {
              num = num * 10 + (s[i] - b'0') as i32;
              i += 1;
      }

      match op {
          b'*' => {
              second = second * num;
          }
          b'/' => {
              second = second / num;
          }
          b'+' => {
              (first, second) = (first + second, num);
          }
          b'-' => {
              (first, second) = (first + second, -num);
          }
          _ => {}
      }
      } else {
      if s[i] != b' ' {
          op = s[i]
      }
      i += 1;
      }
  }

  first + second
}

fn main() {
    println!("Hello, world!");
}
