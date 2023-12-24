pub use error::WIMCError;
pub use method::WIMCMethods;
pub use out::WIMCOutput;
pub use r#in::WIMCInput;

mod error;
mod r#in;
mod method;
mod out;
pub mod presets;