use std::env;
use std::fs::File;
use std::io::BufReader;

use reqwest::blocking::{Client, Response};
use serde::{Deserialize, Serialize};

use crate::RequestError;

pub enum HttpMethod {
    GET,
    POST,
}

fn make_request<T: Serialize>(endpoint: &str, body: T, method: HttpMethod) -> Result<Response, RequestError> {
    let port = env::var("SDK_SPEAK_PORT").expect("SDK_SPEAK_PORT environment variable not set");

    match method {
        HttpMethod::POST => Ok(Client::new()
            .post(&format!("http://localhost:{}/{}", port, endpoint))
            .json(&body)
            .send()?
            .error_for_status()?),
        HttpMethod::GET => Ok(Client::new()
            .get(&format!("http://localhost:{}/{}", port, endpoint))
            .send()?
            .error_for_status()?),
    }
}

fn read_fifo(filename: String) -> Result<serde_json::Value, RequestError> {
    let file = File::open(&filename)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

pub fn simple_request(endpoint: &str, body: impl Serialize, method: HttpMethod) -> Result<(), RequestError> {
    make_request(endpoint, body, method)?;
    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
struct ValueResponse {
    value: serde_json::Value,
}

pub fn sync_request(
    endpoint: &str,
    body: impl Serialize,
    method: HttpMethod
) -> Result<serde_json::Value, RequestError> {
    Ok(make_request(endpoint, body, method)?.json::<ValueResponse>()?.value)
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct DaemonResponse {
    reply_filename: String,
}

pub fn async_request(
    endpoint: &str,
    body: impl Serialize,
    method: HttpMethod
) -> Result<serde_json::Value, RequestError> {
    let filename = make_request(endpoint, body, method)?
        .json::<DaemonResponse>()?
        .reply_filename;

    Ok(read_fifo(filename)?)
}
