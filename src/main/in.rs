use crate::method::WIMCMethods;

pub struct WIMCInput {
    payload: String,
    params: Vec<String>,
    method: WIMCMethods,
}