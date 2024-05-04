use std::cmp;

fn ge(a: &Vec<i32>, b: &Vec<i32>) -> bool {
  a[0] * a[0] + a[1] * a[1] >= b[0] * b[0] + b[1] * b[1]
}

fn topK(data: &mut Vec<Vec<i32>>, l: usize, r: usize, k: usize) {
  if r - l + 1 <= k {
    return;
  }

  let mut l = l;
  let mut r = r;
  let mut k = k;

  loop {
    let mut i = l + 1;
    let mut j = r;

    while i <= j {
      if ge(&data[i], &data[l]) {
        data.swap(i, j);
        j = j - 1;
      } else {
        i = i + 1;
      }
    }

    data.swap(l, j);
    let count = j - l + 1;
    if count == k {
      break
    } else if count < k {
      l = j + 1;
      k = k - count;
    } else {
      r = j - 1;
    }
  }
}


pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
  let len = points.len();
  topK(&mut points, 0, len - 1, k as usize);
  points[0..k as usize].to_vec()
}


fn main() {
    println!("Hello, world!");
}
