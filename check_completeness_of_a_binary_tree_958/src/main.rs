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
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
      if root.is_none() {
        return true
      }
      let mut deque = VecDeque::from([root]);

      while !deque.is_empty() {
        let size = deque.len();
        let mut sawNone = false;
        for _ in 0..size {
          let node = deque.pop_front().unwrap();
          if let Some(node) = node {
            if sawNone {
              return false;
            }
            let node = node.borrow();
            deque.push_back(node.left.clone());
            deque.push_back(node.right.clone());
          } else {
            sawNone = true;
          }
        }
        if sawNone && !deque.is_empty() {
          return false;
        }
      }
      true
    }
}


fn main() {
    println!("Hello, world!");
}
