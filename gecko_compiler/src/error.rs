
use std::{fmt, error::Error};

#[derive(Debug)]
pub struct TypeError;

impl fmt::Display for TypeError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Parse Error: Invalid parse")
    }
}

impl Error for TypeError { }