struct Solution {}

impl Solution {
  fn help(nums: &Vec<i32>, target: i32, first: bool) -> usize {
    let mut l = 0 as usize;
    let mut r = nums.len() - 1;

    while l < nums.len() && r < nums.len() && l < r {
      print!("{l}, {r} {first}\n");

      let mid = (r + l) / 2;
      if nums[mid] == target {
        if first {
          r = mid;
        } else {
          if nums[mid + 1] == target {
            l = mid + 1
          } else {
            return mid
          }
        }
      } else if nums[mid] > target {
        r = mid - 1
      } else {
        l = mid + 1
      }
    }

    if r == l && nums[r] == target {
      return r;
    }

    return usize::MAX
  }

  pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let l = Self::help(&nums, target, true);
    let r = Self::help(&nums, target, false);
    vec![l as i32, r as i32]
  }
}

fn main() {
    let nums = vec!(5, 7, 7, 8, 8, 10);
    print!("{:?}", Solution::search_range(nums, 8));
}
