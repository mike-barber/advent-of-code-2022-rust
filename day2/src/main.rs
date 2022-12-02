use std::{str::FromStr, fs::File, io::Read};

use strum::EnumString;

#[derive(Copy, Clone, EnumString)]
enum Opponent {
    A, // rock
    B, // paper
    C, // scissors
}

#[derive(Copy, Clone, EnumString)]
enum Myself {
    X = 1, // rock
    Y = 2, // paper
    Z = 3, // scissors
}

enum Play {
    R,
    P,
    S,
}
impl From<Opponent> for Play {
    fn from(opp: Opponent) -> Self {
        match opp {
            Opponent::A => Play::R,
            Opponent::B => Play::P,
            Opponent::C => Play::S,
        }
    }
}
impl From<Myself> for Play {
    fn from(myself: Myself) -> Self {
        match myself {
            Myself::X => Play::R,
            Myself::Y => Play::P,
            Myself::Z => Play::S,
        }
    }
}

#[derive(Copy, Clone)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn outcome(opp: Opponent, me: Myself) -> Outcome {
    let opp_play: Play = opp.into();
    let me_play: Play = me.into();

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
    let outcome = outcome(opp, me);
    let outcome_value = outcome as i32;
    let my_choice_value = me as i32;
    outcome_value + my_choice_value
}

fn parse_round(line: &str) -> (Opponent, Myself) {
    let mut fields = line.split(' ');
    (
        Opponent::from_str(fields.next().unwrap()).unwrap(),
        Myself::from_str(fields.next().unwrap()).unwrap(),
    )
}

fn main() {
    part1();    
}

fn part1() {
    let mut input = String::new();
    File::open("input1.txt").expect("file").read_to_string(&mut input).expect("read");
    
    let rounds = input.lines().map(|round| {
        let (opp,me) = parse_round(round);
        score_round(opp, me)
    });

    let total:i32 = rounds.sum();

    println!("Total score for part 1: {total}");
}

#[cfg(test)]
mod tests {
    use crate::{parse_round, score_round};

    const TEST_INPUT: &str = "A Y
B X
C Z";

    const EXPECTED_VALS: [i32;3] = [8, 1, 6];

    #[test]
    fn scores_correct() {
        let rounds = TEST_INPUT.lines();

        for (round, expected) in rounds.zip(EXPECTED_VALS) {
            let (opp,me) = parse_round(round);
            let score = score_round(opp, me);
            assert_eq!(score, expected);
        }
    }
}
