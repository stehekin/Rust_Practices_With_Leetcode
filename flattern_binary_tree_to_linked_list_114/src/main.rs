use std::borrow::Borrow;
// Definition for a binary tree node.
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

struct Solution {}

impl Solution {
  fn preorder(node: &mut Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = node {
      let ltail = Self::preorder(&mut node.borrow_mut().left);
      let mut rtail = Self::preorder(&mut node.borrow_mut().right);

      if let Some(ltail) = ltail {
        ltail.borrow_mut().right = node.borrow_mut().right.take();
        let mut node = node.borrow_mut();
        node.right = node.left.take();
        if rtail.is_none() {
          rtail = Some(ltail.clone());
        }
      }

      node.borrow_mut().left = None;
      if rtail.is_none() {
        Some(node.clone())
      } else {
        rtail
      }
    } else {
      None
    }
  }

  pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    Self::preorder(root);
  }
}

fn main() {
    println!("Hello, world!");
}
