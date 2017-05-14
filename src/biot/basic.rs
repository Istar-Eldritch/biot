pub fn pattern_count(text: &str, pattern: &str) -> i32 {
    let mut count: i32 = 0;

    for i in 0..(text.len() - pattern.len() + 1) {
        let slice = &text[i..(i + pattern.len())];
        if slice == pattern {
            count += 1;
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::pattern_count;

    #[test]
    fn it_returns_0_on_empty() {
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