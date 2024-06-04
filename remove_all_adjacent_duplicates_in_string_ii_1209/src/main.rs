struct Solution {}

impl Solution {
  pub fn remove_duplicates(mut s: String, k: i32) -> String {
    let mut stack_count = Vec::<usize>::new();

    let stack_value = unsafe {s.as_bytes_mut()};
    // Top of the value stack.
    let mut i = usize::MAX;

    for j in 0..stack_value.len() {
      stack_value[i + 1] = stack_value[j];
      i += 1;

      if stack_count.is_empty() {
        stack_count.push(1);
      } else {
        let last = stack_count.last_mut().unwrap();
        if stack_value[i - 1] == stack_value[j] {
          *last += 1;
          if *last == k as usize {
            i -= k as usize;
            stack_count.pop();
          }
        } else {
          stack_count.push(1);
        }
      }
    }

    String::from_utf8_lossy(&stack_value[0..i + 1]).into()
  }
}

fn main() {
    println!("Hello, world!");
}
