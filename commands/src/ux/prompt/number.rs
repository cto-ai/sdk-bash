use super::{DEFAULT, FLAG, MESSAGE, NAME};
use crate::descriptions::prompt;
use crate::validate::{numeric, NumericArg};
use clap::{App, Arg};
use cto_ai::ux::prompt::{Number, Prompt};

pub const CMD: &str = "number";

static MAX: &str = "max";
static MIN: &str = "min";

// Init the cli commands for the number prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD)
        .about(prompt::NUMBER)
        .arg(
            Arg::with_name(NAME)
                .long(NAME)
                .short("n")
                .help("Name of the number prompt")
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
            Arg::with_name(DEFAULT)
                .long(DEFAULT)
                .short("d")
                .help("Default value for the number prompt")
                .value_name("DEFAULT VALUE")
                .validator(numeric),
        )
        .arg(
            Arg::with_name(MIN)
                .long(MIN)
                .help("The minimum acceptable value for the prompt (inclusive)")
                .value_name("MIN")
                .validator(numeric),
        )
        .arg(
            Arg::with_name(MAX)
                .long(MAX)
                .help("The maximum acceptable value for the prompt (inclusive)")
                .value_name("MAX")
                .validator(numeric),
        )
}

// Runs the number prompt
pub fn run(matches: &clap::ArgMatches) {
    let mut number = Number::new(
        matches.value_of(NAME).unwrap(),
        matches.value_of(MESSAGE).unwrap(),
    );

    if let Some(default) = matches.value_of_i32(DEFAULT) {
        number = number.default_value(default);
    }

    if let Some(min) = matches.value_of_i32(MIN) {
        number = number.min(min);
    }

    if let Some(max) = matches.value_of_i32(MAX) {
        number = number.max(max);
    }

    if let Some(flag) = matches.value_of(FLAG) {
        number = number.flag(flag);
    }

    let final_value = number.execute().unwrap();
    println!("{}", final_value);
}
