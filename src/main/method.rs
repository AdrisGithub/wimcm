#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug,Default)]
pub enum WIMCMethods {
    #[default]
    Ping,
    Echo,
    Cleanup,
}