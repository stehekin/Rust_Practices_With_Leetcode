use std::rc::Rc;
use std::cell::RefCell;

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

fn sum(node: &Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32, result: &mut i32) {
  if let Some(node) = node {
    let val = node.borrow().val;
    if val <= high && val >= low {
      *result += val;
      sum(&node.borrow().left, low, high, result);
      sum(&node.borrow().right, low, high, result)
    } else if val < low {
      sum(&node.borrow().right, low, high, result);
    } else {
      sum(&node.borrow().left, low, high, result);
    }
  }
}

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
  let mut result = 0;
  sum(&root, low, high, &mut result);
  result
}

fn main() {
    println!("Hello, world!");
}
