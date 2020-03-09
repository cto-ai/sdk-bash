use super::Prompt;
use crate::{daemon::stringify, RequestError};
use chrono::{DateTime, Utc};
use serde::Serialize;

/// Public facing Datetime
#[derive(Debug, Clone, Serialize)]
pub struct Datetime<'a> {
    #[serde(rename = "type")]
    prompt_type: &'a str,
    name: &'a str,
    #[serde(rename = "message")]
    question: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    flag: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variant: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<DateTime<Utc>>,
}

impl<'a> Prompt for Datetime<'a> {
    fn name(&self) -> &str {
        self.name
    }
}

impl<'a> Datetime<'a> {
    /// Returtns a new Datetime
    ///
    /// # Example
    ///
    /// ```norun
    /// use chrono::Utc;
    /// use cto_ai::ux::prompt::Datetime;
    ///
    /// let dt_val = Datetime::new("date", "DateTime")
    ///    .variant("time")
    ///    .min(Utc::now())
    ///    .max(Utc::now())
    ///    .execute();
    ///
    /// println!("{}", dt_val);
    /// ```
    pub fn new(name: &'a str, question: &'a str) -> Self {
        Datetime {
            prompt_type: "datetime",
            name,
            question,
            flag: None,
            variant: None,
            default: None,
            min: None,
            max: None,
        }
    }

    /// Optional default
    ///
    /// Sets the  time to initialize the prompt with. If not specified, will default
    /// to the current time.
    pub fn default_value(mut self, default: DateTime<Utc>) -> Self {
        self.default = Some(default);
        self
    }

    /// Optional variant
    ///
    /// `("date" | "time" | "datetime" )` specifies which time information to prompt for, either
    ///a date (day/month/year) or a time (hour/minute/second), or both. Default is "datetime".
    pub fn variant(mut self, value: &'a str) -> Self {
        self.variant = Some(value);
        self
    }

    /// Optional min
    ///
    /// The minimum time to permit in the prompt.
    pub fn min(mut self, value: DateTime<Utc>) -> Self {
        self.min = Some(value);
        self
    }

    /// Optional max
    ///
    /// The maximum time to permit in the prompt.
    pub fn max(mut self, value: DateTime<Utc>) -> Self {
        self.max = Some(value);
        self
    }

    pub fn flag(mut self, flag: &'a str) -> Self {
        self.flag = Some(flag);
        self
    }

    /// Executes query based on the values set for Datetime
    pub fn execute(self) -> Result<String, RequestError> {
        self.get_value().and_then(stringify)
    }
}
