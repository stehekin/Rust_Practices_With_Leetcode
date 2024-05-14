struct Solution {

}

impl Solution {
  fn isAlpha(v :u8) -> bool {
    (v >= b'a' && v <= b'z') || (v >= b'0' && v <= b'9')
  }

  fn toLower(v:u8) -> u8 {
    if v >= b'A' && v <= b'Z' {
      return v - b'A' + b'a';
    }

    v
  }

  pub fn is_palindrome(s: String) -> bool {
    let (mut l, mut r) = (0 as usize, s.len() - 1);

    while l < r {
      let lb = Self::toLower(s.as_bytes()[l]);
      let rb = Self::toLower(s.as_bytes()[r]);

      match (Self::isAlpha(lb), Self::isAlpha(rb)) {
        (true, true) => {
          if lb != rb {
            return false
          }
          l += 1;
          r -= 1;
        }
        (true, false) => {
          r -= 1;
        }
        (false, true) => {
          l += 1;
        }
        (false, false) => {
          l += 1;
          r -= 1;
        }
      }
    }

    true
  }
}

fn main() {
    println!("Hello, world!");
}
