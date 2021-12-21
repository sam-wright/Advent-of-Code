use memoize::memoize;

#[derive(Debug)]
pub struct Player {
    pub position: u32,
    pub score: u32,
}

impl Player {
    pub fn win(&self) -> bool {
        self.score >= 1000
    }
}

pub fn move_distance(current_pos: u32, distance: u32) -> u32 {
    (current_pos + distance - 1) % 10 + 1
}

pub fn roll_deterministic(die: &mut u32, rolls: &mut u32) -> u32 {
    *die += 1;
    *rolls += 1;

    if *die > 100 {
        *die = 1;
    }

    *die
}

fn get_dice_permutations() -> Vec<u32> {
    vec![
        3, 4, 5, 4, 5, 6, 5, 6, 7, 4, 5, 6, 5, 6, 7, 6, 7, 8, 5, 6, 7, 6, 7, 8, 7, 8, 9,
    ]
}

#[memoize]
// algorithm adapted from https://www.reddit.com/r/adventofcode/comments/rl8dzq/2021_day_21_part_2_python3_when_running_code_on/
// a very good demonstration of memoization nonetheless
pub fn calculate_2(p1: u32, p2: u32, p1_score: u32, p2_score: u32, win_score: u32) -> (u64, u64) {
    let mut num_wins = (0, 0);
    for roll1 in get_dice_permutations() {
        for roll2 in get_dice_permutations() {
            let p1_ = move_distance(p1, roll1);
            let p2_ = move_distance(p2, roll2);

            let p1_score_ = p1_score + p1_;
            let p2_score_ = p2_score + p2_;

            if p1_score_ >= win_score {
                num_wins.0 += 1;
                break;
            }
            if p2_score_ >= win_score {
                num_wins.1 += 1;
                continue;
            }
            let l = calculate_2(p1_, p2_, p1_score_, p2_score_, win_score);
            num_wins.0 += l.0;
            num_wins.1 += l.1;
        }
    }
    num_wins
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example1() {
        assert_eq!(move_distance(0, 3), 3);
        assert_eq!(move_distance(4, 1 + 2 + 3), 10);
        assert_eq!(move_distance(8, 4 + 5 + 6), 3);

        //
        // Player 1 starting position: 4
        // Player 2 starting position: 8
        //
        let mut p1 = Player {
            score: 0,
            position: 4,
        };
        let mut p2 = Player {
            score: 0,
            position: 8,
        };

        let mut die = 0;
        let mut rolls = 0;

        loop {
            // player1 goes:
            let roll1 = roll_deterministic(&mut die, &mut rolls);
            let roll2 = roll_deterministic(&mut die, &mut rolls);
            let roll3 = roll_deterministic(&mut die, &mut rolls);

            let points = move_distance(p1.position, roll1 + roll2 + roll3);
            p1.score += points;
            p1.position = points;

            if p1.win() {
                break;
            }

            // player2 goes:
            let roll1 = roll_deterministic(&mut die, &mut rolls);
            let roll2 = roll_deterministic(&mut die, &mut rolls);
            let roll3 = roll_deterministic(&mut die, &mut rolls);

            let points = move_distance(p2.position, roll1 + roll2 + roll3);
            p2.score += points;
            p2.position = points;

            if p2.win() {
                break;
            }
        }

        assert!(p1.win());
        assert_eq!(745, p2.score);
        assert_eq!(993, rolls);
        assert_eq!(739785, p2.score * rolls);
    }

    #[test]
    fn part1() {
        //
        // Player 1 starting position: 2
        // Player 2 starting position: 7
        //
        let mut p1 = Player {
            score: 0,
            position: 2,
        };
        let mut p2 = Player {
            score: 0,
            position: 7,
        };

        let mut die = 0;
        let mut rolls = 0;

        loop {
            // player1 goes:
            let roll1 = roll_deterministic(&mut die, &mut rolls);
            let roll2 = roll_deterministic(&mut die, &mut rolls);
            let roll3 = roll_deterministic(&mut die, &mut rolls);

            let points = move_distance(p1.position, roll1 + roll2 + roll3);
            p1.score += points;
            p1.position = points;

            if p1.win() {
                break;
            }

            // player2 goes:
            let roll1 = roll_deterministic(&mut die, &mut rolls);
            let roll2 = roll_deterministic(&mut die, &mut rolls);
            let roll3 = roll_deterministic(&mut die, &mut rolls);

            let points = move_distance(p2.position, roll1 + roll2 + roll3);
            p2.score += points;
            p2.position = points;

            if p2.win() {
                break;
            }
        }

        assert!(p2.win());
        assert_eq!(805932, p1.score * rolls);
    }

    #[test]
    fn example2() {
        let (p1_wins, p2_wins) = calculate_2(4, 8, 0, 0, 21);

        assert_eq!(444356092776315, p1_wins);
        assert_eq!(341960390180808, p2_wins);
    }

    #[test]
    fn part2() {
        let (p1_wins, p2_wins) = calculate_2(2, 7, 0, 0, 21);

        assert_eq!(133029050096658, p1_wins);
        assert_eq!(74014892331523, p2_wins);
    }
}
