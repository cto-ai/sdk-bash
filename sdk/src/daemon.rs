//! Code for interacting with the daemon API directly

use std::env;
use std::fs::File;
use std::io::BufReader;

use reqwest::blocking::{Client, Response};
use serde::{Deserialize, Serialize};

use crate::RequestError;

/// Make a request to the daemon API, given the endpoint and request body
fn make_request(endpoint: &str, body: impl Serialize) -> Result<Response, RequestError> {
    let port = env::var("SDK_SPEAK_PORT").expect("SDK_SPEAK_PORT environment variable not set");

    Ok(Client::new()
        .post(&format!("http://localhost:{}/{}", port, endpoint))
        .json(&body)
        .send()?
        .error_for_status()?)
}

/// Read the JSON from the daemon async response fifo
fn read_fifo(filename: String) -> Result<serde_json::Value, RequestError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

/// Make a request to the daemon expecting an empty response on success
pub fn simple_request(endpoint: &str, body: impl Serialize) -> Result<(), RequestError> {
    make_request(endpoint, body)?;
    Ok(())
}

/// The response body from the daemon when a value is returned directly.
#[derive(Debug, Clone, Deserialize)]
struct ValueResponse {
    value: serde_json::Value,
}

/// Make a request to the daemon receiving a direct JSON response to the HTTP request.
pub fn sync_request(
    endpoint: &str,
    body: impl Serialize,
) -> Result<serde_json::Value, RequestError> {
    Ok(make_request(endpoint, body)?.json::<ValueResponse>()?.value)
}

/// The response body from the daemon when a value is returned through a named pipe
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct DaemonResponse {
    reply_filename: String,
}

/// Make a request to the daemon and read the reply from the supplied fifo
pub fn async_request(
    endpoint: &str,
    body: impl Serialize,
) -> Result<serde_json::Value, RequestError> {
    let filename = make_request(endpoint, body)?
        .json::<DaemonResponse>()?
        .reply_filename;

    read_fifo(filename)
}
