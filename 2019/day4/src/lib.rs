#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    fn is_six_digits(val: i32) -> bool {
        val > 99999 && val < 1000000
    }

    fn has_adj_dup(val: i32) -> bool {
        let str_val = val.to_string();
        let bytes_val = str_val.as_bytes();

        for i in 0..5 {
            if bytes_val[i] == bytes_val[i + 1] {
                return true;
            }
        }
        false
    }

    fn has_2adj_dup(val: i32) -> bool {
        let str_val = val.to_string();
        let bytes_val = str_val.as_bytes().to_vec();
        let mut observed = HashMap::new();

        for byte in bytes_val {
            let e = observed.entry(byte).or_insert(0);
            *e += 1;
        }

        for o in observed.values() {
            if *o == 2 {
                return true;
            }
        }
        false
    }

    fn never_decreases(val: i32) -> bool {
        let str_val = val.to_string();
        let bytes_val = str_val.as_bytes();

        for i in 0..5 {
            if bytes_val[i + 1] < bytes_val[i] {
                return false;
            }
        }
        true
    }

    fn crack_code1(min: i32, max: i32) -> i32 {
        let mut victory = 0;
        for val in min..=max {
            if is_six_digits(val) && has_adj_dup(val) && never_decreases(val) {
                victory += 1;
            }
        }

        victory
    }

    fn crack_code2(min: i32, max: i32) -> i32 {
        let mut victory = 0;
        for val in min..=max {
            if is_six_digits(val) && has_2adj_dup(val) && never_decreases(val) {
                victory += 1;
            }
        }

        victory
    }

    #[test]
    fn tests() {
        // check 6 digits
        assert!(is_six_digits(108457));
        assert!(is_six_digits(562041));
        assert!(!is_six_digits(1108457));
        assert!(!is_six_digits(62041));

        // check for adj_dupes
        assert!(!has_adj_dup(108457));
        assert!(!has_adj_dup(123456));
        assert!(has_adj_dup(118457));
        assert!(has_adj_dup(108477));
        assert!(has_adj_dup(111111));

        // check for never decreasing
        assert!(!never_decreases(108457));
        assert!(never_decreases(111111));
        assert!(never_decreases(123456));

        // check for 2 adh dupes
        assert!(has_2adj_dup(112345));
        assert!(!has_2adj_dup(111234));
        assert!(has_2adj_dup(122345));
        assert!(!has_2adj_dup(122234));
        assert!(has_2adj_dup(123455));
        assert!(!has_2adj_dup(123444));
        assert!(!has_2adj_dup(111111));
        assert!(has_2adj_dup(111221));
    }

    #[test]
    fn part1() {
        assert_eq!(crack_code1(108457, 562041), 2779);
    }
    #[test]
    fn part2() {
        assert_eq!(crack_code2(108457, 562041), 1972);
    }
}
