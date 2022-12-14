use std::cmp::Ordering;

pub mod parser;

#[derive(Debug, Clone, PartialEq, Ord, Eq)]
pub enum Value {
    Literal(i32),
    List(Vec<Value>),
}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Literal(l), Value::Literal(r)) => l.partial_cmp(r),
            (Value::Literal(l), Value::List(r)) => partial_cmp_list(&singleton_list(*l), r),
            (Value::List(l), Value::Literal(r)) => partial_cmp_list(l, &singleton_list(*r)),
            (Value::List(l), Value::List(r)) => partial_cmp_list(l, r),
        }
    }
}
impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Literal(v)
    }
}
impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Value::List(v)
    }
}

fn partial_cmp_list(left: &[Value], right: &[Value]) -> Option<std::cmp::Ordering> {
    // check common items
    for (l, r) in std::iter::zip(left, right) {
        let cmp = l.partial_cmp(r).unwrap();
        if cmp != Ordering::Equal {
            return Some(cmp);
        }
    }

    // run out of common items; check length
    left.len().partial_cmp(&right.len())
}

fn singleton_list(literal: i32) -> Vec<Value> {
    vec![literal.into()]
}

#[derive(Debug, Clone)]
pub struct Pair(pub Value, pub Value);

#[derive(Debug, Clone)]
pub struct Problem {
    pub pairs: Vec<Pair>,
}
