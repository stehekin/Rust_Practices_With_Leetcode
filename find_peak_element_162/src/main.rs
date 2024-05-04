pub fn find_peak_element(nums: Vec<i32>) -> i32 {
  let mut i = 0 as usize;
  let mut j = nums.len() - 1;

  while i < j {
    let mid = (i + j) / 2;
    if nums[mid] < nums[mid + 1] {
      i = mid + 1;
    } else {
      j = mid;
    }
  }

  i as i32
}

fn main() {
    println!("Hello, world!");
}
