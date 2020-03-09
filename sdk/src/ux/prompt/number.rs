use super::Prompt;
use crate::{daemon::numberize, RequestError};
use serde::Serialize;

/// Public facing Number
#[derive(Debug, Clone, Serialize)]
pub struct Number<'a> {
    #[serde(rename = "type")]
    prompt_type: &'a str,
    name: &'a str,
    #[serde(rename = "message")]
    question: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flag: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<i32>,
}

impl<'a> Prompt for Number<'a> {
    fn name(&self) -> &str {
        self.name
    }
}

impl<'a> Number<'a> {
    /// Returns a new Number
    ///
    /// # Example
    ///
    /// ```norun
    /// use cto_ai::ux::prompt::Number;
    ///
    /// let number_val = Number::new("number", "Number").min(1).max(20).execute();
    /// println!("{}", number_val);
    /// ```
    pub fn new(name: &'a str, question: &'a str) -> Self {
        Number {
            prompt_type: "number",
            name,
            question,
            flag: None,
            default: None,
            min: None,
            max: None,
        }
    }

    ///  **[Optional]** Default value to be provided on the terminal and accepted if the user just presses return.
    pub fn default_value(mut self, default: i32) -> Self {
        self.default = Some(default);
        self
    }

    /// **[Optional]** The minimum time to permit in the prompt.
    pub fn min(mut self, value: i32) -> Self {
        self.min = Some(value);
        self
    }

    ///  **[Optional]** The maximum time to permit in the prompt.
    pub fn max(mut self, value: i32) -> Self {
        self.max = Some(value);
        self
    }

    pub fn flag(mut self, flag: &'a str) -> Self {
        self.flag = Some(flag);
        self
    }

    /// Executes query based on the values set for Number
    pub fn execute(self) -> Result<i64, RequestError> {
        self.get_value().and_then(numberize)
    }
}
