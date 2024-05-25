struct Solution {}

impl Solution {
  pub fn count_substrings(s: String) -> i32 {
    let size = s.len();
    let mut mem = vec![vec![false; size]; size];
    for i in 0..size {
      mem[i][i] = true;
    }

    let mut count = size;
    let bytes = s.as_bytes();

    for sz in 2..=size {
      for i in 0..size + 1 - sz {
        let j = i + sz - 1;
        if bytes[i] != bytes[j] {
          continue;
        }

        mem[i][j] = sz == 2 || mem[i+1][j-1];
        count += if mem[i][j] {1} else {0};
      }
    }
    count as i32
  }
}

fn main() {
    println!("Hello, world!");
}
