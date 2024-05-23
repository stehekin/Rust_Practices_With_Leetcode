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

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

struct Solution{}

impl Solution {

  fn help(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if node.is_none() {
      return (-1, -1)
    }

    let node = node.as_ref().unwrap();
    let left = &node.borrow().left;
    let right = &node.borrow().right;

    let (lmax, mut lmaxr) = Self::help(left);
    let (rmax, mut rmaxr) = Self::help(right);

    if let Some(left) = left {
      let lvalue = left.borrow().val;
      if node.borrow().val == lvalue {
        lmaxr = lmaxr + 1;
      } else {
        lmaxr = 0;
      }
    } else {
      lmaxr = 0;
    }

    if let Some(right) = right {
      let rvalue = right.borrow().val;
      if node.borrow().val == rvalue {
        rmaxr = rmaxr + 1;
      } else {
        rmaxr = 0;
      }
    } else {
      rmaxr = 0;
    }

    (max(max(lmax, rmax), lmaxr + rmaxr), max(lmaxr, rmaxr))
  }

  pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let (rv, _) = Self::help(&root);
    rv
  }
}

fn main() {
    println!("Hello, world!");
}
