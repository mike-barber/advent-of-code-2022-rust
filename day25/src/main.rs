use anyhow::bail;
use common::AnyResult;

struct Snafu(String);
impl From<&str> for Snafu {
    fn from(value: &str) -> Self {
        Snafu(value.to_owned())
    }
}
impl TryInto<i32> for Snafu {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<i32, Self::Error> {
        let mut chars = self.0.chars();

        let mut digits = vec![];
        while let Some(ch) = chars.next() {
            let val = match ch {
                '0'..='2' => ch as i32 - '0' as i32,
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

fn int_to_snafu(value: i32) -> Snafu {
    todo!()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn parse_snafu_correct() {
        assert_eq!(1, Snafu::from("1").try_into().unwrap());
        assert_eq!(2, Snafu::from("2").try_into().unwrap());
        assert_eq!(3, Snafu::from("1=").try_into().unwrap());
        assert_eq!(4, Snafu::from("1-").try_into().unwrap());
        assert_eq!(5, Snafu::from("10").try_into().unwrap());
        assert_eq!(6, Snafu::from("11").try_into().unwrap());
        assert_eq!(7, Snafu::from("12").try_into().unwrap());
        assert_eq!(8, Snafu::from("2=").try_into().unwrap());
        assert_eq!(9, Snafu::from("2-").try_into().unwrap());
        assert_eq!(10, Snafu::from("20").try_into().unwrap());
        assert_eq!(15, Snafu::from("1=0").try_into().unwrap());
        assert_eq!(20, Snafu::from("1-0").try_into().unwrap());
        assert_eq!(2022, Snafu::from("1=11-2").try_into().unwrap());
        assert_eq!(12345, Snafu::from("1-0---0").try_into().unwrap());
        assert_eq!(314159265, Snafu::from("1121-1110-1=0").try_into().unwrap());
    }
}
