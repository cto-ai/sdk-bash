//! The SDK `sdk` module, just like other SDKs

use crate::daemon::{simple_request, sync_request};
use crate::RequestError;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;
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

pub fn get_all_state() -> Result<HashMap<String, serde_json::Value>, RequestError> {
    Ok(serde_json::from_value(sync_request(
        "state/get-all",
        json!({}),
    )?)?)
}

pub fn get_all_config() -> Result<HashMap<String, String>, RequestError> {
    Ok(serde_json::from_value(sync_request(
        "config/get-all",
        json!({}),
    )?)?)
}

/// A request body with a key in it
#[derive(Debug, Clone, Serialize)]
struct KeyBody<'a> {
    key: &'a str,
}

pub fn get_state<T: DeserializeOwned>(key: &str) -> Result<T, RequestError> {
    Ok(serde_json::from_value(sync_request(
        "state/get",
        KeyBody { key },
    )?)?)
}

pub fn get_config(key: &str) -> Result<Option<String>, RequestError> {
    Ok(serde_json::from_value(sync_request(
        "config/get",
        KeyBody { key },
    )?)?)
}

/// A request body with a value in it
#[derive(Debug, Clone, Serialize)]
struct KeyValueBody<'a> {
    key: &'a str,
    value: serde_json::Value,
}

pub fn set_state(key: &str, value: impl Into<serde_json::Value>) -> Result<(), RequestError> {
    simple_request(
        "state/set",
        KeyValueBody {
            key,
            value: value.into(),
        },
    )
}

pub fn set_config(key: &str, value: impl Into<serde_json::Value>) -> Result<(), RequestError> {
    simple_request(
        "config/set",
        KeyValueBody {
            key,
            value: value.into(),
        },
    )
}

pub fn delete_config(key: &str) -> Result<bool, RequestError> {
    Ok(serde_json::from_value(sync_request(
        "config/delete",
        KeyBody { key },
    )?)?)
}

#[derive(Debug, Clone, Serialize)]
struct EventsBody<'a> {
    start: &'a str,
    end: &'a str,
}

pub fn events(start: &str, end: &str) -> Result<Vec<serde_json::Value>, RequestError> {
    Ok(serde_json::from_value(sync_request(
        "events",
        EventsBody { start, end },
    )?)?)
}

#[derive(Debug, Clone, Serialize)]
struct EventsBody<'a> {
    start: &'a str,
    end: &'a str,
}

pub fn events(start: &str, end: &str) -> Result<Vec<serde_json::Value>, RequestError> {
    Ok(serde_json::from_value(sync_request(
        "events",
        EventsBody { start, end },
    )?)?)
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

#[derive(Debug, Clone, Serialize)]
pub struct StartOp<'a> {
    #[serde(rename(serialize = "workflowName"))]
    workflow_name: &'a str,
    trigger: bool
}

impl<'a> StartOp<'a> {
    pub fn new(workflow_name: &'a str) -> Self {
        Self {
            workflow_name,
            trigger: true
        }
    }

    pub fn send(self) -> Result<(), RequestError> {
        simple_request("track", self)
    }
}