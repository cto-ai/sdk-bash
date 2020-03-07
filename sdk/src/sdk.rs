use crate::daemon::simple_request;
use crate::RequestError;
use serde::Serialize;
use std::collections::HashMap;
use std::env;

pub fn get_state_path() -> String {
    env::var("SDK_STATE_DIR").expect("SDK_STATE_DIR environment variable not set")
}

pub fn get_config_path() -> String {
    env::var("SDK_CONFIG_DIR").expect("SDK_CONFIG_DIR environment variable not set")
}

pub fn get_home_path() -> String {
    env::var("SDK_HOME_DIR").unwrap_or_else(|_| "/root".to_owned())
}

pub fn get_host_os() -> String {
    env::var("OPS_HOST_PLATFORM").unwrap_or_else(|_| "unknown".to_owned())
}

pub fn get_interface_type() -> String {
    env::var("SDK_INTERFACE_TYPE").unwrap_or_else(|_| "terminal".to_owned())
}

#[derive(Debug, Clone, Serialize)]
pub struct Track<'a> {
    #[serde(default)]
    tags: Vec<&'a str>,
    event: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<&'a str>,
    #[serde(flatten)]
    metadata: HashMap<&'a str, serde_json::Value>,
}

impl<'a> Track<'a> {
    pub fn new(event: &'a str) -> Self {
        Self {
            tags: Default::default(),
            event,
            error: None,
            metadata: HashMap::new(),
        }
    }

    pub fn error(mut self, err: &'a str) -> Self {
        self.error = Some(err);
        self
    }

    pub fn tag(mut self, tag: &'a str) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn tags(mut self, tags: &[&'a str]) -> Self {
        self.tags.extend_from_slice(tags);
        self
    }

    pub fn metadata(mut self, key: &'a str, value: impl Into<serde_json::Value>) -> Self {
        self.metadata.insert(key, value.into());
        self
    }

    pub fn send(self) -> Result<(), RequestError> {
        simple_request("track", self)
    }
}
