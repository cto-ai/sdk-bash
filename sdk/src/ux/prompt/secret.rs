use crate::ux::prompt::executer;
use crate::RequestError;
use serde::Serialize;

/// Public facing Secret
#[derive(Debug, Clone, Serialize)]
pub struct Secret<'a> {
    #[serde(rename = "type")]
    prompt_type: &'a str,
    name: &'a str,
    #[serde(rename = "message")]
    question: &'a str,
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
        }
    }

    /// Executes query based on the values set for Secret.
    pub fn execute(self) -> Result<serde_json::Value, RequestError> {
        executer::get_value(self)
    }
}
