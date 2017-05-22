
pub fn hamming_distance(str1: &str, str2: &str) ->i32 {

    let mut distance = 0;
    if str1.len() > 0 {
        let mut chars1 = str1.chars();
        let mut chars2 = str2.chars();

        for i in 0..str1.len() {
            let c1 = chars1.next().unwrap();
            let c2 = chars2.next().unwrap();
            if c1 != c2 {
                distance += 1;
            }
        }
    }
    distance
}

#[cfg(test)]
mod skew_test {
    use super::hamming_distance;

    #[test]
    fn hamming_distance_empty() {
        assert!(hamming_distance("", "") == 0);
    }

    #[test]
    fn hamming_distance_same() {
        assert!(hamming_distance("ACTG", "ACTG") == 0);
    }

    #[test]
    fn hamming_distance_diff() {
        assert!(hamming_distance("GGGCCGTTGGT", "GGACCGTTGAC") == 3);
    }
}