
pub fn skew(genome: &str) -> Vec<i32> {
    let mut result = vec!();
    result.push(0);

    let mut chars = genome.chars();
    loop {
        let prev = result.last().unwrap().clone();
        match chars.next() {
            Some(a) => {
                match a {
                    'C' => result.push(prev - 1),
                    'G' => result.push(prev + 1),
                    _ => result.push(prev)
                }
            },
            None => break
        }
    }
    result
}

#[cfg(test)]
mod skew_test {
    use super::skew;

    #[test]
    fn skew_empty() {
        assert!(skew("") == [0]);
    }

    #[test]
    fn skew_c_subtracts() {
        assert!(skew("C") == [0, -1]);
    }

    #[test]
    fn skew_g_adds() {
        assert!(skew("G") == [0, 1]);
    }

    #[test]
    fn skew_a_and_t_are_neutral() {
        assert!(skew("A") == [0, 0]);
        assert!(skew("T") == [0, 0]);
    }

    #[test]
    fn skew_larger_str() {
        assert!(skew("CATGGGCATCGGCCATACGCC") == [
            0, -1, -1, -1, 0, 1, 2, 1, 1, 1, 0,
            1, 2, 1, 0, 0, 0, 0, -1, 0, -1, -2
        ]);
    }
}