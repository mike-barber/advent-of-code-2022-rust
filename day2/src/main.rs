use std::{fs::File, io::Read};

// this allows us to map a string to an enum value
// e.g. "A" -> Opponent::A
use strum::EnumString;

/// Domain model: opponent's input, which
/// gets mapped to rock, paper or scissors.
#[derive(Copy, Clone, EnumString)]
enum Opponent {
    A, // rock
    B, // paper
    C, // scissors
}

/// Domain model: "my" input, X, Y or Z; this
/// means different things depending on the part
/// of the problem.
#[derive(Copy, Clone, EnumString)]
enum Myself {
    X, // rock (part1); lose (part2)
    Y, // paper (part1); draw (part2)
    Z, // scissors (part1); win (part2)
}

/// Domain model: Rock, Paper or Scissors, with
/// the value of the play as the enum value.
#[derive(Copy, Clone)]
enum Play {
    R = 1,
    P = 2,
    S = 3,
}

// both parts - map opponent's input to a play
impl From<Opponent> for Play {
    fn from(opp: Opponent) -> Self {
        match opp {
            Opponent::A => Play::R,
            Opponent::B => Play::P,
            Opponent::C => Play::S,
        }
    }
}

// part 1 - map second column to a play
impl From<Myself> for Play {
    fn from(myself: Myself) -> Self {
        match myself {
            Myself::X => Play::R,
            Myself::Y => Play::P,
            Myself::Z => Play::S,
        }
    }
}

// part 2 - map second column to an outcome
impl From<Myself> for Outcome {
    fn from(myself: Myself) -> Self {
        match myself {
            Myself::X => Outcome::Lose,
            Myself::Y => Outcome::Draw,
            Myself::Z => Outcome::Win,
        }
    }
}

/// Domain model: result of the round, with
/// associated value.
#[derive(Copy, Clone)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

mod part1 {
    use crate::{Myself, Opponent, Outcome, Play};
    use std::str::FromStr;

    fn outcome(opp: Opponent, me_play: Play) -> Outcome {
        let opp_play: Play = opp.into();
        match (me_play, opp_play) {
            (Play::R, Play::R) => Outcome::Draw,
            (Play::R, Play::P) => Outcome::Lose,
            (Play::R, Play::S) => Outcome::Win,
            (Play::P, Play::R) => Outcome::Win,
            (Play::P, Play::P) => Outcome::Draw,
            (Play::P, Play::S) => Outcome::Lose,
            (Play::S, Play::R) => Outcome::Lose,
            (Play::S, Play::P) => Outcome::Win,
            (Play::S, Play::S) => Outcome::Draw,
        }
    }

    fn score_round(opp: Opponent, me: Myself) -> i32 {
        let my_play: Play = me.into();
        let outcome = outcome(opp, my_play);
        let outcome_value = outcome as i32;

        let my_choice_value = my_play as i32;

        outcome_value + my_choice_value
    }

    fn parse_round(line: &str) -> (Opponent, Myself) {
        let mut fields = line.split(' ');
        (
            Opponent::from_str(fields.next().unwrap()).unwrap(),
            Myself::from_str(fields.next().unwrap()).unwrap(),
        )
    }

    pub fn run_part1(input: &str) {
        let rounds = input.lines().map(|round| {
            let (opp, me) = parse_round(round);
            score_round(opp, me)
        });
        let total: i32 = rounds.sum();

        println!("Total score for part 1: {total}");
    }

    #[cfg(test)]
    mod tests {
        use crate::part1::{parse_round, score_round};
        const EXPECTED_VALS: [i32; 3] = [8, 1, 6];
        const TEST_INPUT: &str = "A Y
B X
C Z";

        #[test]
        fn scores_correct() {
            let rounds = TEST_INPUT.lines();

            for (round, expected) in rounds.zip(EXPECTED_VALS) {
                let (opp, me) = parse_round(round);
                let score = score_round(opp, me);
                assert_eq!(score, expected);
            }
        }
    }
}

mod part2 {
    use crate::{Myself, Opponent, Outcome, Play};
    use std::str::FromStr;

    fn required_play(opp: Opponent, outcome: Outcome) -> Play {
        let opp_play: Play = opp.into();
        match (outcome, opp_play) {
            (Outcome::Lose, Play::R) => Play::S,
            (Outcome::Lose, Play::P) => Play::R,
            (Outcome::Lose, Play::S) => Play::P,
            (Outcome::Draw, Play::R) => Play::R,
            (Outcome::Draw, Play::P) => Play::P,
            (Outcome::Draw, Play::S) => Play::S,
            (Outcome::Win, Play::R) => Play::P,
            (Outcome::Win, Play::P) => Play::S,
            (Outcome::Win, Play::S) => Play::R,
        }
    }

    fn score_round(opp: Opponent, me: Myself) -> i32 {
        let outcome: Outcome = me.into();
        let my_play = required_play(opp, outcome);

        let outcome_value = outcome as i32;
        let my_choice_value = my_play as i32;
        outcome_value + my_choice_value
    }

    fn parse_round(line: &str) -> (Opponent, Myself) {
        let mut fields = line.split(' ');
        (
            Opponent::from_str(fields.next().unwrap()).unwrap(),
            Myself::from_str(fields.next().unwrap()).unwrap(),
        )
    }

    pub fn run_part2(input: &str) {
        let rounds = input.lines().map(|round| {
            let (opp, me) = parse_round(round);
            score_round(opp, me)
        });
        let total: i32 = rounds.sum();

        println!("Total score for part 1: {total}");
    }

    #[cfg(test)]
    mod tests {
        use crate::part2::{parse_round, score_round};
        const EXPECTED_VALS: [i32; 3] = [4, 1, 7];
        const TEST_INPUT: &str = "A Y
B X
C Z";

        #[test]
        fn scores_correct() {
            let rounds = TEST_INPUT.lines();

            for (round, expected) in rounds.zip(EXPECTED_VALS) {
                let (opp, me) = parse_round(round);
                let score = score_round(opp, me);
                assert_eq!(score, expected);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    File::open("input1.txt")
        .expect("file")
        .read_to_string(&mut input)
        .expect("read");

    part1::run_part1(&input);
    part2::run_part2(&input);
}
