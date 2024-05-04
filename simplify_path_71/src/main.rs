pub fn simplify_path(path: String) -> String {
  let components: Vec<&str> = path.split('/').collect();
  let mut stack = Vec::<&str>::new();

  for c in components {
      match c {
      ".." => {
          stack.pop();
      }
      "" | "." | "/" => {

      }
      _ => {
          stack.push(c);
      }
      }
  }

  if stack.len() == 0 {
      return "/".to_string();
  }

  let mut result = "".to_string();
  for c in stack {
      result.push('/');
      result.push_str(c);
  }
  result
}

fn main() {
    println!("Hello, world!");
}
