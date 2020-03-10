use std::env;
use std::fs::File;
use std::io::BufReader;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::RequestError;

fn make_request<T: Serialize>(endpoint: &str, body: T) -> Result<reqwest::Response, RequestError> {
    let port = env::var("SDK_SPEAK_PORT").expect("SDK_SPEAK_PORT environment variable not set");

    Ok(Client::new()
        .post(&format!("http://localhost:{}/{}", port, endpoint))
        .json(&body)
        .send()?
        .error_for_status()?)
}

fn read_fifo(filename: String) -> Result<serde_json::Value, RequestError> {
    let file = File::open(&filename)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

pub fn simple_request(endpoint: &str, body: impl Serialize) -> Result<(), RequestError> {
    make_request(endpoint, body)?;
    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
struct ValueResponse {
    value: serde_json::Value,
}

pub fn sync_request(
    endpoint: &str,
    body: impl Serialize,
) -> Result<serde_json::Value, RequestError> {
    Ok(make_request(endpoint, body)?.json::<ValueResponse>()?.value)
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct DaemonResponse {
    reply_filename: String,
}

pub fn async_request(
    endpoint: &str,
    body: impl Serialize,
) -> Result<serde_json::Value, RequestError> {
    let filename = make_request(endpoint, body)?
        .json::<DaemonResponse>()?
        .reply_filename;

    Ok(read_fifo(filename)?)
}

pub fn stringify(value: serde_json::Value) -> Result<String, RequestError> {
    value
        .as_str()
        .map(str::to_owned)
        .ok_or(RequestError::BadTypeError)
}

pub fn numberize(value: serde_json::Value) -> Result<i64, RequestError> {
    value.as_i64().ok_or(RequestError::BadTypeError)
}

pub fn booleanize(value: serde_json::Value) -> Result<bool, RequestError> {
    value.as_bool().ok_or(RequestError::BadTypeError)
}

pub fn arrayize(value: serde_json::Value) -> Result<Vec<String>, RequestError> {
    value
        .as_array()
        .ok_or(RequestError::BadTypeError)?
        .iter()
        .cloned()
        .map(stringify)
        .collect()
}
