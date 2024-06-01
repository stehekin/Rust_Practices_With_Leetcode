use std::cmp::max;

struct Solution {}

impl Solution {
  pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
    let mut i = 0 as usize;
    let mut j = 1 as usize;
    let mut result = 1 as usize;

    while j < nums.len() {
      if nums[j] <= nums[j - 1] {
        i = j;
      }
      j += 1;
      result = max(result, j - i);
    }
    result as i32
  }
}