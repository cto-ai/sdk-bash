use super::Prompt;
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
    flag: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<&'a str>,
}

impl<'a> Prompt<String> for Editor<'a> {
    fn name(&self) -> &str {
        self.name
    }
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
            flag: None,
            default: None,
        }
    }
    /// **[Optional]** Default value to be provided to the editor when opened.
    pub fn default_value(mut self, default: &'a str) -> Self {
        self.default = Some(default);
        self
    }

    pub fn flag(mut self, flag: &'a str) -> Self {
        self.flag = Some(flag);
        self
    }
}
