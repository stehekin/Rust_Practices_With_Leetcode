use std::collections::HashMap;

struct Node {
  value: i32,
  key: i32,
  prev: *mut Node,
  next: *mut Node,
}

impl Node {
  fn new() -> *mut Node {
    let node = Box::new(Node{
      value: -1,
      key: -1,
      prev: std::ptr::null_mut(),
      next: std::ptr::null_mut(),
    });
    return Box::leak(node)
  }
}

impl Drop for Node {
    fn drop(&mut self) {
        print!("drop!\n")
    }
}

struct LRUCache {
  map: HashMap<i32, *mut Node>,
  head: *mut Node,
  size: usize,
  capacity: usize,
}

impl LRUCache {
  fn new(capacity: i32) -> Self {
    let dummy = Node::new();
    unsafe {
      dummy.as_mut().unwrap().next = dummy;
      dummy.as_mut().unwrap().prev = dummy;
    }

    LRUCache {
      map: HashMap::new(),
      head: dummy,
      size: 0,
      capacity: capacity as usize,
    }
  }


  unsafe fn detach(node: *mut Node) {
    if node.is_null() {
      return
    }

    let prev = node.as_mut().unwrap().prev;
    let next = node.as_mut().unwrap().next;

    (*prev).next = next;
    (*next).prev = prev;
  }


  unsafe fn attach(head: *mut Node, node: *mut Node) {
    let next = head.as_mut().unwrap().next;

    (*node).next = next;
    (*next).prev = node;

    (*head).next = node;
    (*node).prev = head;
  }


  fn get(&mut self, key: i32) -> i32 {
    if let Some(&mut node) = self.map.get_mut(&key) {
      unsafe {
        let result = node.as_ref().unwrap().value;
        Self::detach(node);
        Self::attach(self.head, node);
        result
      }
    } else {
      -1
    }
  }

  fn put(&mut self, key: i32, value: i32) {
    if let Some(&mut node) = self.map.get_mut(&key) {
      unsafe {
        (*node).value = value;
        Self::detach(node);
        Self::attach(self.head, node);
      }
    } else {
      let node = Node::new();
      unsafe {
        (*node).key = key;
        (*node).value = value;
        Self::attach(self.head, node);
      }
      self.map.insert(key, node);
      if self.size < self.capacity {
        self.size += 1;
      } else {
        unsafe {
          let tail = (*self.head).prev;
          Self::detach(tail);
          self.map.remove(&(*tail).key);
          let _ = Box::from_raw(tail);
        }
      }
    }
  }
}


impl Drop for LRUCache {
    fn drop(&mut self) {
      let map = std::mem::take(&mut self.map);
      for (_, v) in map {
        unsafe {
          v.drop_in_place()
        }
      }
    }
}

fn main() {
  let mut lru = LRUCache::new(2);
  lru.put(1, 1);
  lru.put(2, 2);
  print!(">> {} <<\n", lru.get(1));
  lru.put(3, 3);
  print!(">> {} <<\n", lru.get(2));
  lru.put(4, 4);


  print!(">> {} <<\n", lru.get(1));
  print!(">> {} <<\n", lru.get(3));
  print!(">> {} <<\n", lru.get(4));
}
