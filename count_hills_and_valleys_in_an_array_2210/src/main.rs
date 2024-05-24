struct Solution {}

impl Solution {
  pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut i = 1;

    while i < nums.len() - 1 {
      let mut l = i - 1;
      while l < usize::MAX && nums[l] == nums[i] {
        l -= 1;
      }

      if l == usize::MAX {
        i += 1;
        continue;
      }

      let mut r = i + 1;
      while r < nums.len() && nums[r] == nums[i] {
        r += 1;
      }

      if r >= nums.len() {
        i += 1;
        continue;
      }

      count += if (nums[l] > nums[i] && nums[r] > nums[i]) || (nums[l] < nums[i] && nums[r] < nums[i]) {1} else {0}
      i = r;
    }

    count
  }
}

fn main() {

}