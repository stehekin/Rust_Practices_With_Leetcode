use std::cmp::{max, min};

struct Solution {

}

impl Solution {
  fn overlap(i1: &Vec<i32>, i2: &Vec<i32>) -> Vec<i32> {
    vec![max(i1[0], i2[0]), min(i1[1], i2[1])]
  }

  pub fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let (mut l1, mut l2) = (first_list.iter(), second_list.iter());
    let (mut i1, mut i2) = (l1.next(), l2.next());

    loop {
      match (i1, i2) {
        (Some(i1v), Some(i2v)) => {
          let overlap = Self::overlap(i1v, i2v);
          if overlap[0] <= overlap[1] {
            result.push(overlap)
          }

          if i1v[1] < i2v[1] {
            i1 = l1.next()
          } else {
            i2 = l2.next()
          }
        },
        _ => break,
      }
    }

    result
  }
}

fn main() {
    println!("Hello, world!");
}
