use wjp::{map, ParseError, Serialize, SerializeHelper, Values};

use crate::method::WIMCMethods;

pub struct WIMCInput {
    payload: Values,
    params: Vec<String>,
    method: WIMCMethods,
}

impl Serialize for WIMCInput {
    fn serialize(&self) -> Values {
        Values::Struct(map!(
            ("payload", self.payload.clone()),
            ("params", self.params.serialize()),
            ("method", self.method.serialize())
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
        Self {
            payload: obj.serialize(),
            params,
            method,
        }
    }

    pub fn get_method(&self) -> &WIMCMethods {
        &self.method
    }
    pub fn set_method(&mut self, method: WIMCMethods) -> &mut Self {
        self.method = method;
        self
    }
    pub fn get_params(&self) -> &Vec<String> {
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
