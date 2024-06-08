use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
      let mut result = vec![];
      if root.is_none() {
        return result;
      }

      let mut queue = VecDeque::from([root.unwrap().clone()]);
      while !queue.is_empty() {
        let len = queue.len();
        let mut max = i32::MIN;
        for _ in 0..len {
          let node = queue.pop_front().unwrap();
          let node = node.borrow();

          if node.val > max {
            max = node.val;
          }

          if let Some(left) = &node.left {
            queue.push_back(left.clone());
          }

          if let Some(right) = &node.right {
            queue.push_back(right.clone());
          }
        }
        result.push(max);
      }

      result
    }
}