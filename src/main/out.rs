use wjp::{Deserialize, map, ParseError, Serialize, Values};

use crate::error::WIMCError;

#[derive(Clone,Debug)]
pub struct WIMCOutput(Result<Values, WIMCError>);

impl Serialize for WIMCOutput {
    fn serialize(&self) -> Values {
        self.clone().serialize()
    }
}

impl Default for WIMCOutput {
    fn default() -> Self {
        Self(Ok(Values::String(String::from("Pong"))))
    }
}

impl TryFrom<Values> for WIMCOutput {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let mut struc = value.get_struct().ok_or(ParseError::new())?;
        let error = struc.remove("error").ok_or(ParseError::new())?;
        let value = struc.remove("result").ok_or(ParseError::new())?;
        Ok(if error.is_null() {
            Self::from_values(value)
        } else {
            Self::from(WIMCError::try_from(error)?)
        })
    }
}

impl From<WIMCError> for WIMCOutput {
    fn from(value: WIMCError) -> Self {
        Self(Err(value))
    }
}

impl WIMCOutput {
    pub const fn from_values(val: Values) -> Self {
        Self(Ok(val))
    }
    pub const fn is_error(&self) -> bool {
        self.0.is_err()
    }
    pub const fn is_okay(&self) -> bool {
        !self.is_error()
    }
    pub fn ok(self) -> Option<Values> {
        self.0.ok()
    }
    pub fn deserialize<T: Deserialize>(self) -> Result<T, ParseError> {
        match self.0 {
            Ok(v) => T::deserialize(v.to_string()),
            Err(_) => Err(ParseError::new()),
        }
    }
    pub fn err(self) -> Option<WIMCError> {
        self.0.err()
    }
    pub fn map_err<E, O: FnOnce(WIMCError) -> E>(self, op: O) -> Result<Values, E> {
        self.0.map_err(op)
    }
    pub fn map_ok<U, F: FnOnce(Values) -> U>(self, op: F) -> Result<U, WIMCError> {
        self.0.map(op)
    }
    fn serialize(self) -> Values {
        match self.0 {
            Ok(val) => Values::Struct(map!(("result", &val), ("error", &Values::Null))),
            Err(err) => Values::Struct(map!(("result", &Values::Null), ("error", &err))),
        }
    }
}
