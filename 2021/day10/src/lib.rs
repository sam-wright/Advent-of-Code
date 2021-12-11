use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|x| x.unwrap()).collect()
}

fn score_bad_char(line: &str) -> u64 {
    for c in line.chars() {
        if c == ')' {
            return 3;
        } else if c == ']' {
            return 57;
        } else if c == '}' {
            return 1197;
        } else if c == '>' {
            return 25137;
        }
    }
    0
}

pub fn evaluate_line(line: &str) -> u64 {
    let mut line = line.to_string();
    loop {
        if line.contains("()") {
            line = line.replace("()", "");
            continue;
        } else if line.contains("[]") {
            line = line.replace("[]", "");
            continue;
        } else if line.contains("{}") {
            line = line.replace("{}", "");
            continue;
        } else if line.contains("<>") {
            line = line.replace("<>", "");
            continue;
        } else if line.is_empty() {
            return 0;
        } else {
            println!("Bad Line: {}", line);
            return score_bad_char(&line);
        }
    }
}

fn generate_closing(line: &str) -> String {
    let closing = line.to_string();

    let closing = closing.replace("(", ")");
    let closing = closing.replace("[", "]");
    let closing = closing.replace("{", "}");
    let closing = closing.replace("<", ">");

    closing.chars().rev().collect::<String>()
}

fn score_closing(closing: &str) -> u64 {
    let closing = closing.to_string();
    closing.chars().fold(0, |accum, x| {
        let val = match x {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("badness"),
        };
        accum * 5 + val
    })
}

pub fn complete_line(line: &str) -> u64 {
    let mut line = line.to_string();
    loop {
        if line.contains("()") {
            line = line.replace("()", "");
            continue;
        } else if line.contains("[]") {
            line = line.replace("[]", "");
            continue;
        } else if line.contains("{}") {
            line = line.replace("{}", "");
            continue;
        } else if line.contains("<>") {
            line = line.replace("<>", "");
            continue;
        } else if line.is_empty() {
            return 0;
        } else {
            if score_bad_char(&line) > 0 {
                return 0;
            } else {
                println!("Incomplete Line: {}", line);
                let closing = generate_closing(&line);
                println!("Closing: {}", closing);

                return score_closing(&closing);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example1() {
        let good1 = "()";
        let good2 = "[]";
        let good3 = "([])";
        let good4 = "{()()()}";
        let good5 = "<([{}])>";
        let good6 = "[<>({}){}[([])<>]]";
        let good7 = "(((((((((())))))))))";

        assert_eq!(evaluate_line(good1), 0);
        assert_eq!(evaluate_line(good2), 0);
        assert_eq!(evaluate_line(good3), 0);
        assert_eq!(evaluate_line(good4), 0);
        assert_eq!(evaluate_line(good5), 0);
        assert_eq!(evaluate_line(good6), 0);
        assert_eq!(evaluate_line(good7), 0);

        let bad1 = "{([(<{}[<>[]}>{[]{[(<()>";
        let bad2 = "[[<[([]))<([[{}[[()]]]";
        let bad3 = "[{[{({}]{}}([{[{{{}}([]";
        let bad4 = "[<(<(<(<{}))><([]([]()";
        let bad5 = "<{([([[(<>()){}]>(<<{{";

        assert_eq!(evaluate_line(&bad1), 1197);
        assert_eq!(evaluate_line(&bad2), 3);
        assert_eq!(evaluate_line(&bad3), 57);
        assert_eq!(evaluate_line(&bad4), 3);
        assert_eq!(evaluate_line(&bad5), 25137);

        let input = read_input("example.txt");
        assert_eq!(
            input.iter().fold(0, |sum, line| sum + evaluate_line(line)),
            26397
        );
    }

    #[test]
    fn part1() {
        let input = read_input("input.txt");
        assert_eq!(
            input.iter().fold(0, |sum, line| sum + evaluate_line(line)),
            390993
        );
    }

    #[test]
    fn example2() {
        assert_eq!(complete_line("[({(<(())[]>[[{[]{<()<>>"), 288957);
        assert_eq!(complete_line("[(()[<>])]({[<{<<[]>>("), 5566);
        assert_eq!(complete_line("(((({<>}<{<{<>}{[]{[]{}"), 1480781);
        assert_eq!(complete_line("{<[[]]>}<{[{[{[]{()[[[]"), 995444);
        assert_eq!(complete_line("<{([{{}}[<[[[<>{}]]]>[]]"), 294);

        let mut scores1 = vec![
            complete_line("[({(<(())[]>[[{[]{<()<>>"),
            complete_line("[(()[<>])]({[<{<<[]>>("),
            complete_line("(((({<>}<{<{<>}{[]{[]{}"),
            complete_line("{<[[]]>}<{[{[{[]{()[[[]"),
            complete_line("<{([{{}}[<[[[<>{}]]]>[]]"),
        ];

        scores1.sort();

        assert_eq!(scores1[scores1.len() / 2], 288957);

        let input = read_input("example.txt");
        let mut scores = Vec::new();
        for line in input {
            let score = complete_line(&line);
            if score > 0 {
                println!("Pushing {}", score);
                scores.push(score);
            }
        }
        scores.sort();

        assert_eq!(scores[scores.len() / 2], 288957);
    }

    #[test]
    fn part2() {
        let input = read_input("input.txt");
        let mut scores = Vec::new();
        for line in input {
            let score = complete_line(&line);
            if score > 0 {
                println!("Pushing {}", score);
                scores.push(score);
            }
        }
        scores.sort();

        assert_eq!(scores[scores.len() / 2], 2391385187);

    }
}
