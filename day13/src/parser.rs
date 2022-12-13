use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map, recognize},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

use crate::Value;

pub fn parse_value(s: &str) -> IResult<&str, Value> {
    let (s, expr) = alt((parse_parens, parse_number))(s)?;
    Ok((s, expr))
}

pub fn parse_parens(s: &str) -> IResult<&str, Value> {
    let start = recognize(tag("["));
    let end = recognize(tag("]"));
    let elements = separated_list0(tag(","), parse_value);
    let mapped = map(elements, |v| Value::List(v));
    delimited(start, mapped, end)(s)
}

pub fn parse_complete_expression(s: &str) -> IResult<&str, Value> {
    all_consuming(parse_value)(s)
}

pub fn parse_number(i: &str) -> IResult<&str, Value> {
    map(nom::character::complete::i32, |n| Value::Literal(n))(i)
}

pub fn parse(input: &str) -> anyhow::Result<Value> {
    let (_, expr) = parse_complete_expression(input).map_err(|e| e.to_owned())?;
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_list() {
        let s = "[1,2,3]";
        let res = parse(s).unwrap();
        assert_eq!(res, vec![1.into(), 2.into(), 3.into()].into());
    }

    #[test]
    fn parse_nested_list() {
        let s = "[1,[2,3],4]";
        let res = parse(s).unwrap();
        assert_eq!(
            res,
            vec![1.into(), vec![2.into(), 3.into()].into(), 4.into()].into()
        );
    }
}
