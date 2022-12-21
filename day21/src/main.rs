use std::{collections::HashMap, str::FromStr};

use anyhow::bail;
use common::{read_file, AnyResult, OptionAnyhow};
use regex::Regex;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
enum MonkeyExpr<'a> {
    Literal(i64),
    Operation(Op, &'a str, &'a str),
}

type Monkeys<'a> = HashMap<&'a str, MonkeyExpr<'a>>;
type Values<'a> = HashMap<&'a str, i64>;

fn parse_input(input: &str) -> AnyResult<Monkeys> {
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

fn calculate(id: &str, monkeys: &Monkeys, values: &mut Values) -> Option<i64> {
    if let Some(value) = values.get(id) {
        return Some(*value);
    }

    let expr = monkeys.get(id)?;
    match expr {
        MonkeyExpr::Literal(v) => Some(*v),
        MonkeyExpr::Operation(op, l, r) => {
            let left = values
                .get(l)
                .copied()
                .or_else(|| calculate(l, monkeys, values))?;
            let right = values
                .get(r)
                .copied()
                .or_else(|| calculate(r, monkeys, values))?;
            Some(match op {
                Op::Add => left + right,
                Op::Sub => left - right,
                Op::Mul => left * right,
                Op::Div => left / right,
            })
        }
    }
}

fn part1(monkeys: &Monkeys) -> Option<i64> {
    let mut values: Values = HashMap::new();
    calculate("root", monkeys, &mut values)
}

fn part2(monkeys: &Monkeys) -> Option<i64> {
    let root = monkeys.get("root")?;
    let (left_id, right_id) = {
        if let MonkeyExpr::Operation(_, l, r) = root {
            (*l, *r)
        } else {
            panic!("invalid root")
        }
    };

    let mut monkeys = monkeys.clone();
    let mut values: Values = HashMap::new();
    // for humn in [0, 100, 200, 300, 301] {
    //     values.clear();
    //     monkeys
    //         .entry("humn")
    //         .and_modify(|v| *v = MonkeyExpr::Literal(humn));
    //     let left = calculate(left_id, &monkeys, &mut values);
    //     let right = calculate(right_id, &monkeys, &mut values);
    //     println!("humn {humn} left {left:?} right {right:?}");
    // }

    let mut evaluate_error = |value: i64| -> Option<i64> {
        values.clear();
        monkeys
            .entry("humn")
            .and_modify(|v| *v = MonkeyExpr::Literal(value));
        let left = calculate(left_id, &monkeys, &mut values);
        let right = calculate(right_id, &monkeys, &mut values);
        Some(left? - right?)
    };
    // for humn in 0..100 {
    //     let error = evaluate_error(humn);
    //     println!("humn = {humn} error = {error:?}");
    // }

    let mut x0 = 1637;
    let mut x1 = x0 + 100;
    for i in 0.. {
        let fx0 = evaluate_error(x0).unwrap() as f64;
        let fx1 = evaluate_error(x1).unwrap() as f64;

        if fx1.abs() < 1e-6 {
            println!("found {x1} => {fx1}");
            break;
        }

        let xn = (x0 as f64 * fx1 - x1 as f64 * fx0) / (fx1 - fx0);
        
        x0 = x1;
        x1 = xn.round() as i64;

        println!("iter {i}; {x0} => {fx1}")
    }

    Some(x1)
}

fn main() -> AnyResult<()> {
    let contents = read_file("day21/input.txt")?;
    let monkeys = parse_input(&contents)?;
    // for i in input {
    //     println!("{i:?}");
    // }

    println!("part1 result: {:?}", part1(&monkeys));
    println!("part2 result: {:?}", part2(&monkeys));
    println!("note: 3887609741191 is too high");

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

    #[test]
    fn part1_correct() {
        let input = parse_input(TEST_INPUT).unwrap();
        let res = part1(&input).unwrap();
        assert_eq!(res, 152);
    }

    #[test]
    fn part2_correct() {
        let input = parse_input(TEST_INPUT).unwrap();
        let res = part2(&input).unwrap();
        assert_eq!(res, 301);
    }
}
