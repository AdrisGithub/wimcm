use wjp::Values;

use crate::method::WIMCMethods;
use crate::out::WIMCOutput;
use crate::r#in::WIMCInput;

const SPACE: char = ' ';
const CLEANUP: WIMCInput = WIMCInput::from_val(Values::Null, Vec::new(), WIMCMethods::Cleanup);
const PING: WIMCInput = WIMCInput::from_val(Values::Null, Vec::new(), WIMCMethods::Ping);

pub fn echo(msg: String) -> WIMCInput {
    let vec = msg.split(SPACE).map(String::from).collect();
    WIMCInput::from_val(Values::Null, vec, WIMCMethods::Echo)
}

pub const fn ping() -> WIMCInput {
    PING
}

pub fn pong() -> WIMCOutput {
    WIMCOutput::default()
}

pub fn respond(msg: Vec<String>) -> WIMCOutput {
    let mut string = String::new();
    for word in msg {
        string.push(SPACE);
        string.push_str(word.as_str());
    };
    WIMCOutput::from_values(Values::String(string))
}

pub const fn cleanup() -> WIMCInput {
    CLEANUP
}

