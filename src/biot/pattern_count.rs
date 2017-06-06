
use hamming_distance;

pub fn pattern_count(text: &str, pattern: &str) -> i32 {
  let mut count: i32 = 0;

  if text.len() >= pattern.len() && pattern.len() > 0 {
    for i in 0..(text.len() - pattern.len() + 1) {
      let slice = &text[i..(i + pattern.len())];
      if slice == pattern {
        count += 1;
      }
    }
  }

  return count;
}

pub fn approx_pattern_count(text: &str, pattern: &str, distance: i32) -> i32 {
  let mut count: i32 = 0;
  if text.len() >= pattern.len() && pattern.len() > 0 {
    for i in 0..(text.len() - pattern.len() + 1) {
      let slice = &text[i..(i+pattern.len())];
      if hamming_distance(slice, pattern) <= distance {
        count += 1;
      }
    }
  }
  count
}

pub fn approx_pattern_index(text: &str, pattern: &str, distance: i32) -> Vec<i32> {
  let mut result: Vec<i32> = vec!();
  if pattern.len() > 0 && text.len() >= pattern.len() {
    for i in 0..(text.len() - pattern.len() + 1) {
      let slice = &text[i..(i + pattern.len())];
      if hamming_distance(slice, pattern) <= distance {
        result.push(i as i32);
      }
    }
  }
  result
}

#[cfg(test)]
mod approx_pattern_index_test {
  use super::approx_pattern_index;

  #[test]
  fn on_empty() {
    assert!(approx_pattern_index("abc", "", 2) == []);
  }

  #[test]
  fn match_exact_on_distance_0() {
    println!("{:?}", approx_pattern_index("abc", "abc", 0));
    assert!(approx_pattern_index("abc", "abc", 0) == [0]);
    assert!(approx_pattern_index("abc", "acc", 0) == []);
  }

  #[test]
  fn match_when_distance_is_less() {
    assert!(approx_pattern_index("abc", "acc", 1) == [0]);
  }
}

#[cfg(test)]
mod pattern_count_test {
  use super::pattern_count;

  #[test]
  fn it_returns_0_on_empty() {
    assert!(pattern_count("abc", "") == 0);
  }

  #[test]
  fn it_returns_0_on_pattern_bigger_than_text() {
    assert!(pattern_count("a", "asd") == 0);
  }

  #[test]
  fn it_returns_0_on_non_match() {
    assert!(pattern_count("abc", "d") == 0);
  }

  #[test]
  fn it_returns_1_at_beginning() {
    assert!(pattern_count("dabc", "d") == 1);
  }

  #[test]
  fn it_returns_1_at_end() {
    assert!(pattern_count("abcd", "d") == 1);
  }

  #[test]
  fn it_returns_1_at_middle() {
    assert!(pattern_count("abc", "b") == 1);
  }
}
