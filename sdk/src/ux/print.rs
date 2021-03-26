use crate::daemon::{simple_request, HttpMethod};
use crate::RequestError;

use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
struct Print<'a> {
    text: &'a str,
}

/// Display text to the user. Locally, prints to the terminal.
///
/// # Example
/// ```norun
/// use cto_ai::ux::print;
///
/// print("Some printed value");
/// ```
pub fn print(text: &str) -> Result<(), RequestError> {
    simple_request("print", Print { text }, HttpMethod::POST)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use mockito::mock;

    #[test]
    fn test_ux_print() {
        std::env::set_var(
            "SDK_SPEAK_PORT",
            mockito::server_address().port().to_string(),
        );
        let m = mock("POST", "/print")
            .match_header("Content-Type", "application/json")
            .with_status(204)
            .create();
        // TODO: we need to go back to this test as the print request doesn't return anything
        // and the printint if done inside the daemon.
        assert!(print("Some Value").is_ok());
        m.assert();
    }
}
