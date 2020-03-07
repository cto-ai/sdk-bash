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
use serde::Serialize;

pub trait Prompt: Serialize + Clone {
    fn name(&self) -> &str;

    fn get_value(self) -> Result<serde_json::Value, RequestError> {
        daemon::async_request("prompt", &self).and_then(|value| {
            value
                .get(self.name())
                .cloned()
                .ok_or(RequestError::JsonKeyError)
        })
    }
}
