use std::collections::HashSet;

pub fn get_start(input: &str, marker_len: usize) -> usize {
    let mut pos = marker_len;
    for win in input.as_bytes().windows(marker_len) {
        let win_set: HashSet<&u8> = win.iter().collect();

        if win_set.len() == marker_len {
            break;
        } else {
            pos += 1;
        }
    }
    pos
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn example1() {
        let marker_len = 4;
        assert_eq!(get_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb", marker_len), 7);
        assert_eq!(get_start("bvwbjplbgvbhsrlpgdmjqwftvncz", marker_len), 5);
        assert_eq!(get_start("nppdvjthqldpwncqszvftbrmjlhg", marker_len), 6);
        assert_eq!(
            get_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", marker_len),
            10
        );
        assert_eq!(
            get_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", marker_len),
            11
        );
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("input.txt").unwrap();
        assert_eq!(get_start(&input, 4), 1343);
    }

    #[test]
    fn example2() {
        let marker_len = 14;
        assert_eq!(get_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb", marker_len), 19);
        assert_eq!(get_start("bvwbjplbgvbhsrlpgdmjqwftvncz", marker_len), 23);
        assert_eq!(get_start("nppdvjthqldpwncqszvftbrmjlhg", marker_len), 23);
        assert_eq!(
            get_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", marker_len),
            29
        );
        assert_eq!(
            get_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", marker_len),
            26
        );
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("input.txt").unwrap();
        assert_eq!(get_start(&input, 14), 2193);
    }
}
