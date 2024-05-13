struct Solution {

}

impl Solution {
  pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut j = (m + n - 1) as usize;
    let (mut m, mut n) = (m, n);
    while m >= 0 && n >= 0 {
      if nums1[m as usize] >= nums2[n as usize] {
        nums1[j] = nums1[m as usize];
        m -= 1;
      } else {
        nums1[j] = nums1[n as usize];
        n -= 1;
      }
      j -= 1;
    }

    while n >= 0 {
      nums1[j] = nums2[n as usize];
      n -= 1;
      j -= 1;
    }
  }
}

fn main() {
    println!("Hello, world!");
}
