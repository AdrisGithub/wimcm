use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use wjp::{ParseError, Serialize, Values};

use crate::error::WIMCError;

const NAMES: [&str; 6] = ["Ping", "Echo", "Cleanup", "Store", "Get", "Query"];

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Default, Debug)]
pub enum WIMCMethods {
    #[default]
    Ping,
    Echo,
    Cleanup,
    Store,
    Get,
    Query,
}

impl Serialize for WIMCMethods {
    fn serialize(&self) -> Values {
        Values::String(self.to_string())
    }
}

impl Display for WIMCMethods {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl TryFrom<Values> for WIMCMethods {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        Self::try_from(value.get_string().ok_or(ParseError::new())?)
            .map_err(|_err| ParseError::new())
    }
}

impl FromStr for WIMCMethods {
    type Err = WIMCError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NAMES
            .iter()
            .position(|&name| name.eq_ignore_ascii_case(s))
            .map(WIMCMethods::try_from)
            .ok_or(WIMCError)?
    }
}

impl TryFrom<String> for WIMCMethods {
    type Error = WIMCError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(value.as_str())
    }
}

impl TryFrom<usize> for WIMCMethods {
    type Error = WIMCError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WIMCMethods::Ping),
            1 => Ok(WIMCMethods::Echo),
            2 => Ok(WIMCMethods::Cleanup),
            3 => Ok(WIMCMethods::Store),
            4 => Ok(WIMCMethods::Get),
            5 => Ok(WIMCMethods::Query),
            _ => Err(WIMCError),
        }
    }
}
