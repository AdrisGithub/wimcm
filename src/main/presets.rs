use wbdl::Date;
use wjp::{Serialize, Values};

use crate::method::WIMCMethods;
use crate::out::{WIMCOutput};
use crate::r#in::WIMCInput;
use crate::WIMCError;

const SPACE: char = ' ';
const CLEANUP: WIMCInput = WIMCInput::from_val(Values::Null, Vec::new(), WIMCMethods::Cleanup,None);
const PING: WIMCInput = WIMCInput::from_val(Values::Null, Vec::new(), WIMCMethods::Ping,None);

pub fn echo(msg: &str) -> WIMCInput {
    let vec = msg.split(SPACE).map(String::from).collect();
    WIMCInput::from_val(Values::Null, vec, WIMCMethods::Echo,None)
}

pub const fn ping() -> WIMCInput {
    PING
}

pub fn pong() -> WIMCOutput {
    WIMCOutput::from_values(Values::String(String::from("Pong")))
}

pub fn respond(msg: Vec<String>) -> WIMCOutput {
    let mut string = String::new();
    let len = msg.len();
    for word in msg {
        string.push_str(word.as_str());
        string.push(SPACE);
    }
    if len == 1 {
        string.pop();
    }
    WIMCOutput::from_values(Values::String(string))
}

pub const fn cleanup() -> WIMCInput {
    CLEANUP
}

pub fn store<T: Serialize>(obj: T, mut params: Vec<String>, time: Option<Date>,id: Option<u128>) -> WIMCInput {
    if let Some(time) = time {
        params.push(time.to_string())
    }
    WIMCInput::new(obj, params, WIMCMethods::Store,id)
}
pub fn store_incr<T: Serialize>(obj: T, mut params: Vec<String>, time: Option<Date>) -> WIMCInput{
    if let Some(time) = time {
        params.push(time.to_string())
    }
    WIMCInput::new(obj, params, WIMCMethods::StoreInc,None)
}

pub const fn stored(id: u128) -> WIMCOutput {
    WIMCOutput::from_values(Values::Number(id as f64))
}

pub const fn error(error: WIMCError) -> WIMCOutput {
    WIMCOutput::from_err(error)
}

pub const fn found(values: Values) -> WIMCOutput {
    WIMCOutput::from_values(values)
}

pub const fn get(id: u128) -> WIMCInput {
    WIMCInput::from_val(Values::Number(id as f64), vec![], WIMCMethods::Get,None)
}
pub const fn query(params: Vec<String>) -> WIMCInput {
    WIMCInput::from_val(Values::Null, params, WIMCMethods::Query,None)
}
pub const fn remove(id: u128) -> WIMCInput {
    WIMCInput::from_val(Values::Number(id as f64), vec![], WIMCMethods::Remove,None)
}
