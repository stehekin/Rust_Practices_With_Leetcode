struct Solution {}

impl Solution {
  fn help(n: i32, edges: &Vec<Vec<i32>>, has_apple: &Vec<bool>, visited: &mut Vec<bool>) -> Option<i32> {
    let mut cost = 0;
    visited[n as usize] = true;

    for child in edges[n as usize].iter() {
      if visited[*child as usize] {
        continue;
      }
      let cost_child = Self::help(*child, edges, has_apple, visited);
      if cost_child.is_some() {
        cost += cost_child.unwrap() + 2;
      }
    }

    if cost == 0 {
      if has_apple[n as usize] {
        return Some(0);
      } else {
        return None;
      }
    } else {
      return Some(cost);
    }
  }

  pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
    let mut conn = vec![vec![]; n as usize];
    for edge in edges {
      conn[edge[0] as usize].push(edge[1]);
      conn[edge[1] as usize].push(edge[0]);
    }

    let mut visited = vec![false; n as usize];

    let result = Self::help(0, &conn, &has_apple, &mut visited);
    result.unwrap_or_default()
  }
}

fn main() {
    println!("Hello, world!");
}
