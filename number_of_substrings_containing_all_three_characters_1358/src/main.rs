use std::cmp::min;

struct Solution {}

impl Solution {
  pub fn number_of_substrings(str: String) -> i32 {
    let mut pos = [-1; 3];
    let mut result = 0;

    for (i, b) in str.as_bytes().iter().enumerate() {
      pos[(*b - b'a') as usize] = i as i32;
      result += min(min(pos[0], pos[1]), pos[2]);
    }

    result
  }
}

fn main() {
    println!("Hello, world!");
}
