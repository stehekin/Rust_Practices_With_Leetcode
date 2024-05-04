use std::collections::HashMap;

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
  let mut hash = HashMap::<i32, usize>::from([(0, 1)]);
  let mut sum = 0;
  let mut result = 0;
  for v in nums.iter() {
    sum += v;
    result += hash.get(&(sum - k)).unwrap_or(&(0 as usize));
    let v = hash.entry(sum).or_insert(0);
    *v = *v + 1;
  }
  result as i32
}

fn main() {
    println!("Hello, world!");
}
