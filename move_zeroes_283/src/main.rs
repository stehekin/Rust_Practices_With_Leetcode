
pub fn move_zeroes(nums: &mut Vec<i32>) {
  let mut i = 0;
  while i < nums.len() && nums[i] != 0 {
    i += 1;
  }

  for j in i + 1..nums.len() {
    if nums[j] != 0 {
      nums.swap(i, j);
      i += 1;
    }
  }
}

fn main() {
    println!("Hello, world!");
}
