use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> anyhow::Result<()> {
    iterators()?;
    imperative()?;
    Ok(())
}

fn iterators() -> anyhow::Result<()> {
    let reader = BufReader::new(File::open("input1.txt")?);

    let lines: Result<Vec<String>, _> = reader.lines().collect();
    let lines = lines?;
    let groups = lines.split(|l| l.is_empty());

    let mut sums: Vec<i32> = groups
        .map(|grp| {
            let total: i32 = grp.iter().map(|s| s.parse::<i32>().unwrap()).sum();
            total
        })
        .collect();

    sums.sort_by_key(|x| -x);

    if sums.len() < 3 {
        anyhow::bail!("too few groups")
    }

    println!("Answer 1 = {}", sums.first().unwrap());

    let sum_top_3: i32 = sums[0..3].iter().sum();
    println!("Answer 2 = {}", sum_top_3);

    Ok(())
}

fn imperative() -> anyhow::Result<()> {
    let reader = BufReader::new(File::open("input1.txt")?);

    let mut sum = 0;
    let mut sums = Vec::new();
    for res in reader.lines() {
        let l = res?;
        if l.is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            let val: i32 = l.parse()?;
            sum += val;
        }
    }

    sums.sort_by_key(|x| -x);

    if sums.len() < 3 {
        anyhow::bail!("too few groups")
    }

    println!("Answer 1 = {}", sums[0]);

    let sum_top_3: i32 = sums[0..3].iter().sum();
    println!("Answer 2 = {}", sum_top_3);

    Ok(())
}
