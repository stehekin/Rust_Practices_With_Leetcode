use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

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

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
  let mut result = vec![];
  if root.is_none() {
    return result
  }

  let root = root.unwrap();

  let mut queue = VecDeque::from(vec![root.clone()]);

  while !queue.is_empty() {
    let size = queue.len();
    for i in 0..size {
      let node = queue.pop_front().unwrap().clone();
      let node = node.borrow();
      if let Some(left) = &node.left {
        queue.push_back(left.clone());
      }
      if let Some(right) = &node.right {
        queue.push_back(right.clone());
      }
      if i == size - 1 {
        result.push(node.val);
      }
    }
  }

  result
}

fn main() {
    println!("Hello, world!");
}
