use super::Prompt;
use serde::Serialize;

static LIST_TYPE: &str = "list";
static AUTOCOMPLETE_TYPE: &str = "autocomplete";

/// Public facing List
#[derive(Debug, Clone, Serialize)]
pub struct List<'a> {
    #[serde(rename = "type")]
    prompt_type: &'a str,
    name: &'a str,
    #[serde(rename = "message")]
    question: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    flag: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<&'a str>,
    choices: Vec<String>,
}

impl<'a> Prompt<String> for List<'a> {
    fn name(&self) -> &str {
        self.name
    }
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
            prompt_type: LIST_TYPE,
            name,
            question,
            flag: None,
            default: None,
            choices,
        }
    }

    /// **[Optional]** Default value to be provided on the terminal and accepted if the user just presses return.
    pub fn default_value(mut self, default: &'a str) -> Self {
        self.default = Some(default);
        self
    }

    pub fn autocomplete(mut self) -> Self {
        self.prompt_type = AUTOCOMPLETE_TYPE;
        self
    }

    pub fn flag(mut self, flag: &'a str) -> Self {
        self.flag = Some(flag);
        self
    }
}
