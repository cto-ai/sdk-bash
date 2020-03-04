use crate::ux::prompt::executer;
use crate::RequestError;
use serde::Serialize;

/// Public facing Autocomplete
#[derive(Debug, Clone, Serialize)]
pub struct Autocomplete<'a> {
    #[serde(rename = "type")]
    prompt_type: &'a str,
    name: &'a str,
    #[serde(rename = "message")]
    question: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<&'a str>,
    choices: Vec<String>,
}

impl<'a> Autocomplete<'a> {
    /// Returns a new Autocomplete
    ///
    /// # Example
    ///
    /// ```norun
    /// use cto_ai::ux::prompt::Autocomplete;
    ///
    /// Autocomplete::new(
    ///   "my_autocomplete",
    ///   "Autocomplete",
    ///   vec!["foo".to_owned(), "bar".to_owned()],
    /// ).execute();
    /// ```
    pub fn new(name: &'a str, question: &'a str, choices: Vec<String>) -> Self {
        Autocomplete {
            prompt_type: "autocomplete",
            name,
            question,
            default: None,
            choices,
        }
    }

    /// Sets default value to Autocomplete
    ///
    /// # Example
    ///
    /// ```norun
    /// use cto_ai::ux::prompt::Autocomplete;
    ///
    /// Autocomplete::new(
    ///   "my_autocomplete",
    ///   "Autocomplete",
    ///   vec!["foo".to_owned(), "bar".to_owned()],
    /// )
    ///     .default_value("Some Default Value")
    ///     .execute();
    /// ```
    pub fn default_value(mut self, default: &'a str) -> Self {
        self.default = Some(default);
        self
    }

    /// Executes query based on the values set for Autocomplete
    pub fn execute(self) -> Result<serde_json::Value, RequestError> {
        executer::get_value(self)
    }
}
