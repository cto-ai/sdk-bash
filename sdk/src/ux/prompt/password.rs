use crate::ux::prompt::executer;
use crate::RequestError;
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
    pub fn execute(self) -> Result<serde_json::Value, RequestError> {
        executer::get_value(self)
    }
}
