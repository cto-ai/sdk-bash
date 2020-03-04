use crate::ux::prompt::executer;
use crate::RequestError;
use serde::Serialize;

/// Public facing Editor
#[derive(Debug, Clone, Serialize)]
pub struct Editor<'a> {
    #[serde(rename = "type")]
    prompt_type: &'a str,
    name: &'a str,
    #[serde(rename = "message")]
    question: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<&'a str>,
}

impl<'a> Editor<'a> {
    /// Return a new Editor
    ///
    /// # Example
    ///
    /// ```norun
    /// use cto_ai::ux::prompt::Editor;
    ///
    /// let editor_val = Editor::new("editor", "Editor")
    ///     .default_value("Just hit ctrl + x")
    ///     .execute();
    /// ```
    pub fn new(name: &'a str, question: &'a str) -> Self {
        Editor {
            prompt_type: "editor",
            name,
            question,
            default: None,
        }
    }
    /// **[Opitional]** Default value to be provided to the editor when opened.
    pub fn default_value(mut self, default: &'a str) -> Self {
        self.default = Some(default);
        self
    }

    /// Executes query based on the values set for Editor
    pub fn execute(self) -> Result<serde_json::Value, RequestError> {
        executer::get_value(self)
    }
}
