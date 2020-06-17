use super::Prompt;
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
    default: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variant: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<&'a str>,
}

impl<'a> Prompt<String> for Datetime<'a> {
    fn name(&self) -> &str {
        self.name
    }
}

impl<'a> Datetime<'a> {
    /// Returns a new Datetime prompt
    ///
    /// # Example
    ///
    /// ```norun
    /// use chrono::Utc;
    /// use cto_ai::ux::prompt::Datetime;
    ///
    /// let dt_val = Datetime::new("date", "DateTime")
    ///    .variant("time")
    ///    .minimum(Utc::now())
    ///    .maximum(Utc::now())
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
            minimum: None,
            maximum: None,
        }
    }

    /// Optional default
    ///
    /// Sets the  time to initialize the prompt with. If not specified, will default
    /// to the current time.
    pub fn default_value(mut self, default: &'a str) -> Self {
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

    /// Optional minimum
    ///
    /// The minimum time to permit in the prompt.
    pub fn minimum(mut self, value: &'a str) -> Self {
        self.minimum = Some(value);
        self
    }

    /// Optional maximum
    ///
    /// The maximum time to permit in the prompt.
    pub fn maximum(mut self, value: &'a str) -> Self {
        self.maximum = Some(value);
        self
    }

    pub fn flag(mut self, flag: &'a str) -> Self {
        self.flag = Some(flag);
        self
    }
}
