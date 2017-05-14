use biot::pattern_count;

pub fn frequent_words(text: &str, k: usize) -> Vec<&str> {
    let mut frequents = vec!();

    if k > 0 && text.len() > k {
        let mut count = vec![0; text.len() - k];

        for i in 0..(text.len() - k) {
            let word = &text[i..(i + k)];
            count[i] = pattern_count(&text, &word);
        }

        let max = count.iter().max().unwrap();

        for i in 0..count.len() {
            if &count[i] == max {
                let word = &text[i..(i+k)];
                if !frequents.contains(&word) {
                    frequents.push(&word);
                }
            }
        }
    }

    println!("{:?}", frequents);
    return frequents;
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
}