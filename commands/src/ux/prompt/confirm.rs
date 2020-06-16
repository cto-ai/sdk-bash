use super::{FLAG, MESSAGE, NAME};
use crate::descriptions::prompt;
use clap::{App, Arg};
use cto_ai::ux::prompt::{Confirm, Prompt};

pub const CMD: &str = "confirm";

static DEFAULT_TRUE: &str = "default-true";
static DEFAULT_FALSE: &str = "default-false";

// Init the cli commands for the Confirm prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD)
        .about(prompt::CONFIRM)
        .arg(
            Arg::with_name(NAME)
                .long(NAME)
                .short("n")
                .help("Name of the confirm prompt")
                .value_name("NAME")
                .required(true),
        )
        .arg(
            Arg::with_name(MESSAGE)
                .long(MESSAGE)
                .short("m")
                .help("Message to be displayed to the user")
                .required(true)
                .value_name("MESSAGE"),
        )
        .arg(
            Arg::with_name(FLAG)
                .long(FLAG)
                .short("f")
                .help("Command line flag alias associated with this prompt")
                .value_name("FLAG"),
        )
        .arg(
            Arg::with_name(DEFAULT_TRUE)
                .long(DEFAULT_TRUE)
                .short("T")
                .help("Sets the default response to true"),
        )
        .arg(
            Arg::with_name(DEFAULT_FALSE)
                .long(DEFAULT_FALSE)
                .conflicts_with(DEFAULT_TRUE)
                .short("F")
                .help("Sets the default response to false"),
        )
}

// Runs the Confirm prompt
pub fn run(matches: &clap::ArgMatches) {
    let mut confirm = Confirm::new(
        matches.value_of(NAME).unwrap(),
        matches.value_of(MESSAGE).unwrap(),
    );

    if matches.is_present(DEFAULT_FALSE) {
        confirm = confirm.default_value(false);
    }

    if matches.is_present(DEFAULT_TRUE) {
        confirm = confirm.default_value(true);
    }

    if let Some(flag) = matches.value_of(FLAG) {
        confirm = confirm.flag(flag);
    }

    let final_value = confirm.execute().unwrap();
    println!("{}", final_value);
}
