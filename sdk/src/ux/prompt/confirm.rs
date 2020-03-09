use super::Prompt;
use crate::{daemon::booleanize, RequestError};
use serde::Serialize;

/// Public facing Confirm
#[derive(Debug, Clone, Serialize)]
pub struct Confirm<'a> {
    #[serde(rename = "type")]
    prompt_type: &'a str,
    name: &'a str,
    #[serde(rename = "message")]
    question: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flag: Option<&'a str>,
}

impl<'a> Prompt for Confirm<'a> {
    fn name(&self) -> &str {
        self.name
    }
}

impl<'a> Confirm<'a> {
    /// Return a new Confirm
    ///
    /// # Example
    ///
    /// ```norun
    /// use cto_ai::ux::prompt::Confirm;
    ///
    /// let confirm_val = Confirm::new("confirm", "Confirm").execute();
    /// println!("{}", confirm_val);
    /// ```
    pub fn new(name: &'a str, question: &'a str) -> Self {
        Confirm {
            prompt_type: "confirm",
            name,
            question,
            flag: None,
            default: None,
        }
    }

    /// **[Optional]** Default value to be provided on the terminal and accepted if the user just presses return.
    pub fn default_value(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }

    pub fn flag(mut self, flag: &'a str) -> Self {
        self.flag = Some(flag);
        self
    }

    /// Executes query based on the values set for Confirm.
    pub fn execute(self) -> Result<bool, RequestError> {
        self.get_value().and_then(booleanize)
    }
}
