use biot::{pattern_count, approx_pattern_count, neighbors};

pub fn frequent_words(text: &str, k: usize) -> Vec<String> {
  let mut frequents = vec!();

  if k > 0 && text.len() > k {
    let mut count = vec!();

    for i in 0..(text.len() - k) {
      let word = &text[i..(i + k)];
      count.push(pattern_count(&text, &word));
    }

    let max = count.iter().max().unwrap();

    for i in 0..count.len() {
      if &count[i] == max {
        let word = &text[i..(i+k)];
        if !frequents.contains(&String::from(word)) {
          frequents.push(String::from(word));
        }
      }
    }
  }

  return frequents;
}

pub fn frequent_words_with_mismatches(text: &str, k: usize, d: i32) -> Vec<String> {
  let mut frequents: Vec<String> = vec!();
  let mut freq_array = vec!(0; 4usize.pow(k as u32));
  let mut close = vec!(0; 4usize.pow(k as u32));

  for i in 0..(text.len() - k) {
    let neighborhood = neighbors(&text[i..(i + k)], d);
    for p in neighborhood {
      let index = pattern_to_number(&p);
      close[index] = 1;
    }
  }
  for i in 0..close.len() {
    if close[i] == 1 {
      let pattern = number_to_pattern(i, k);
      freq_array[i] = approx_pattern_count(text, &pattern, d) as usize;
    }
  }
  let max_count = freq_array.iter().max().unwrap();

  for i in 0..freq_array.len() {
    if &freq_array[i] == max_count {
      frequents.push(number_to_pattern(i, k))
    }
  }
  frequents
}

pub fn freq_array_words(text: &str, k: usize) -> Vec<String> {
  let mut frequents: Vec<String> = vec!();
  if k > 0 && text.len() >= k {
    let freqs = freq_array(text, k as u32);
    let max_count = freqs.iter().max().unwrap();
    for i in 0..(4usize.pow(k as u32) - 1) {
      if &freqs[i] == max_count {
        let pattern = number_to_pattern(i, k);
        frequents.push(pattern);
      }
    }
  }
  frequents
}

pub fn sort_frequent_words(text: &str, k: usize) -> Vec<String> {
  let mut frequents: Vec<String> = vec!();
  if k > 0 && text.len() >= k {
    let mut index: Vec<i32> = vec!();
    let mut count: Vec<i32> = vec!(1; text.len() - 1);

    for i in 0..text.len() - k + 1 {
      let pattern = &text[i..(i + k)];
      index.push(pattern_to_number(pattern) as i32);
    }
    index.sort();
    for i in 1..index.len() {
      if index[i] == index[i - 1] {
        count[i] = count[i - 1] + 1;
      }
    }

    let max = count.iter().max().unwrap();
    for i in 0..count.len() {
      if &count[i] == max {
        let pattern = number_to_pattern(index[i] as usize, k);
        frequents.push(pattern);
      }
    }
  }

  frequents
}

fn freq_array(text: &str, k: u32) -> Vec<i32> {
  let mut frequency = vec![0; 4usize.pow(k)];
  for i in 0..(text.len() - 1) {
    let num = pattern_to_number(&text[i..i + k as usize]);
    frequency[num] += 1;
  }

  frequency
}

fn pattern_to_number(pattern: &str) -> usize {
  pattern_to_number_helper(pattern, 0)
}

fn pattern_to_number_helper(pattern: &str, acc: usize) -> usize {
  match pattern.len() {
    0 => acc,
    _ => {
      4 * pattern_to_number(&pattern[0..(pattern.len() -1)]) + symbol_to_number(pattern.chars().nth(pattern.len() - 1).unwrap())
    }
  }
}

fn symbol_to_number(s: char) -> usize {
  match s {
    'A' | 'a' => 0,
    'C' | 'c' => 1,
    'G' | 'g' => 2,
    'T' | 't' => 3,
    _ => {
      println!("NON VALID: {:?}", s);
      unreachable!()
    }
  }
}

fn number_to_pattern(index: usize, k: usize) -> String {
  _number_to_pattern(index, k, String::from(""))
}

