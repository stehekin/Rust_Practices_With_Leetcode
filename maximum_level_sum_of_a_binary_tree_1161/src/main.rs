// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution {}

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
      if root.is_none() {
        return 0
      }

      let root = root.unwrap();
      let mut queue = VecDeque::from([root]);
      let mut result = 0;
      let mut max_sum = i32::MIN;
      let mut level = 0;

      while queue.len() != 0 {
        let len = queue.len();
        let mut sum = 0;
        level += 1;
        for _ in 0..len {
          let node = queue.pop_front().unwrap();
          let node = node.borrow();
          sum += node.val;

          if let Some(left) = &node.left {
            queue.push_back(left.clone());
          }

          if let Some(right) = &node.right {
            queue.push_back(right.clone());
          }
        }

        if sum > max_sum {
          result = level;
          max_sum = sum;
        }
      }

      result
    }
}

fn main() {
    println!("Hello, world!");
}
