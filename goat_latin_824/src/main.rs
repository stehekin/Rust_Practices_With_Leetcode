struct Solution {}

impl Solution {
  fn is_vowel(c: u8) -> bool {
    c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u' ||
    c == b'A' || c == b'E' || c == b'I' || c == b'O' || c == b'U'
  }

  fn transform(mut word: String, index: usize) -> String {
    if word.len() == 0 {
      return word
    }

    let word_vec = unsafe {word.as_mut_vec()};

    if !Self::is_vowel(word_vec[0]) {
      word_vec.push(word_vec[0]);
      word_vec.remove(0);
    }
    word_vec.push(b'm');
    word_vec.push(b'a');

    for _ in 0..=index {
      word_vec.push(b'a')
    }

    String::from_utf8_lossy(word_vec).into_owned()
  }


  pub fn to_goat_latin(sentence: String) -> String {
    let words = sentence.split(" ");
    let mut result = Vec::<String>::new();
    for (i, w) in words.enumerate() {
      let word = Self::transform(w.to_owned(), i);
      result.push(word)
    }
    result.join(" ")
  }
}

fn main() {
    println!("Hello, world!");
}
