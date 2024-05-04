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

fn help(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
  if node.is_none() {
    return (-1, -1)
  }

  let node = node.as_ref().unwrap().clone();
  let (l, lr) = help(&node.borrow().left);
  let (r, rr) = help(&node.borrow().right);

  let nr = i32::max(lr, rr) + 1;
  let n = i32::max(i32::max(l, r), lr + rr + 2);

  (n, nr)
}

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  let (result, _) = help(&root);
  result
}

fn main() {
    println!("Hello, world!");
}
