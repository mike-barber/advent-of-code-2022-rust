use anyhow::bail;
use common::{read_file, AnyResult};

#[derive(Clone, Debug, PartialEq)]
struct Snafu(String);
impl From<&str> for Snafu {
    fn from(value: &str) -> Self {
        Snafu(value.to_owned())
    }
}
impl TryInto<i64> for Snafu {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<i64, Self::Error> {
        let mut digits = vec![];
        for ch in self.0.chars() {
            let val = match ch {
                '0'..='2' => ch as i64 - '0' as i64,
                '-' => -1,
                '=' => -2,
                _ => bail!("invalid snafu digit: {}", ch),
            };
            digits.push(val);
        }

        let mut acc = 0;
        let mut multiplier = 1;
        for digit in digits.iter().rev() {
            let val = digit * multiplier;
            acc += val;
            multiplier *= 5;
        }
        Ok(acc)
    }
}
impl From<i64> for Snafu {
    fn from(value: i64) -> Self {
        let mut v = value;
        let mut chars = vec![];
        while v != 0 {
            let rem = v % 5;
            v /= 5;

            let (ch, carry) = match rem {
                0 => ('0', 0),
                1 => ('1', 0),
                2 => ('2', 0),
                3 => ('=', 1),
                4 => ('-', 1),
                _ => panic!("division failed"),
            };

            chars.push(ch);
            v += carry;
        }

        Snafu(chars.iter().rev().collect())
    }
}

fn parse_input(input: &str) -> AnyResult<Vec<i64>> {
    input.lines().map(|s| Snafu::from(s).try_into()).collect()
}

fn main() -> AnyResult<()> {
    let input = parse_input(&read_file("day25/input.txt")?)?;

    let sum: i64 = input.iter().sum();
    let part1_result = Snafu::from(sum);
    println!("part1 result: {part1_result:?}");

    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::*;
    use indoc::indoc;

    #[test]
    fn parse_snafu_correct() {
        assert_eq!(1_i64, Snafu::from("1").try_into().unwrap());
        assert_eq!(2_i64, Snafu::from("2").try_into().unwrap());
        assert_eq!(3_i64, Snafu::from("1=").try_into().unwrap());
        assert_eq!(4_i64, Snafu::from("1-").try_into().unwrap());
        assert_eq!(5_i64, Snafu::from("10").try_into().unwrap());
        assert_eq!(6_i64, Snafu::from("11").try_into().unwrap());
        assert_eq!(7_i64, Snafu::from("12").try_into().unwrap());
        assert_eq!(8_i64, Snafu::from("2=").try_into().unwrap());
        assert_eq!(9_i64, Snafu::from("2-").try_into().unwrap());
        assert_eq!(10_i64, Snafu::from("20").try_into().unwrap());
        assert_eq!(15_i64, Snafu::from("1=0").try_into().unwrap());
        assert_eq!(20_i64, Snafu::from("1-0").try_into().unwrap());
        assert_eq!(2022_i64, Snafu::from("1=11-2").try_into().unwrap());
        assert_eq!(12345_i64, Snafu::from("1-0---0").try_into().unwrap());
        assert_eq!(
            314159265_i64,
            Snafu::from("1121-1110-1=0").try_into().unwrap()
        );
    }

    #[test]
    fn write_snafu_correct() {
        assert_eq!(Snafu::from(1), Snafu::from("1"));
        assert_eq!(Snafu::from(2), Snafu::from("2"));
        assert_eq!(Snafu::from(3), Snafu::from("1="));
        assert_eq!(Snafu::from(4), Snafu::from("1-"));
        assert_eq!(Snafu::from(5), Snafu::from("10"));
        assert_eq!(Snafu::from(6), Snafu::from("11"));
        assert_eq!(Snafu::from(7), Snafu::from("12"));
        assert_eq!(Snafu::from(8), Snafu::from("2="));
        assert_eq!(Snafu::from(9), Snafu::from("2-"));
        assert_eq!(Snafu::from(10), Snafu::from("20"));
        assert_eq!(Snafu::from(15), Snafu::from("1=0"));
        assert_eq!(Snafu::from(20), Snafu::from("1-0"));
        assert_eq!(Snafu::from(2022), Snafu::from("1=11-2"));
        assert_eq!(Snafu::from(12345), Snafu::from("1-0---0"));
        assert_eq!(Snafu::from(314159265), Snafu::from("1121-1110-1=0"));
    }

    #[test]
    fn part1_correct() {
        let input = indoc! {"
            1=-0-2
            12111
            2=0=
            21
            2=01
            111
            20012
            112
            1=-1=
            1-12
            12
            1=
            122
        "};

        let sum: i64 = parse_input(input).unwrap().iter().sum();
        assert_eq!(sum, 4890);
        let snafu = Snafu::from(sum);
        assert_eq!(snafu, Snafu::from("2=-1=0"));
    }
}
