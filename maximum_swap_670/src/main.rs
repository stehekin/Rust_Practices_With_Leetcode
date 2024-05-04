use std::collections::HashMap;

pub fn maximum_swap(num: i32) -> i32 {
  let mut vec: Vec<i32> =
    num.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

  let mut pos = [0 as usize; 10];

  for (i, &v) in vec.iter().enumerate() {
    pos[v as usize] = i;
  }

  'outer: for (i , &v) in vec.iter().enumerate() {
    for d in (v + 1..=9).rev() {
      if pos[d as usize] > i {
        vec.swap(i, pos[d as usize]);
        break 'outer;
      }
    }
  }

  return   vec.into_iter().fold(0, |r, x| r * 10 + x)
}

fn main() {
    println!("Hello, world!");
}
