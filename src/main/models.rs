use std::fmt::{Debug, Display};

use wjp::{Deserialize, ParseError};

pub struct WIMCInput {
    payload: String,
    params: Vec<String>,
    method: WIMCMethods,
}

pub enum WIMCMethods {}

pub struct WIMCError {}

pub struct WIMCOutput(Result<dyn Deserialize<Error=ParseError>, WIMCError>);