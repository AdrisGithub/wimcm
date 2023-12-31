use wbdl::Date;
use wjp::{Serialize, Values};

use crate::method::WIMCMethods;
use crate::out::WIMCOutput;
use crate::r#in::WIMCInput;
use crate::WIMCError;

const SPACE: char = ' ';
const CLEANUP: WIMCInput = WIMCInput::from_val(Values::Null, Vec::new(), WIMCMethods::Cleanup);
const PING: WIMCInput = WIMCInput::from_val(Values::Null, Vec::new(), WIMCMethods::Ping);

pub fn echo(msg: &str) -> WIMCInput {
    let vec = msg.split(SPACE).map(String::from).collect();
    WIMCInput::from_val(Values::Null, vec, WIMCMethods::Echo)
}

pub const fn ping() -> WIMCInput {
    PING
}

pub fn pong() -> WIMCOutput {
    respond(vec![String::from("Pong")])
}

pub fn respond(msg: Vec<String>) -> WIMCOutput {
    let mut string = String::new();
    let mut first = true;
    for word in msg {
        if first {
            first = false;
        } else {
            string.push(SPACE);
        }
        string.push_str(word.as_str());
    };
    WIMCOutput::from_values(Values::String(string))
}

pub const fn cleanup() -> WIMCInput {
    CLEANUP
}

pub fn store<T: Serialize>(obj: T, mut params: Vec<String>, time: Option<Date>) -> WIMCInput {
    if let Some(time) = time {
        params.push(time.to_string())
    }
    WIMCInput::new(obj, params, WIMCMethods::Store)
}

pub fn stored(id: usize) -> WIMCOutput {
    WIMCOutput::from_values(Values::Number(id as f64))
}

pub fn error(error: WIMCError) -> WIMCOutput {
    WIMCOutput::from(error)
}

pub fn found(values: Values) -> WIMCOutput {
    WIMCOutput::from_values(values)
}

pub fn get(id: usize) -> WIMCInput {
    WIMCInput::from_val(Values::Null, vec![id.to_string()], WIMCMethods::Get)
}

pub fn query(params: Vec<String>) -> WIMCInput {
    WIMCInput::from_val(Values::Null, params, WIMCMethods::Query)
}
