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

fn dfs(node: Rc<RefCell<TreeNode>>, target: f64, mut min: f64, value: &mut i32) {
  let v = node.borrow().val;
  let diff = (v as f64 - target).abs();
  if diff < min {
    *value = v;
    min = diff;
  }

  if diff == min && v < *value {
    *value = v;
  }

  if diff == 0f64 {
    return;
  }

  if v as f64 > target {
    if let Some(left) = node.borrow().left.clone() {
      dfs(left, target, min, value)
    }
  } else {
    if let Some(right) = node.borrow().right.clone() {
      dfs(right, target, min, value)
    }
  }
}

pub fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
  let mut value = 0;
  dfs(root.unwrap().clone(), target, f64::MAX, &mut value);
  value
}

fn main() {
    println!("Hello, world!");
}
