use std::{cmp, vec};

pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  intervals.sort_by(|a, b| a[0].cmp(&b[0]));
  let mut result = Vec::<Vec<i32>>::new();

  for i in intervals {
    if let Some(last) = result.last_mut() {
      if last[1] >= i[0] {
        last[1] = cmp::max(last[1], i[1]);
        continue;
      }
    }
    result.push(i);
  }
  result
}

fn main() {
    println!("Hello, world!");
}
