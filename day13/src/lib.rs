pub mod parser;

#[derive(Debug,Clone,PartialEq)]
pub enum Value {
    Literal(i32),
    List(Vec<Value>)
}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
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

#[derive(Debug,Clone)]
pub struct Pair(pub Value, pub Value);

#[derive(Debug,Clone)]
pub struct Problem {
    pub pairs: Vec<Pair>    
}


// TODO: put this in a common location
pub trait OptionAnyhow<T> {
    fn ok_anyhow(self) -> anyhow::Result<T>;
}
impl<T> OptionAnyhow<T> for Option<T> {
    fn ok_anyhow(self) -> anyhow::Result<T> {
        self.ok_or_else(|| anyhow::anyhow!("expected Some value"))
    }
}