use std::fmt::{Debug, Display, Formatter};

use wjp::{Serialize, Values};

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Default, Debug)]
pub enum WIMCMethods {
    #[default]
    Ping,
    Echo,
    Cleanup,
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