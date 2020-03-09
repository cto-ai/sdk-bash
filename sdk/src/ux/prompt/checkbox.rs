use super::Prompt;
use crate::{daemon::arrayize, RequestError};
use serde::Serialize;

/// Public facing Checkbox
#[derive(Debug, Clone, Serialize)]
pub struct Checkbox<'a> {
    #[serde(rename = "type")]
    prompt_type: &'a str,
    name: &'a str,
    #[serde(rename = "message")]
    question: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    flag: Option<&'a str>,
    choices: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<Vec<String>>,
}

impl<'a> Prompt for Checkbox<'a> {
    fn name(&self) -> &str {
        self.name
    }
}

impl<'a> Checkbox<'a> {
    /// Return a new Checkbox
    ///
    /// # Example
    ///
    /// ```norun
    /// use cto_ai::ux::prompt::Checkbox;
    ///
    /// let checkbox_val = Checkbox::new(
    ///     "my_checkbox",
    ///     "Checkbox",
    ///     vec!["foo".to_owned(), "bar".to_owned()],
    /// )
    /// .execute();
    ///
    ///println!("{}", checkbox_val);
    /// ```
    pub fn new(name: &'a str, question: &'a str, choices: Vec<String>) -> Self {
        Checkbox {
            prompt_type: "checkbox",
            name,
            question,
            choices,
            default: None,
            flag: None,
        }
    }

    pub fn default(mut self, default: Vec<String>) -> Self {
        self.default = Some(default);
        self
    }

    pub fn flag(mut self, flag: &'a str) -> Self {
        self.flag = Some(flag);
        self
    }

    /// Executes query based on the values set for Checkbox
    pub fn execute(self) -> Result<Vec<String>, RequestError> {
        self.get_value().and_then(arrayize)
    }
}
