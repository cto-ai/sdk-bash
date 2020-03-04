use crate::daemon::async_request;
use crate::RequestError;
use serde::Serialize;

pub fn get_value<T: Serialize>(data: T) -> Result<serde_json::Value, RequestError> {
    async_request("prompt", data)
}
