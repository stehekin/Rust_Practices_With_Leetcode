pub fn help(s: &[u8], mut i: usize, mut j: usize) -> bool {
  while i < j {
    if s[i] != s[j] {
      return false
    }
    i += 1;
    j -= 1;
  }
  true
}

pub fn valid_palindrome(s: String) -> bool {
  let s = s.as_bytes();
  let mut i = 0 as usize;
  let mut j = s.len() - 1;

  while i < j {
    if s[i] != s[j] {
      return help(s, i + 1, j) || help(s, i, j - 1);
    }
    i += 1;
    j -= 1;
  }

  true
}

fn main() {
    println!("Hello, world!");
}
