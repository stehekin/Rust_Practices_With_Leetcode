use std::{collections::VecDeque, rc::Rc};
use std::vec::Vec;

struct Node <S, A> where S: PartialEq {
  state: Rc<S>,
  parent: Option<Rc<Node<S, A>>>,
  action: Option<Rc<A>>,
}

trait Coll<T> {
  fn add(&mut self, item: T);
  fn remove(&mut self) -> Option<T>;
}

impl<T> Coll<T> for Vec<T> {
    fn add(&mut self, item: T) {
        self.push(item)
    }

    fn remove(&mut self) -> Option<T> {
      self.pop()
   }
}

impl<T> Coll<T> for VecDeque<T> {
    fn add(&mut self, item: T) {
        self.push_back(item)
    }

    fn remove(&mut self) -> Option<T> {
        self.pop_front()
    }
}

trait Frontier<S, A, C> where
    S: PartialEq,
    C:Coll<Rc<Node<S, A>>> + IntoIterator<Item = Rc<Node<S, A>>>,
    for<'a> &'a mut C: IntoIterator<Item = &'a mut Rc<Node<S, A>>>,
    for<'a> &'a C: IntoIterator<Item = &'a Rc<Node<S, A>>> {

  fn collection_mut(&mut self) -> &mut C;
  fn collection(&self) -> &C;

  fn add(&mut self, node: Rc<Node<S, A>>) {
    self.collection_mut().add(node.clone())
  }

  fn remove(&mut self) -> Option<Rc<Node<S, A>>> {
    self.collection_mut().remove()
  }

  fn contains(&self, state: S) -> bool {
    for node in self.collection().into_iter() {
      if node.state.as_ref().eq(&state) {
        return true
      }
    }
    false
  }
}

struct StackFrontier<S, A> where S:PartialEq {
  collection: Vec<Rc<Node<S, A>>>
}

impl<S, A> Frontier<S, A, Vec<Rc<Node<S, A>>>> for StackFrontier<S, A> where S:PartialEq {
    fn collection_mut(&mut self) -> &mut Vec<Rc<Node<S, A>>> {
        &mut self.collection
    }

    fn collection(&self) -> &Vec<Rc<Node<S, A>>> {
        &self.collection
    }
}

struct QueueFrontier<S, A> where S:PartialEq {
  collection: VecDeque<Rc<Node<S, A>>>
}

impl<S, A> Frontier<S, A, VecDeque<Rc<Node<S, A>>>> for QueueFrontier<S, A> where S:PartialEq {
    fn collection_mut(&mut self) -> &mut VecDeque<Rc<Node<S, A>>> {
        &mut self.collection
    }

    fn collection(&self) -> &VecDeque<Rc<Node<S, A>>> {
        &self.collection
    }
}
