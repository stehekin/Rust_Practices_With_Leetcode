use std::{collections::HashMap};
use rand::prelude::*;

struct Solution {
  map: HashMap<i32, Vec<usize>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
      let mut result = Solution {
        map: HashMap::new(),
      };

      for (i, v) in nums.iter().enumerate() {
        let lst = result.map.entry(*v).or_insert(Vec::new());
        lst.push(i);
      }

      result
    }

    fn pick(&self, target: i32) -> i32 {
      let l = self.map.get(&target).unwrap();
      let mut rng = rand::thread_rng();
      let v = rng.gen_range(0..l.len()) as usize;
      l[v] as i32
    }
}

fn main() {
    println!("Hello, world!");
}
