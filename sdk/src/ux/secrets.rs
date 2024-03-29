use crate::daemon::async_request;
use crate::RequestError;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetSecret<'a> {
    key: &'a str,
    hidden: bool,
}

impl<'a> GetSecret<'a> {
    pub fn new(key: &'a str) -> Self {
        Self { key, hidden: false }
    }

    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    pub fn execute(self) -> Result<String, RequestError> {
        let name = self.key;
        async_request("secret/get", self)
            .and_then(|value| value.get(name).cloned().ok_or(RequestError::JsonKeyError))
            .and_then(|v| Ok(serde_json::from_value(v)?))
    }
}

/// Gets a secret from the user's vault.
///
/// # Example
/// ```norun
/// use cto_ai::ux::secrets;
///
/// let secret_val = secrets::get("SECRET_NAME").unwrap();
/// println!("{}", secret_val);
/// ```
pub fn get(key: &str) -> Result<String, RequestError> {
    GetSecret::new(key).execute()
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Secrets<'a> {
    #[serde(rename = "key")]
    name: Option<&'a str>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    secret: Option<&'a str>,
}

impl<'a> Secrets<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a secret to the user's vault.
    ///
    /// # Example
    /// ```norun
    /// use cto_ai::ux::secrets;
    ///
    /// let response = secrets::Secrets::new().set("SECRET_NAME", "SECRET_VALUE");
    /// println!("{}", response);
    ///
    /// let secret_val = secret::Secrets::new().get("SECRET_NAME");
    /// println!("{}", secret_val);
    /// ```
    pub fn set(
        mut self,
        name: &'a str,
        secret: &'a str,
    ) -> Result<serde_json::Value, RequestError> {
        self.name = Some(name);
        self.secret = Some(secret);
        async_request("secret/set", self)
            .and_then(|value| value.get("key").cloned().ok_or(RequestError::JsonKeyError))
    }
}
