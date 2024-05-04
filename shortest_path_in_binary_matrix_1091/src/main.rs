use std::collections::VecDeque;

pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
  let rs = grid.len();
  let cs = grid[rs - 1].len();

  if grid[0][0] == 1 || grid[rs - 1][cs - 1] == 1{
    return -1
  }

  let mut visited = vec![vec![false; cs]; rs];
  let mut queue = VecDeque::from(vec![(0, 0)]);
  let mut path = 0;

  while queue.len() > 0 {
    let size = queue.len();
    for i in 0..size {
      let (r, c) = queue[i];
      if r == rs as i32 - 1 && c == cs as i32 - 1 {
        return path;
      }
      if visited[r as usize][c as usize] {
        continue
      }
      visited[r as usize][c as usize] = true;
      for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)] {
        let r = r + dr;
        let c = c + dc;
        if r >= 0 && r < rs as i32 && c >= 0 && c < cs as i32 && !visited[r as usize][c as usize] && grid[r as usize][c as usize] == 0 {
          queue.push_back((r, c));
        }
      }
    }
    queue.drain(..size);
    path += 1;
  }

  -1
}

fn main() {
    println!("Hello, world!");
}
