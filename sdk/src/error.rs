use custom_error::custom_error;

custom_error! {
    pub RequestError

    DaemonRequestError{source: reqwest::Error} = "Invalid request",
    FileOpenError{source: std::io::Error} = "Error opening file",
    DeserializeError{source: serde_json::Error} = "Error parsing file",
}
