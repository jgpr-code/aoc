use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Answer {
    Num(i128),
    Str(String),
}
impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Answer::Num(i) => write!(f, "{}", i),
            Answer::Str(s) => write!(f, "{}", s),
        }
    }
}
impl From<&str> for Answer {
    fn from(value: &str) -> Self {
        Self::Str(String::from(value))
    }
}
impl From<i128> for Answer {
    fn from(value: i128) -> Self {
        Self::Num(value)
    }
}
