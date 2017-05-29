
pub fn hamming_distance(str1: &str, str2: &str) ->i32 {

    let mut distance = 0;
    if str1.len() > 0 {
        let mut chars1 = str1.chars();
        let mut chars2 = str2.chars();

        for _ in 0..str1.len() {
            let c1 = chars1.next().unwrap();
            let c2 = chars2.next().unwrap();
            if c1 != c2 {
                distance += 1;
            }
        }
    }
    distance
}

pub fn reverse_complement(str: &str) -> String {
    let mut complement = String::new();
    for i in 0..str.len() {
        let symbol = &str[i..i+1];
        let c_symbol = match symbol {
            "A" => "T",
            "C" => "G",
            "G" => "C",
            "T" => "A",
            _ => unreachable!("Wops")
        };
        complement.push_str(&c_symbol);
    }

    complement.chars().rev().collect()
}

pub fn neighbors(pattern: &str, d: i32) -> Vec<String> {
    if d == 0 {
        return vec![String::from(pattern)]
    } else if pattern.len() == 1 {
        return vec!["A", "C", "G", "T"].into_iter().map(|x| String::from(x)).collect()
    }
    let mut neighborhood: Vec<String> = vec!();
    let suffix_pattern = &pattern[1..];
    let suffix_neighbors = neighbors(suffix_pattern, d);
    for i in 0..suffix_neighbors.len() {
        if hamming_distance(suffix_pattern, &suffix_neighbors[i]) < d {
            for n in vec!["A", "C", "G", "T"] {
                let mut str = String::from(n);
                str.push_str(&suffix_neighbors[i]);
                neighborhood.push(str)
            }
        } else {
            let mut str = String::from(&pattern[..1]);
            str.push_str(&suffix_neighbors[i]);
            neighborhood.push(str);
        }
    }
    neighborhood
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

#[cfg(test)]
mod reverse_complement_test {
    use super::reverse_complement;

    #[test]
    fn reverse_complement_0() {
        assert_eq!(reverse_complement(""), "");
    }

    #[test]
    fn reverse_complement_one() {
        assert_eq!(reverse_complement("A"), "T");
    }

    #[test]
    fn reverse_complement_complete() {
        assert_eq!(reverse_complement("ACTG"), "CAGT");
    }

}