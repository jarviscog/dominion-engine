
use std::fmt;

#[derive(Debug, Clone)]
pub enum RuntimeCardName {
    Const(String),
    FromContext(String),
}

impl fmt::Display for RuntimeCardName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Const(str) => write!(f, "\"{}\"", str),
            Self::FromContext(str) => write!(f, "FromContext({})", str),
        }
    }
}
