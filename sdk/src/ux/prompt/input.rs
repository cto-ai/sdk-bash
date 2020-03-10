use super::Prompt;
use serde::Serialize;

/// Public facing Input
#[derive(Debug, Clone, Serialize)]
pub struct Input<'a> {
    #[serde(rename = "type")]
    prompt_type: &'a str,
    name: &'a str,
    #[serde(rename = "message")]
    question: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<&'a str>,
    allow_empty: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    flag: Option<&'a str>,
}

impl<'a> Prompt<String> for Input<'a> {
    fn name(&self) -> &str {
        self.name
    }
}

impl<'a> Input<'a> {
    /// Return a new Input
    ///
    /// # Example
    ///
    /// ```norun
    /// use cto_ai::ux::prompt::{Prompt, Input};
    ///
    /// let input_val = Input::new("input", "Input").execute();
    /// println!("{}", input_val);
    /// ```
    pub fn new(name: &'a str, question: &'a str) -> Self {
        Input {
            prompt_type: "input",
            name,
            question,
            flag: None,
            default: None,
            allow_empty: false,
        }
    }

    /// Sets a default value to be returned if the user just presses enter
    pub fn default_value(mut self, default: &'a str) -> Self {
        self.default = Some(default);
        self
    }

    /// Configures the prompt to accept an empty string input (by default it will not accept it)
    pub fn allow_empty(mut self) -> Self {
        self.allow_empty = true;
        self
    }

    pub fn flag(mut self, flag: &'a str) -> Self {
        self.flag = Some(flag);
        self
    }
}
