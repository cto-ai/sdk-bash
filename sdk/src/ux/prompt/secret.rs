use super::Prompt;
use serde::Serialize;

/// Public facing Secret
#[derive(Debug, Clone, Serialize)]
pub struct Secret<'a> {
    #[serde(rename = "type")]
    prompt_type: &'a str,
    name: &'a str,
    #[serde(rename = "message")]
    question: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    flag: Option<&'a str>,
}

impl<'a> Prompt<String> for Secret<'a> {
    fn name(&self) -> &str {
        self.name
    }
}

impl<'a> Secret<'a> {
    /// Return a new Secret
    ///
    /// # Example
    ///
    /// ```norun
    /// use cto_ai::ux::prompt::Secret;
    ///
    /// let secret_val = Secret::new("my_secret", "Secret").execute();
    /// println!("{}", secret_val);
    /// ```
    pub fn new(name: &'a str, question: &'a str) -> Self {
        Secret {
            prompt_type: "secret",
            name,
            question,
            flag: None,
        }
    }

    pub fn flag(mut self, flag: &'a str) -> Self {
        self.flag = Some(flag);
        self
    }
}
