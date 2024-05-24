use std::collections::HashSet;


struct Solution {}

impl Solution {
  pub fn total_fruit(fruits: Vec<i32>) -> i32 {

    let mut l = 0;
    let mut r = l + 1;

    let mut result = 1;
    let mut count = 1;

    let mut fset = HashSet::from([fruits[l]]);
    let mut delimiter = 0;

    while r < fruits.len() {
      fset.insert(fruits[r]);
      count += 1;

      if fset.len() > 2 {
        let t = fruits[delimiter - 1];
        fset.remove(&t);
        l = delimiter;
        count = r - l + 1;
        delimiter = r;
      } else if fruits[delimiter] != fruits[r] {
        delimiter = r;
      }
      result = std::cmp::max(count, result);
      r += 1;
    }

    result as i32
  }
}
fn main() {
    println!("Hello, world!");
}
