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

struct Solution {}

impl Solution {
  fn walk(node: &Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
    let node = node.as_ref().unwrap();

    let left = &node.as_ref().borrow().left;
    let right = &node.as_ref().borrow().right;

    let mut ld = 0;
    let mut lc = None;
    if left.is_some() {
      (lc, ld) = Self::walk(left)
    }

    let mut rd = 0;
    let mut rc = None;
    if right.is_some() {
      (rc, rd) = Self::walk(right)
    }

    if ld == rd {
      return (Some(node.clone()), ld + 1)
    } else if ld > rd {
      return (lc, ld + 1)
    } else {
      return (rc, rd + 1)
    }
  }

    pub fn subtree_with_all_deepest(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
      let (node, _) = Self::walk(&root);
      node
    }
}

fn main() {

}