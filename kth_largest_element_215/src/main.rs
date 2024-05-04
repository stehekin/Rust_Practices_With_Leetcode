pub fn help(mut nums: Vec<i32>, i: usize, j: usize, k: usize) -> i32 {
  let mut l = i + 1;
  let mut r = j;
  let pivot = nums[i];

  while l <= r {
    if nums[l] <= pivot {
      nums.swap(l, r);
      r -= 1;
    } else {
      l += 1;
    }
  }

  nums.swap(i, r);
  let count = r - i + 1;

  if count == k {
    return nums[r];
  }

  if count > k {
    return help(nums, i, r - 1, k)
  }

  help(nums, r + 1, j, k - count)
}

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
  let j = nums.len();
  help(nums, 0, j, k as usize)
}

fn main() {
    println!("Hello, world!");
}
