use wjp::{Deserialize, map, ParseError, Serialize, SerializeHelper, Values};

use crate::method::WIMCMethods;
#[derive(Clone,Debug)]
pub struct WIMCInput {
    payload: Values,
    params: Vec<String>,
    method: WIMCMethods,
}

impl Serialize for WIMCInput {
    fn serialize(&self) -> Values {
        Values::Struct(map!(
            ("payload", &self.payload),
            ("params", &self.params),
            ("method", &self.method)
        ))
    }
}

impl Default for WIMCInput {
    fn default() -> Self {
        Self {
            payload: Values::Null,
            params: Vec::default(),
            method: WIMCMethods::default(),
        }
    }
}

impl WIMCInput {
    pub fn new<T: Serialize>(obj: T, params: Vec<String>, method: WIMCMethods) -> Self {
        Self::from_val(obj.serialize(), params, method)
    }
    pub const fn from_val(obj: Values, params: Vec<String>, method: WIMCMethods) -> Self {
        Self {
            params,
            method,
            payload: obj,
        }
    }

    pub const fn get_method(&self) -> &WIMCMethods {
        &self.method
    }
    pub fn set_method(&mut self, method: WIMCMethods) -> &mut Self {
        self.method = method;
        self
    }
    pub const fn get_params(&self) -> &Vec<String> {
        &self.params
    }
    pub fn set_params(&mut self, params: Vec<String>) -> &mut Self {
        self.params = params;
        self
    }
    pub fn insert_params(&mut self, param: String, index: usize) -> &mut Self {
        self.params.insert(index, param);
        self
    }
    pub fn add_param(&mut self, param: String) -> &mut Self {
        self.params.push(param);
        self
    }
    pub fn mutate_params(mut self, func: fn(Vec<String>) -> Vec<String>) -> Self {
        self.params = func(self.params);
        self
    }
    pub fn get_payload(&self) -> &Values {
        &self.payload
    }
    pub fn get_parsed_payload<T: Deserialize>(&self) -> Result<T, ParseError> {
        T::deserialize_str(self.payload.to_string().as_str())
    }
    pub fn set_payload(&mut self, values: Values) -> &mut Self {
        self.payload = values;
        self
    }
    pub fn set_parsed_payload<T: Serialize>(&mut self, obj: T) -> &mut Self {
        self.payload = obj.serialize();
        self
    }
}

impl TryFrom<Values> for WIMCInput {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let mut struc = value.get_struct().ok_or(ParseError::new())?;
        let payload = struc.remove("payload").ok_or(ParseError::new())?;
        let params = struc.map_val("params", Vec::try_from)?;
        let method = struc.map_val("method", WIMCMethods::try_from)?;
        Ok(Self {
            method,
            params,
            payload,
        })
    }
}
