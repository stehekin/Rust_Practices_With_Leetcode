pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
  for r in 0..matrix.len() - 1 {
    for c in 0..matrix[r].len() - 1 {
      if matrix[r][c] != matrix[r + 1][c + 1] {
        return false
      }
    }
  }
  true
}

fn main() {
    println!("Hello, world!");
}
