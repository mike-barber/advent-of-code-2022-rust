use std::{collections::HashMap, str::FromStr};

use anyhow::bail;
use common::{AnyResult, OptionAnyhow, read_file};
use regex::Regex;

#[derive(Debug,Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            _ => Err(anyhow::anyhow!("Invalid operation: {}", s)),
        }
    }
}

#[derive(Debug,Clone, Copy)]
enum MonkeyExpr<'a> {
    Literal(i32),
    Operation(Op, &'a str, &'a str),
}


fn parse_input(input: &str) -> AnyResult<HashMap<&str, MonkeyExpr>> {
    let re_literal = Regex::new(r#"(\w+): (\d+)"#)?;
    let re_expr = Regex::new(r#"(\w+)+: (\w+) ([+\-*/]) (\w+)"#)?;

    let mut monkeys = HashMap::new();
    for line in input.lines() {
        if let Some(literal) = re_literal.captures(line) {
            let id = literal.get(1).ok_anyhow()?.as_str();
            let val = literal.get(2).ok_anyhow()?.as_str().parse()?;
            monkeys.insert(id, MonkeyExpr::Literal(val));
        } else if let Some(expression) = re_expr.captures(line) {
            let id = expression.get(1).ok_anyhow()?.as_str();
            let left = expression.get(2).ok_anyhow()?.as_str();
            let op = expression.get(3).ok_anyhow()?.as_str().parse()?;
            let right = expression.get(4).ok_anyhow()?.as_str();
            monkeys.insert(id, MonkeyExpr::Operation(op, left, right));
        } else {
            bail!("Failed to match: {}", line);
        }
    }

    Ok(monkeys)
}

fn main() -> AnyResult<()>{
    let contents = read_file("day21/input.txt")?;
    let input = parse_input(&contents)?;
    
    for i in input {
        println!("{i:?}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        root: pppw + sjmn
        dbpl: 5
        cczh: sllz + lgvd
        zczc: 2
        ptdq: humn - dvpt
        dvpt: 3
        lfqf: 4
        humn: 5
        ljgn: 2
        sjmn: drzm * dbpl
        sllz: 4
        pppw: cczh / lfqf
        lgvd: ljgn * ptdq
        drzm: hmdt - zczc
        hmdt: 32
    "};

    #[test]
    fn parse_input_correct() {
        let input = parse_input(TEST_INPUT).unwrap();
        for i in &input {
            println!("{i:?}");
        }
    }

    // #[test]
    // fn part1_correct() {
    //     let input = parse_input(TEST_INPUT);
    //     let res = part1(&input).unwrap();
    //     assert_eq!(res, 64);
    // }

    // #[test]
    // fn part2_correct() {
    //     let input = parse_input(TEST_INPUT);
    //     let res = part2(&input).unwrap();
    //     assert_eq!(res, 58);
    // }
}
