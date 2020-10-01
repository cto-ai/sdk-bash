//! Error definitions for the Rust SDK

use custom_error::custom_error;

custom_error! {
    /// An error making the daemon request
    pub RequestError

    DaemonRequestError{source: reqwest::Error} = "Invalid request",
    FileOpenError{source: std::io::Error} = "Error opening file",
    DeserializeError{source: serde_json::Error} = "Error parsing file",
    JsonKeyError = "Expected key not found",
    BadTypeError = "Expected type not found",
}
