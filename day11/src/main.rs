use anyhow::{anyhow, bail};
use num_bigint::BigInt;
use regex::Regex;
use std::{fs::File, io::Read};


fn read_file(file_name: &str) -> String {
    let mut contents = String::new();
    File::open(file_name)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    contents
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    AddConst(i32),
    MulConst(i32),
    Square,
}
impl Operation {
    fn calculate_new(&self, old: &BigInt, division_factor: &BigInt) -> BigInt {
        let new = match self {
            Operation::AddConst(a) => old + a,
            Operation::MulConst(a) => old * a,
            Operation::Square => old * old,
        };
        new / division_factor
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    operation: Operation,
    test_divisible_by: BigInt,
    test_true: usize,
    test_false: usize,
}

#[derive(Debug, Clone)]
struct Simulation {
    monkeys: Vec<Monkey>,
    holding_items: Vec<Vec<BigInt>>,
    inspection_counts: Vec<usize>,
}

impl Simulation {

    fn simulation_part1(&mut self) {
        self.simulation_round(3.into());
    }

    fn simulation_part2(&mut self) {
        self.simulation_round(1.into());
    }


    fn simulation_round(&mut self, division_factor: BigInt) {
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
                let new_val = monkey.operation.calculate_new(item, &division_factor);
                let dest_monkey = if &new_val % &monkey.test_divisible_by == 0.into() {
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
            println!("{:?}", items);
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

    println!("{monkeys:?}");
    println!("{holding_items:?}");
    
    let inspection_counts = vec![0; monkeys.len()];

    Ok(Simulation {
        monkeys,
        holding_items,
        inspection_counts
    })
}


fn part1(inputs: &str) -> anyhow::Result<usize> {
    let mut simulation = parse_input(inputs)?;

    for _ in 0..20 {
        simulation.simulation_part1();
    }

    let mut counts = simulation.inspection_counts;
    counts.sort();
    let monkey_business = counts.iter().rev().take(2).product();
    
    Ok(monkey_business)
}


// fn parse_input(inputs: &str) -> anyhow::Result<Vec<Instruction>> {
//     let mut instructions = vec![];
//     for line in inputs.lines() {
//         let mut split = line.split_whitespace();
//         let code = split.next();
//         let instruction = match code {
//             Some("noop") => Instruction::NoOp,
//             Some("addx") => {
//                 let argument = split.next().ok_or_else(|| anyhow!("no argument for add"))?;
//                 let argument = argument.parse()?;
//                 Instruction::Add(argument)
//             }
//             _ => bail!("unrecognised instruction {code:?}"),
//         };
//         instructions.push(instruction);
//     }
//     Ok(instructions)
// }

fn main() -> anyhow::Result<()> {
    let inputs = read_file("input.txt");
    
    println!("part1 result: {}", part1(&inputs)?);

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
        println!("{:?}", sim.holding_items);
        println!("{:?}", sim.inspection_counts);
        sim.simulation_part1();
        println!("{:?}", sim.holding_items);
        println!("{:?}", sim.inspection_counts);
    }

    #[test]
    fn part1_correct() {
        let res = part1(TEST_INPUT).unwrap();
        assert_eq!(res, 10605);
    }

    #[test]
    fn simulate_one_part2() {
        let mut sim = parse_input(TEST_INPUT).unwrap();
        for i in 1..=10_000 {
            sim.simulation_part2();
            //println!("round {i} => {:?} holding {:?}", sim.inspection_counts, sim.holding_items);
            println!("round {i} => {:?}", sim.inspection_counts);
        }
    }
}
