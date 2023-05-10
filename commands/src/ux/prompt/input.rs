use super::{DEFAULT, FLAG, MESSAGE, NAME};
use crate::descriptions::prompt;
use clap::{App, Arg};
use cto_ai::ux::prompt::{Input, Prompt};

pub const CMD: &str = "input";

static ALLOW_EMPTY: &str = "allow empty";

// Init the cli commands for the input prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD)
        .about(prompt::INPUT)
        .arg(
            Arg::with_name(NAME)
                .long(NAME)
                .short("n")
                .help("Name of the input prompt")
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
            Arg::with_name(DEFAULT)
                .long(DEFAULT)
                .short("d")
                .help("Sets a default response")
                .value_name("DEFAULT"),
        )
        .arg(
            Arg::with_name(FLAG)
                .long(FLAG)
                .short("f")
                .help("Command line flag alias associated with this prompt")
                .value_name("FLAG"),
        )
        .arg(
            Arg::with_name(ALLOW_EMPTY)
                .long(ALLOW_EMPTY)
                .short("a")
                .help("Allows the user to submit an empty value"),
        )
}

// Runs the input prompt
pub fn run(matches: &clap::ArgMatches) {
    let mut input = Input::new(
        matches.value_of(NAME).unwrap(),
        matches.value_of(MESSAGE).unwrap(),
    );

    if let Some(default) = matches.value_of(DEFAULT) {
        input = input.default_value(default);
    }

    if matches.is_present(ALLOW_EMPTY) {
        input = input.allow_empty();
    }

    if let Some(flag) = matches.value_of(FLAG) {
        input = input.flag(flag);
    }

    let final_value = input.execute().unwrap();
    println!("{}", final_value);
}
