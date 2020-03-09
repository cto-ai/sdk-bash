pub static APP: &str = "CLI client for The Ops Platform built-in functionality";

// Top level subcommands
pub static PRINT: &str = "Displays text to the user";
pub static PROGRESSBAR: &str = "Displays progress bars to the user";
pub static PROMPT: &str = "Prompts the user for information";
pub static SECRETS: &str = "Accesses the user's secret store";
pub static SPINNER: &str = "Displays spinners to the user";
pub static TRACK: &str = "Track an analytics event";

// Progress bar subcommands
pub mod progress {
    pub static START: &str = "Starts a new progress bar";
    pub static ADVANCE: &str = "Advances the current progress bar";
    pub static STOP: &str = "Stops the current progress bar";
}

// Prompt subcommands
pub mod prompt {
    pub static CHECKBOX: &str = "Prompts the user to select multiple options from a list";
    pub static CONFIRM: &str = "Prompts the user for a yes-no answer";
    pub static DATETIME: &str = "Prompts the user for a date and/or time";
    pub static EDITOR: &str = "Prompts the user for multi-line text";
    pub static INPUT: &str = "Prompts the user for a single line of text";
    pub static LIST: &str = "Prompts the user to select a single option from a list";
    pub static NUMBER: &str = "Prompts the user for a number";
    pub static PASSWORD: &str = "Prompts the user for a password";
    pub static SECRET: &str = "Prompts the user for a secret";
}

// Secret subcommands
pub mod secrets {
    pub static GET: &str = "Gets a secret from the user's secret store";
    pub static SET: &str = "Sets a secret in the user's secret store";
}

// Spinner subcommands
pub mod spinner {
    pub static START: &str = "Starts a new spinner";
    pub static STOP: &str = "Stops the current spinner";
}