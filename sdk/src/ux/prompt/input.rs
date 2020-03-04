use crate::ux::prompt::executer;
use crate::RequestError;
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
    flag: &'a str,
}

impl<'a> Input<'a> {
    /// Return a new Input
    ///
    /// # Example
    ///
    /// ```norun
    /// use cto_ai::ux::prompt::Input;
    ///
    /// let input_val = Input::new("input", "Input").execute();
    /// println!("{}", input_val);
    /// ```
    pub fn new(name: &'a str, question: &'a str) -> Self {
        Input {
            prompt_type: "input",
            name,
            question,
            flag: "o",
            default: None,
            allow_empty: false,
        }
    }

    // **[Opitional]** Default value to be provided on the terminal and accepted if the user just presses return.
    pub fn default_value(mut self, default: &'a str) -> Self {
        self.default = Some(default);
        self
    }

    // **[Opitional]** Allows input to be emmpy.
    pub fn allow_empty(mut self) -> Self {
        self.allow_empty = true;
        self
    }

    /// Executes query based on the values set for Input.
    pub fn execute(self) -> Result<serde_json::Value, RequestError> {
        executer::get_value(self)
    }
}
