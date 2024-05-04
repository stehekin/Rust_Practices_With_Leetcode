struct Solution {
  sum: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(w: Vec<i32>) -> Self {
      let mut sum = vec![0; w.len()];
      sum[0] = w[0];
      for (i, v) in w.iter().skip(1).enumerate() {
        sum[i] = sum[i - 1] + v;
      }

      Solution {sum}
    }

    fn pick_index(&self) -> i32 {
      todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
