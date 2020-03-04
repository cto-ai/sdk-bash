use crate::daemon::simple_request;
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
    let p = Print { text };
    simple_request("print", p)?;
    Ok(())
}

#[cfg(test)]
mod test {
    extern crate httpmock;
    use super::*;
    use httpmock::Method::POST;
    use httpmock::{mock, with_mock_server};

    #[test]
    #[with_mock_server]
    fn test_ux_print() {
        std::env::set_var("SDK_SPEAK_PORT", "5000");
        let m = mock(POST, "/print")
            .expect_header("Content-Type", "application/json")
            .return_status(204)
            .create();
        // TODO: we need to go back to this test as the print request doesn't return anything
        // and the printint if done inside the daemon.
        print("Some Value");
        assert_eq!(m.times_called(), 1);
    }
}
