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

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, num: i32, sum: &mut i32) {
  if let Some(node) = node {
    let num = num * 10 + node.borrow().val;
    if node.borrow().left.is_none() && node.borrow().right.is_none() {
      *sum = *sum + num;
    } else {
      dfs(&node.borrow().left, num, sum);
      dfs(&node.borrow().right, num, sum);
    }
  }
}

pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  let mut result = 0;
  dfs(&root, 0, &mut result);
  result
}

fn main() {
    println!("Hello, world!");
}
