use crate::daemon::simple_request;
use crate::RequestError;

use serde::Serialize;

/// Public facing ProgressBar
#[derive(Clone, Copy, Debug, Serialize, Default)]
pub struct ProgressBar<'a> {
    length: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial: Option<u64>,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    increment: Option<u64>,
}

impl<'a> ProgressBar<'a> {
    /// Returns a new ProgressBar.
    ///
    /// # Example
    /// ```
    /// use cto_ai::ux::progress_bar;
    ///
    /// progress_bar::ProgressBar::new(5);
    /// ```
    pub fn new(length: u64) -> Self {
        ProgressBar {
            length,
            initial: None,
            message: None,
            increment: None,
        }
    }

    /// **[Opitional]** Adds message that will be displayed alongside the progess bar.
    ///
    /// # Example
    /// ```
    /// use cto_ai::ux::progress_bar;
    ///
    /// progress_bar::ProgressBar::new(5).message("Doing my complicated thing");
    /// ```
    pub fn message(mut self, msg: &'a str) -> Self {
        self.message = Some(msg);
        self
    }

    /// **[Opitional]** Initial size of the progress bar.
    ///
    /// # Example
    /// ```
    /// use cto_ai::ux::progress_bar;
    ///
    /// progress_bar::ProgressBar::new(5).initial(0);
    /// ```
    pub fn initial(mut self, value: u64) -> Self {
        self.initial = Some(value);
        self
    }

    /// **[Opitional]** Increments the value of a progress bar by amount increments,
    /// or by 1 increment if no argument is provided.
    ///
    /// # Example
    /// ```
    /// use cto_ai::ux::progress_bar;
    ///
    /// progress_bar::ProgressBar::new(5).increment_by(2);
    /// ```
    pub fn increment_by(mut self, value: u64) -> Self {
        self.increment = Some(value);
        self
    }

    /// Display a progress bar, with length increments of which initial are already ready,
    /// and with an accompanying optional message.
    ///
    /// # Example
    /// ```
    /// use cto_ai::ux::progress_bar;
    ///
    /// progress_bar::ProgressBar::new(5).initial(0).message("Doing my complicated thing");
    /// ```
    pub fn start(self) -> Result<Self, RequestError> {
        simple_request("progress-bar/start", self)?;
        Ok(self)
    }
}

/// Increments the value of a progress bar by amount increments,
/// or by 1 increment if no argument is provided.
///
/// # Example
/// ```norun
/// use cto_ai::ux::progress_bar;
/// use std::{thread, time};
///
/// let pb = progress_bar::ProgressBar::new(5).initial(0).message("Doing my complicated thing");
/// for _ in 0..5 {
///     let sleep_time = time::Duration::from_millis(1000);
///     thread::sleep(sleep_time);
///     progress_bar::advance(None) // or progress_bar::advance(Some(3));
/// }
/// ```
pub fn advance(value: Option<u64>) -> Result<(), RequestError> {
    let mut pb = ProgressBar::new(0);
    if let Some(val) = value {
        pb = pb.increment_by(val);
    }
    simple_request("progress-bar/advance", pb)
}

/// Completes the progress bar, incrementing it to 100% and optionally replacing
/// the message with the given one.
///
/// # Example
/// ```norun
/// use cto_ai::ux::progress_bar;
/// use std::{thread, time};
///
/// let pb = progress_bar::ProgressBar::new(5).initial(0).message("Doing my complicated thing");
///
/// for _ in 0..5 {
///     let sleep_time = time::Duration::from_millis(1000);
///     thread::sleep(sleep_time);
///     progress_bar::advance();
/// }
///
/// progress_bar::stop(None) // or progress_bar::stop(Some("Download Done"))
/// ```
pub fn stop(message: Option<&str>) -> Result<(), RequestError> {
    let mut pb = ProgressBar::new(0);
    if let Some(msg) = message {
        pb = pb.message(msg);
    }
    simple_request("progress-bar/stop", pb)
}
