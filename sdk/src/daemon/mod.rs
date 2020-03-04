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

pub fn simple_request<T: Serialize>(endpoint: &str, body: T) -> Result<(), RequestError> {
    make_request(endpoint, body)?;
    Ok(())
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct DaemonResponse {
    reply_filename: String,
}

pub fn async_request<T: Serialize>(
    endpoint: &str,
    body: T,
) -> Result<serde_json::Value, RequestError> {
    let filename = make_request(endpoint, body)?
        .json::<DaemonResponse>()?
        .reply_filename;

    Ok(read_fifo(filename)?)
}
