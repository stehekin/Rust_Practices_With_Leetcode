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
    fn inorder(mut node: Option<Rc<RefCell<TreeNode>>>, nodes: &mut Vec<Rc<RefCell<TreeNode>>>) {
        if node.is_none() {
            return
        }

        let node = node.unwrap();
        Self::inorder(node.borrow_mut().left.take(), nodes);
        nodes.push(node.clone());
        Self::inorder(node.borrow_mut().right.take(), nodes);
    }

    pub fn help(nodes: &Vec<Rc<RefCell<TreeNode>>>, l: usize, r: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if r == usize::MAX || r < l {
            return None
        }

        let mid = (l + r ) / 2;
        let mut rv = nodes[mid].clone();
        rv.borrow_mut().left = Self::help(nodes, l, mid - 1);
        rv.borrow_mut().right = Self::help(nodes, mid + 1, r);
        Some(rv)
    }

    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None
        }

        let mut nodes = Vec::new();
        Self::inorder(root, &mut nodes);
        return Self::help(&nodes, 0, nodes.len() - 1);
    }
}

fn main() {
    println!("Hello, world!");
}
