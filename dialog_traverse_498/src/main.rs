use std::collections::HashMap;

pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
  let mut hash = HashMap::<usize, Vec<i32>>::new();
  let mut result = vec![];
  for r in 0..mat.len() {
    for c in 0..mat[r].len() {
      let vec = hash.entry(r + c).or_default();
      vec.push(mat[r][c]);
    }
  }

  for i in 0..mat.len() + mat[0].len() - 1 {
    if let Some(mut v) = hash.remove(&i) {
      if i % 2 == 0 {
        v.reverse();
      }
      result.append(&mut v);
    }
  }

  result
}

fn main() {
    println!("Hello, world!");
}
