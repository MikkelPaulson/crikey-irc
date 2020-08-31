use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
pub struct ParseError(&'static str);

impl ParseError {
    pub fn new(struct_name: &'static str) -> Self {
        ParseError(struct_name)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse component: {}", self)
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
