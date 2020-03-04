use crate::ux::prompt::executer;
use crate::RequestError;
use serde::Serialize;

/// Public facing List
#[derive(Debug, Clone, Serialize)]
pub struct List<'a> {
    #[serde(rename = "type")]
    prompt_type: &'a str,
    name: &'a str,
    #[serde(rename = "message")]
    question: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<&'a str>,
    choices: Vec<String>,
}

impl<'a> List<'a> {
    /// Return a new List
    ///
    /// # Example
    ///
    /// ```norun
    /// use cto_ai::ux::prompt::List;
    ///
    /// let list_val = List::new("list", "List", vec!["foo".to_owned(), "bar".to_owned()]).execute();
    /// println!("{}", list_val);
    /// ```
    pub fn new(name: &'a str, question: &'a str, choices: Vec<String>) -> Self {
        List {
            prompt_type: "list",
            name,
            question,
            default: None,
            choices,
        }
    }

    /// **[Opitional]** Default value to be provided on the terminal and accepted if the user just presses return.
    pub fn default_value(mut self, default: &'a str) -> Self {
        self.default = Some(default);
        self
    }

    /// Executes query based on the values set for List.
    pub fn execute(self) -> Result<serde_json::Value, RequestError> {
        executer::get_value(self)
    }
}
