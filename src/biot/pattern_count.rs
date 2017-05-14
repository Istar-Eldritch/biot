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