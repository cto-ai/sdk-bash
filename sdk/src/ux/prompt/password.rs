use super::Prompt;
use crate::{daemon::stringify, RequestError};
use serde::Serialize;

/// Public facing Password
#[derive(Debug, Clone, Serialize)]
pub struct Password<'a> {
    #[serde(rename = "type")]
    prompt_type: &'a str,
    name: &'a str,
    #[serde(rename = "message")]
    question: &'a str,
    confirm: bool,
}

impl<'a> Prompt for Password<'a> {
    fn name(&self) -> &str {
        self.name
    }
}

impl<'a> Password<'a> {
    /// Return a new Password
    ///
    /// # Example
    ///
    /// ```norun
    /// use cto_ai::ux::prompt::Password;
    ///
    /// let password_val = Password::new("password", "Password").execute();
    /// println!("{}", password_val);
    /// ```
    pub fn new(name: &'a str, question: &'a str) -> Self {
        Password {
            prompt_type: "password",
            name,
            question,
            confirm: false,
        }
    }

    /// **[Opitional]** Asks the user for password confimation
    pub fn confirm(mut self) -> Self {
        self.confirm = true;
        self
    }

    /// Executes query based on the values set for Password.
    pub fn execute(self) -> Result<String, RequestError> {
        self.get_value().and_then(stringify)
    }
}
