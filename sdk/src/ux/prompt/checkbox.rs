use crate::ux::prompt::executer;
use crate::RequestError;

use serde::Serialize;

/// Public facing Checkbox
#[derive(Debug, Clone, Serialize)]
pub struct Checkbox<'a> {
    #[serde(rename = "type")]
    prompt_type: &'a str,
    name: &'a str,
    #[serde(rename = "message")]
    question: &'a str,
    choices: Vec<String>,
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
        }
    }

    /// Executes query based on the values set for Checkbox
    pub fn execute(self) -> Result<serde_json::Value, RequestError> {
        executer::get_value(self)
    }
}
