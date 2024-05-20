use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
  fn help(word: &mut Vec<char>, i: usize, j: usize, cands: &mut Vec<char>, k: usize, single: Option<char>, result: &mut HashSet<String>) {
    if k >= cands.len() {
      if let Some(single) = single {
        word[i] = single;
      }
      result.insert(word.iter().collect());
    }
    for m in k..cands.len() {
      cands.swap(k, m);
      word[i] = cands[k];
      word[j] = cands[k];
      Self::help(word, i + 1, j - 1, cands, k + 1, single, result);
      cands.swap(k, m)
    }
  }

  pub fn generate_palindromes(s: String) -> Vec<String> {
    let mut cand = vec![];
    let mut count = HashMap::<char, usize>::new();

    for ch in s.chars() {
      *count.entry(ch).or_insert(0) += 1;
      if count[&ch] == 2 {
        cand.push(ch);
        *count.get_mut(&ch).unwrap() = 0;
      }
    }

    let mut result = HashSet::new();
    let mut single = None;
    for (k, v) in count {
      if v== 1 {
        if single.is_some() {
          return result.into_iter().collect()
        }
        single = Some(k)
      }
    }

    let mut wordLen = cand.len() * 2;
    if single.is_some() {
      wordLen += 1;
    }
    let mut word = vec![0 as char; wordLen];
    Self::help(&mut word, 0, wordLen - 1, &mut cand, 0, single, &mut result);
    result.into_iter().collect()
  }
}

fn main() {
  let r = Solution::generate_palindromes("aalls".to_owned());
  print!("{}", r.join("\n"));
}


