use std::collections::HashMap;

pub fn custom_sort_string(order: String, s: String) -> String {
  let mut map: HashMap<u8, usize> = HashMap::new();
  for (i, c) in order.bytes().enumerate() {
      map.insert(c, i);
  }
  let mut chars: Vec<u8> = s.bytes().collect();
  chars.sort_by(|a, b| map.get(a).unwrap_or(&usize::MAX).cmp(map.get(b).unwrap_or(&usize::MAX)));

  String::from_utf8_lossy(&chars).into()
}

fn main() {
    println!("Hello, world!");
}
