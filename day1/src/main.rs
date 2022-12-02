use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> anyhow::Result<()> {
    iterators()?;
    imperative()?;
    Ok(())
}

// note the `anyhow::Result` return for simple error handling; it'll take care
// of mapping the Err types returned by the early breakout `?` operator to the `anyhow` type
// for reporting, so this simplifies app development quite a bit.
fn iterators() -> anyhow::Result<()> {
    // create a reader around the file
    let reader = BufReader::new(File::open("input1.txt")?);

    // read all lines into a vector; note that `collect` understands the type
    // Result<Vec<A>,B> and will map Vec<Result<A,B>> into it nicely if everything
    // succeeeds.
    let lines: Result<Vec<String>, _> = reader.lines().collect();
    let lines = lines?;

    // split the lines list into slices separated by empty lines
    let groups = lines.split(String::is_empty);

    // map those groups of lines into sums for each group
    let mut sums: Vec<i32> = groups
        .map(|grp| {
            // sum the lines in a group: parse to i32 -> sum; not super pretty
            // with the `unwrap` in there, but hey...
            let total: i32 = grp.iter().map(|s| s.parse::<i32>().unwrap()).sum();
            total
        })
        .collect();

    // reverse sort
    sums.sort_by_key(|x| -x);

    // check we have sufficient items in the vec, since we're about
    // to just do straight up indexing (and this would panic if the
    // vec was too short...
    if sums.len() < 3 {
        anyhow::bail!("too few groups")
    }

    println!("Answer 1 = {}", sums[0]);

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
