use std::u128;
use wjp::{Deserialize, map, ParseError, Serialize, SerializeHelper, Values};

use crate::error::WIMCError;

#[derive(Clone, Debug, PartialEq)]
pub struct WIMCOutput(Result<WIMCOut, WIMCError>);

#[derive(Clone, Debug, PartialEq)]
pub struct WIMCOut {
    payload: Values,
    id: Option<u128>,
}


impl From<(Values, u128)> for WIMCOut {
    fn from(value: (Values, u128)) -> Self {
        Self {
            payload: value.0,
            id: Some(value.1),
        }
    }
}

impl Serialize for WIMCOut {
    fn serialize(&self) -> Values {
        Values::Struct(map!(
            ("payload",&self.payload),
            ("id",&self.id)
        ))
    }
}

impl TryFrom<Values> for WIMCOut {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let mut struc = value.get_struct().ok_or(ParseError::new())?;
        let id = struc.map_val("id", u128::try_from).ok();
        let payload = struc.remove("payload").ok_or(ParseError::new())?;
        Ok(Self {
            payload,
            id,
        })
    }
}

impl Serialize for WIMCOutput {
    fn serialize(&self) -> Values {
        self.clone().serialize()
    }
}


impl Default for WIMCOutput {
    fn default() -> Self {
        Self(Ok(WIMCOut::default()))
    }
}

impl Default for WIMCOut {
    fn default() -> Self {
        Self {
            payload: Values::String(String::from("Pong")),
            id: None,
        }
    }
}

impl TryFrom<Values> for WIMCOutput {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let mut struc = value.get_struct().ok_or(ParseError::new())?;
        let error = struc.remove("error").ok_or(ParseError::new())?;
        let value = struc.remove("result").ok_or(ParseError::new())?;
        Ok(if error.is_null() {
            Self::from_out(WIMCOut::try_from(value)?)
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


impl WIMCOut {
    pub fn deserialize<T: Deserialize>(self) -> Result<(T, Option<u128>), ParseError> {
        let t = T::deserialize_str(self.payload.to_string().as_str())?;
        Ok((t, self.id))
    }
    const fn from(value: Values) -> Self {
        Self::new(value,None)
    }
    const fn new(value:Values,id: Option<u128>) -> Self{
        Self{
            payload: value,
            id
        }
    }
}

impl WIMCOutput {
    pub const fn from_values(val: Values) -> Self {
        Self::from_out(WIMCOut::from(val))
    }
    const fn from_out(value: WIMCOut) -> Self {
        Self(Ok(value))
    }
    pub const fn from_err(err: WIMCError) -> Self {
        Self(Err(err))
    }
    pub const fn is_error(&self) -> bool {
        self.0.is_err()
    }
    pub const fn is_okay(&self) -> bool {
        !self.is_error()
    }
    pub fn ok(self) -> Option<WIMCOut> {
        self.0.ok()
    }
    pub fn deserialize<T: Deserialize>(self) -> Result<(T, Option<u128>), ParseError> {
        match self.0 {
            Ok(v) => v.deserialize(),
            Err(_) => Err(ParseError::new()),
        }
    }
    pub fn err(self) -> Option<WIMCError> {
        self.0.err()
    }
    pub fn map_err<E, O: FnOnce(WIMCError) -> E>(self, op: O) -> Result<WIMCOut, E> {
        self.0.map_err(op)
    }
    pub fn map_ok<U, F: FnOnce(WIMCOut) -> U>(self, op: F) -> Result<U, WIMCError> {
        self.0.map(op)
    }
    fn serialize(self) -> Values {
        match self.0 {
            Ok(val) => Values::Struct(map!(("result", &val), ("error", &Values::Null))),
            Err(err) => Values::Struct(map!(("result", &Values::Null), ("error", &err))),
        }
    }
}
#[cfg(test)]
mod tests{
    use wjp::{Serialize, Values};
    use crate::out::WIMCOut;
    use crate::WIMCOutput;

    #[test]
    pub fn test(){
        let obj = WIMCOutput::from_out(WIMCOut::new(Values::Null,Some(1)));
        let ser = obj.json();
        println!("{}",ser);
    }
}