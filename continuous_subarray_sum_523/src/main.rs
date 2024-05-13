use std::collections::HashMap;

struct Solution {

}

impl Solution {
  pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let mut map = HashMap::from([(0, -1)]);
    let mut sum = 0;

    for (index, value) in nums.iter().enumerate() {
      sum += value;
      if k != 0 {
        sum = sum % k
      }

      if map.contains_key(&sum) {
        if index as i32 - map[&sum] > 1 {
          return true
        }
      } else {
        map.insert(sum, index as i32);
      }
    }

    false
  }
}


fn main() {
    println!("Hello, world!");
}
