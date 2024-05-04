use std::collections::HashMap;

fn signature(s: &str) -> String {
  let mut result = "".to_owned();
  let mut i = 1;
  while i < s.bytes().len() {
    let diff = format!("::{}", (s.bytes().nth(i).unwrap() - s.bytes().nth(i - 1).unwrap() + 26) % 26);
    result.push_str(&diff);
    i += 1;
  }
  result
}

pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
  let mut result = Vec::<Vec<String>>::new();
  let mut hash: HashMap<String, Vec<String>> = HashMap::new();

  for s in strings {
    let sig = signature(&s);
    let vec = hash.entry(sig).or_insert(Vec::new());
    vec.push(s);
  }

  for (_, v) in hash {
    result.push(v);
  }

  result
}

fn main() {
    println!("Hello, world!");
}
