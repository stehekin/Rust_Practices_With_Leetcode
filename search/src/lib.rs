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
    for<'a> &'a mut C: IntoIterator<Item = Rc<Node<S, A>>> {
  // type S: PartialEq;
  // type A;
  // type C: Coll<Rc<Node<Self::S, Self::A>>> + IntoIterator<Item = Rc<Node<Self::S, Self::A>>> where for<'a> &'a Self::C: IntoIterator<Item = Rc<Node<Self::S, Self::A>>>;

  fn collection(&self) -> &mut C;

  fn add(&mut self, node: Rc<Node<S, A>>) {
    self.collection().add(node.clone())
  }

  fn remove(&mut self) -> Option<Rc<Node<S, A>>> {
    self.collection().remove()
  }

  fn contains(&self, state: S) -> bool {
    let q = self.collection();
    for node in q.into_iter() {
      if node.state.as_ref().eq(&state) {
        return true
      }
    }
    false
  }
}

// struct StackFrontier<S, A> where S:PartialEq {
//   collection: Vec<Rc<Node<S, A>>>
// }

// impl<S, A> Frontier<S, A> for StackFrontier<S, A> where S:PartialEq {
//     type C=VecDeque<Rc<Node<S, A>>>;

//     fn collection(&self) -> Self::C {
//         self.collection
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
