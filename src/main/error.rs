use std::num::ParseIntError;

use wjp::{ParseError, Serialize, Values};

#[derive(Debug, Default,Copy, Clone)]
pub struct WIMCError;

impl From<ParseIntError> for WIMCError {
    fn from(_value: ParseIntError) -> Self {
        Self
    }
}

impl Serialize for WIMCError {
    fn serialize(&self) -> Values {
        Values::String(String::from("Hello"))
    }
}

impl TryFrom<Values> for WIMCError {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let string = value.get_string().ok_or(ParseError::new())?;
        Ok(WIMCError)
    }
}