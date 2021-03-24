mod checkbox;
mod confirm;
mod datetime;
mod editor;
mod input;
mod list;
mod number;
mod password;
mod secret;

pub use checkbox::Checkbox;
pub use confirm::Confirm;
pub use datetime::Datetime;
pub use editor::Editor;
pub use input::Input;
pub use list::List;
pub use number::Number;
pub use password::Password;
pub use secret::Secret;

use crate::{daemon, RequestError};
use serde::{de::DeserializeOwned, Serialize};

pub trait Prompt<T: DeserializeOwned>: Serialize + Clone {
    fn name(&self) -> &str;

    fn execute(self) -> Result<T, RequestError> {
        daemon::async_request("prompt", &self, daemon::HttpMethod::POST)
            .and_then(|value| {
                value
                    .get(self.name())
                    .cloned()
                    .ok_or(RequestError::JsonKeyError)
            })
            .and_then(|v| Ok(serde_json::from_value(v)?))
    }
}
