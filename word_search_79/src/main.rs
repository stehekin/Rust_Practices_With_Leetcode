struct Solution {}

impl Solution {
  fn help(board: &Vec<Vec<char>>, r: usize, c: usize, word: &[u8], i: usize, visited: &mut Vec<Vec<bool>>) -> bool {
    let nbs = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];

    if i == word.len() {
      return true;
    }

    if r >= board.len() || c >= board[r].len() || visited[r][c] || board[r][c] != word[i] as char {
      return false;
    }

    visited[r][c] = true;

    for nb in nbs {
      let nbr = r + nb.0 as usize;
      let nbc = c + nb.1 as usize;

      if Self::help(board, nbr, nbc, word, i + 1, visited) {
        return true;
      }
    }

    visited[r][c] = false;

    false
  }

  pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let rn = board.len();
    let cn = board[0].len();
    for r in 0..rn {
      for c in 0..cn {
        let mut visited = vec![vec![false; cn]; rn];
        if Self::help(&board, r, c, word.as_bytes(), 0, &mut visited) {
          return true
        }
      }
    }

    false
  }
}

fn main() {
    println!("Hello, world!");
}
