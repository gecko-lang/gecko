
use std::{fmt, error::Error};

#[derive(Debug, Clone)]
pub struct TypeError {
    pub text: String,
}

impl fmt::Display for TypeError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(format!("TypeError: {}", self.text).as_str())
    }
}

impl Error for TypeError { }