use std::collections::HashSet;

struct Solution {}

impl Solution {
  fn isWall(maze: &Vec<Vec<i32>>, r: i32, c: i32) -> bool {
    let rn = maze.len() as i32;
    let cn = maze[0].len() as i32;

    r < 0 || c < 0 || r >= rn || c >= cn || maze[r as usize][c as usize] == 1
  }

  pub fn has_path(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> bool {
    let mut frontier = vec![(start[0], start[1])];
    let mut visited = HashSet::from([(start[0], start[1])]);
    let steps = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while !frontier.is_empty() {
      let node = frontier.pop().unwrap();

      for s in steps {
        let mut node = node;
        let mut nextNode = node;

        while !Self::isWall(&maze, nextNode.0, nextNode.1) {
          node = nextNode;
          nextNode = (node.0 + s.0, node.1 + s.1);
        }

        if node.0 == destination[0] && node.1 == destination[1] {
          return true
        }

        if !visited.contains(&node) {
          visited.insert(node);
          frontier.push(node);
        }
      }
    }

    false
  }
}



fn main() {
    println!("Hello, world!");
}
