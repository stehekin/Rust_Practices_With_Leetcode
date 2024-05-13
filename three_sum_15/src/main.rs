

struct Solution {

}

impl Solution {
  pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    nums.sort();

    for (index, num) in nums.iter().enumerate() {
      if index > 0 && *num == nums[index - 1 ] {
        continue;
      }

      let mut i = index + 1;
      let mut j = nums.len() - 1;

      while i < j {
        let sum = nums[i] + nums[j] + num;
        if sum == 0 {
          result.push(vec![*num, nums[i], nums[j]]);
          loop {
            i += 1;
            if i >= nums.len() || nums[i] != nums[i - 1] {
              break;
            }
          }
        } else if sum > 0 {
          j -= 1;
        } else {
          i += 1;
        }
      }
    }

    result
  }
}

fn main() {
    println!("Hello, world!");
}
