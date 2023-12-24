use wjp::{map, ParseError, Serialize, SerializeHelper, Values};

use crate::method::WIMCMethods;

pub struct WIMCInput {
    payload: String,
    params: Vec<String>,
    method: WIMCMethods,
}

impl Serialize for WIMCInput {
    fn serialize(&self) -> Values {
        Values::Struct(map!(
            ("payload", self.payload.serialize()),
            ("params", self.params.serialize()),
            ("method", self.method.serialize())
        ))
    }
}

impl TryFrom<Values> for WIMCInput {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let mut struc = value.get_struct().ok_or(ParseError::new())?;
        let payload = struc.map_val("payload", String::try_from)?;
        let params = struc.map_val("params", Vec::try_from)?;
        let method = struc.map_val("method", WIMCMethods::try_from)?;
        Ok(Self {
            method,
            params,
            payload,
        })
    }
}
