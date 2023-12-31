pub use error::WIMCError;
pub use method::WIMCMethods;
pub use out::WIMCOutput;
pub use r#in::WIMCInput;

mod error;
mod r#in;
mod method;
mod out;
pub mod presets;

pub const ADDRESS: &str = "0.0.0.0";
pub const PORT: i32 = 6380;
pub const DOUBLE_COLON: char = ':';
