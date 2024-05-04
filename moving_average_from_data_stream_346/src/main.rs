use std::collections::VecDeque;

struct MovingAverage {
  buffer: VecDeque<i32>,
  sum: i32,
  size: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {
    fn new(size: i32) -> Self {
      MovingAverage {
        buffer: VecDeque::with_capacity(size as usize),
        sum: 0,
        size: size as usize,
      }
    }

    fn next(&mut self, val: i32) -> f64 {
      if self.buffer.len() == self.size {
        if let Some(v) = self.buffer.pop_front() {
          self.sum -= v;
        }
      }

      self.sum += val;
      self.buffer.push_back(val);
      return self.sum as f64 / self.buffer.len() as f64
    }
}

fn main() {
    println!("Hello, world!");
}
