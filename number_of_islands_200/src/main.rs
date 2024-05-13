
fn dfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
  let nr = grid.len();
  let nc = grid[0].len();

  if i >= nr || j >= nc || grid[i][j] != '1' || visited[i][j] {
    return
  }

  visited[i][j] = true;

  dfs(grid, visited, i - 1, j);
  dfs(grid, visited, i + 1, j);
  dfs(grid, visited, i, j - 1);
  dfs(grid, visited, i, j + 1);
}

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
  let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
  let mut count = 0;

  for i in 0..grid.len() {
    for j in 0..grid[i].len() {
      if grid[i][j] == '1' && !visited[i][j] {
        dfs(&grid, &mut visited, i, j);
        count += 1;
      }
    }
  }
  count
}

fn main() {
    println!("Hello, world!");
}