fn _number_to_pattern(index: usize, k: usize, mut acc: String) -> String {
  if k == 1 {
    acc.push(number_to_symbol(index));
    acc.chars().rev().collect()
  } else {
    let next_index = index / 4;
    let symbol = number_to_symbol(index % 4);
    acc.push(symbol);
    _number_to_pattern(next_index, k - 1, acc)
  }
}

fn number_to_symbol(s: usize) -> char {
  match s {
    0 => 'A',
    1 => 'C',
    2 => 'G',
    3 => 'T',
    _ => unreachable!()
  }
}

#[cfg(test)]
mod number_to_pattern_test {
  use super::number_to_pattern;

  #[test]
  fn it_returns_a_pattern() {
    println!("{}",number_to_pattern(11,3));
    assert!(number_to_pattern(11, 3) == String::from("AGT"));
  }
}

#[cfg(test)]
mod pattern_to_number_test {
  use super::pattern_to_number;

  #[test]
  fn it_maps_uppercase() {
    assert!(pattern_to_number("AGT") == 11);
  }
}

#[cfg(test)]
mod number_to_symbol_test {
  use super::number_to_symbol;

  #[test]
  fn it_maps_uppercase() {
    assert!(number_to_symbol(0) == 'A');
    assert!(number_to_symbol(1) == 'C');
    assert!(number_to_symbol(2) == 'G');
    assert!(number_to_symbol(3) == 'T');

  }
}

#[cfg(test)]
mod symbol_to_number_test {
  use super::symbol_to_number;

  #[test]
  fn it_maps_uppercase() {
    assert!(symbol_to_number('A') == 0);
    assert!(symbol_to_number('C') == 1);
    assert!(symbol_to_number('G') == 2);
    assert!(symbol_to_number('T') == 3);

  }
}

#[cfg(test)]
mod sort_frequent_words_test {
  use super::sort_frequent_words;

  #[test]
  fn it_returns_empty_on_k_0() {
    assert!(sort_frequent_words("atcg", 0) == vec![] as Vec<&str>)
  }

  #[test]
  fn it_returns_empty_on_text_len_0() {
    assert!(sort_frequent_words("", 3) == vec![] as Vec<&str>)
  }

  #[test]
  fn it_returns_most_frequent_letter() {
    assert!(sort_frequent_words("ata", 1) == vec!["A"])
  }

  #[test]
  fn it_returns_multiple_frequent_words() {
    assert!(sort_frequent_words("atcatca", 2) == vec!["AT", "CA", "TC"])
  }
}

#[cfg(test)]
mod freq_words_with_mismatches {
  use super::frequent_words_with_mismatches;

  #[test]
  fn it_returns_frequent_with_distance() {
    let frequents = frequent_words_with_mismatches("ACGTTGCATGTCGCATGATGCATGAGAGCT", 4, 1);
    assert!(frequents.contains(&String::from("ATGT")));
    assert!(frequents.contains(&String::from("GATG")));
    assert!(frequents.contains(&String::from("ATGC")));
  }
}

#[cfg(test)]
mod freq_array_words_test {
  use super::freq_array_words;

  #[test]
  fn it_returns_empty_on_k_0() {
    assert!(freq_array_words("atcg", 0) == vec![] as Vec<&str>)
  }

  #[test]
  fn it_returns_empty_on_text_len_0() {
    assert!(freq_array_words("", 3) == vec![] as Vec<&str>)
  }

  #[test]
  fn it_returns_most_frequent_letter() {
    assert!(freq_array_words("ata", 1) == vec!["A"])
  }

  #[test]
  fn it_returns_multiple_frequent_words() {
    assert!(freq_array_words("atcatca", 2) == vec!["AT", "CA", "TC"])
  }
}

#[cfg(test)]
mod frequent_words_test {
  use super::frequent_words;

  #[test]
  fn it_returns_empty_on_k_0() {
    assert!(frequent_words("asdf", 0) == vec![] as Vec<&str>)
  }

  #[test]
  fn it_returns_empty_on_text_len_0() {
    assert!(frequent_words("", 3) == vec![] as Vec<&str>)
  }

  #[test]
  fn it_returns_most_frequent_letter() {
    assert!(frequent_words("aba", 1) == vec!["a"])
  }

  #[test]
  fn it_returns_multiple_frequent_words() {
    assert!(frequent_words("abcabca", 2) == vec!["ab", "bc", "ca"])
  }
}
