use crate::daemon::simple_request;
use crate::RequestError;

use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
struct Spinner<'a> {
    text: &'a str,
}

/// Starts a spinner accompanied with the given message.
///
/// # Example
/// ```norun
/// use cto_ai::ux::spinner;
///
/// spinner::start("Processing");
/// ```
pub fn start(text: &str) -> Result<(), RequestError> {
    let s = Spinner { text };
    simple_request("start-spinner", s)
}

/// Stops the currently running spinner, replacing the message with the given text.
///
/// # Example
/// ```norun
/// use cto_ai::ux::spinner;
///
/// spinner::start("Processing");
/// ... // Some expensive call
/// spinner::stop("Process Done!");
/// ```
pub fn stop(text: &str) -> Result<(), RequestError> {
    let s = Spinner { text };
    simple_request("stop-spinner", s)
}
