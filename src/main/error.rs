use std::num::ParseIntError;

#[derive(Debug, Default)]
pub struct WIMCError;

impl From<ParseIntError> for WIMCError {
    fn from(_value: ParseIntError) -> Self {
        Self
    }
}
