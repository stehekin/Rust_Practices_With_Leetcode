fn add_strings(num1: String, num2: String) -> String {
  let mut result = String::new();
  let mut carry = 0;

  let (mut iter1, mut iter2) = (num1.chars().rev(), num2.chars().rev());

  loop {
    let (c1, c2) = (iter1.next(), iter2.next());
    let (v1, v2) = match(c1, c2) {
      (Some(d1), Some(d2)) => (d1.to_digit(10).unwrap(), d2.to_digit(10).unwrap()),
      (Some(d1), None) => (d1.to_digit(10).unwrap(), 0),
      (None, Some(d2)) => (0, d2.to_digit(10).unwrap()),
      (None, None) => break,
    };

    let sum = v1 + v2 + carry;
    carry = sum / 10;
    result.push(std::char::from_digit(sum % 10, 10).unwrap());
  }

  if carry != 0 {
    result.push(std::char::from_digit(carry, 10).unwrap());
  }

  result.chars().rev().collect()
}

fn main() {
    println!("Hello, world!");
}
