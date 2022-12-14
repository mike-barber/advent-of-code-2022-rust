use anyhow::{anyhow, bail};
use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Operation {
    AddConst(i64),
    MulConst(i64),
    Square,
}
impl Operation {
    fn calculate_new(&self, old: &i64) -> i64 {
        match self {
            Operation::AddConst(a) => old + a,
            Operation::MulConst(a) => old * a,
            Operation::Square => old * old,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    operation: Operation,
    test_divisible_by: i64,
    test_true: usize,
    test_false: usize,
}

#[derive(Debug, Clone)]
struct Simulation {
    monkeys: Vec<Monkey>,
    holding_items: Vec<Vec<i64>>,
    inspection_counts: Vec<usize>,
}

impl Simulation {
    fn simulation_part1(&mut self, num_rounds: usize) {
        for _ in 0..num_rounds {
            self.simulation_round(|n| n / 3);
        }
    }

    fn simulation_part2(&mut self, num_rounds: usize) {
        let global_modulus: i64 = self.monkeys.iter().map(|m| m.test_divisible_by).product();
        for _ in 0..num_rounds {
            self.simulation_round(|n| n % global_modulus);
        }
    }

    fn simulation_round<F: Fn(i64) -> i64>(&mut self, post_op: F) {
        let length = self.monkeys.len();
        for i in 0..length {
            let monkey = &mut self.monkeys[i];

            // take all items for this monkey, replacing them with an empty vector
            let mut items = vec![];
            std::mem::swap(&mut self.holding_items[i], &mut items);

            // increase inspection counts for this monkey
            self.inspection_counts[i] += items.len();

            // give the items to the other monkeys according to the rules
            for item in items.iter() {
                let new_val = monkey.operation.calculate_new(item);
                let new_val = post_op(new_val);
                let dest_monkey = if new_val % monkey.test_divisible_by == 0.into() {
                    monkey.test_true
                } else {
                    monkey.test_false
                };

                // give new item to destination monkey
                self.holding_items[dest_monkey].push(new_val);
            }
        }
    }
}

fn parse_input(inputs: &str) -> anyhow::Result<Simulation> {
    //let re_monkey = Regex::new(r#"Monkey (\d+)"#)?;
    let re_numbers = Regex::new(r#"\d+"#)?;
    let re_operation = Regex::new(r#"Operation: new = old ([+*]) (\d+|old)"#)?;

    let mut monkeys = vec![];
    let mut holding_items = vec![];

    let lines: Vec<_> = inputs.lines().collect();
    for block in lines.split(|l| l.is_empty()) {
        let starting_items = {
            let mut items = vec![];
            for cap in re_numbers.captures_iter(block[1]) {
                let item = cap[0].parse()?;
                items.push(item);
            }
            items
        };

        let operation = {
            let cap = re_operation
                .captures(block[2])
                .ok_or_else(|| anyhow!("operation missing"))?;
            match (&cap[1], &cap[2]) {
                ("*", "old") => Operation::Square,
                ("*", num) => Operation::MulConst(num.parse()?),
                ("+", num) => Operation::AddConst(num.parse()?),
                _ => bail!("invalid operation"),
            }
        };

        let test_divisible = re_numbers
            .captures(block[3])
            .ok_or_else(|| anyhow!("divisible by missing"))?[0]
            .parse()?;
        let test_true: usize = re_numbers
            .captures(block[4])
            .ok_or_else(|| anyhow!("divisible by missing"))?[0]
            .parse()?;
        let test_false: usize = re_numbers
            .captures(block[5])
            .ok_or_else(|| anyhow!("divisible by missing"))?[0]
            .parse()?;

        let monkey = Monkey {
            operation,
            test_divisible_by: test_divisible,
            test_true,
            test_false,
        };

        monkeys.push(monkey);
        holding_items.push(starting_items);
    }

    // println!("{monkeys:?}");
    // println!("{holding_items:?}");

    let inspection_counts = vec![0; monkeys.len()];

    Ok(Simulation {
        monkeys,
        holding_items,
        inspection_counts,
    })
}

fn part1(inputs: &str) -> anyhow::Result<usize> {
    let mut simulation = parse_input(inputs)?;
    simulation.simulation_part1(20);

    let mut counts = simulation.inspection_counts;
    counts.sort();
    let monkey_business = counts.iter().rev().take(2).product();

    Ok(monkey_business)
}

fn part2(inputs: &str) -> anyhow::Result<usize> {
    let mut simulation = parse_input(inputs)?;
    simulation.simulation_part2(10_000);

    let mut counts = simulation.inspection_counts;
    counts.sort();
    let monkey_business = counts.iter().rev().take(2).product();

    Ok(monkey_business)
}

fn main() -> anyhow::Result<()> {
    let inputs = common::read_file("input.txt")?;

    println!("part1 result: {}", part1(&inputs)?);
    println!("part2 result: {}", part2(&inputs)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

        Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

        Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
    "};

    #[test]
    fn basic_input_execution() {
        parse_input(TEST_INPUT).unwrap();
    }

    #[test]
    fn simulate_one_part1() {
        let mut sim = parse_input(TEST_INPUT).unwrap();
        sim.simulation_part1(20);
        assert_eq!(sim.inspection_counts, [101, 95, 7, 105])
    }

    #[test]
    fn part1_correct() {
        let res = part1(TEST_INPUT).unwrap();
        assert_eq!(res, 10605);
    }

    #[test]
    fn simulate_one_part2() {
        let mut sim = parse_input(TEST_INPUT).unwrap();
        sim.simulation_part2(10_000);
        assert_eq!(sim.inspection_counts, [52166, 47830, 1938, 52013])
    }
}
