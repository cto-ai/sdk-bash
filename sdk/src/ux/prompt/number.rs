use crate::ux::prompt::executer;
use crate::RequestError;
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
    flag: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<i32>,
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
            flag: "c",
            default: None,
            min: None,
            max: None,
        }
    }

    ///  **[Opitional]** Default value to be provided on the terminal and accepted if the user just presses return.
    pub fn default_value(mut self, default: i32) -> Self {
        self.default = Some(default);
        self
    }

    /// **[Opitional]** The minimum time to permit in the prompt.
    pub fn min(mut self, value: i32) -> Self {
        self.min = Some(value);
        self
    }

    ///  **[Opitional]** The maximum time to permit in the prompt.
    pub fn max(mut self, value: i32) -> Self {
        self.max = Some(value);
        self
    }

    /// Executes query based on the values set for Number
    pub fn execute(self) -> Result<serde_json::Value, RequestError> {
        executer::get_value(self)
    }
}
