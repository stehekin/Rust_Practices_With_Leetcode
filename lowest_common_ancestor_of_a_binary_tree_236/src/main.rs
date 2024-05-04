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

pub fn find(node: Option<Rc<RefCell<TreeNode>>>, p: Rc<RefCell<TreeNode>>, q: Rc<RefCell<TreeNode>>) -> (bool, Option<Rc<RefCell<TreeNode>>>) {
  if let Some(node) = node {
    let (left_found, left_anchor) = find(node.borrow().left.clone(), p.clone(), q.clone());
    if left_anchor.is_some() {
      return (true, left_anchor)
    }

    let (right_found, right_anchor) = find(node.borrow().right.clone(), p.clone(), q.clone());
    if right_anchor.is_some() {
      return (true, right_anchor)
    }

    if left_found && right_found {
      return (true, Some(node))
    }

    if (left_found || right_found) && (node == p || node == q) {
      return (true, Some(node))
    }

    return (left_found || right_found || node == p || node == q, None)
  } else {
    (false, None)
  }
}

pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
  let (_, result) = find(root, p.unwrap(), q.unwrap());
  result
}

fn main() {
    println!("Hello, world!");
}
