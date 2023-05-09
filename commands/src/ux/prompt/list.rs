use super::{DEFAULT, FLAG, MESSAGE, NAME};
use crate::descriptions::prompt;
use clap::{App, Arg};
use cto_ai::ux::prompt::{List, Prompt};

pub const CMD: &str = "list";

static CHOICES: &str = "choices";
static AUTOCOMPLETE: &str = "autocomplete";

// Init the cli commands for the List prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD)
        .about(prompt::LIST)
        .arg(
            Arg::with_name(NAME)
                .long(NAME)
                .short("n")
                .help("Name of the list prompt")
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
            Arg::with_name(CHOICES)
                .help("The choices to include in the list")
                .required(true)
                .value_name("CHOICES")
                .multiple(true),
        )
        .arg(
            Arg::with_name(DEFAULT)
                .long(DEFAULT)
                .short("d")
                .help("Sets default selected value in the list")
                .value_name("DEFAULT VALUE"),
        )
        .arg(
            Arg::with_name(AUTOCOMPLETE)
                .long(AUTOCOMPLETE)
                .short("a")
                .help("Enables autocomplete on the list"),
        )
}

// Runs the List prompt
pub fn run(matches: &clap::ArgMatches) {
    let name = matches.value_of(NAME).unwrap();
    let message = matches.value_of(MESSAGE).unwrap();
    let choices = matches
        .values_of(CHOICES)
        .unwrap()
        .map(String::from)
        .collect();

    let mut list = List::new(name, message, choices);

    if let Some(default) = matches.value_of(DEFAULT) {
        list = list.default_value(default);
    }

    if matches.is_present(AUTOCOMPLETE) {
        list = list.autocomplete();
    }

    if let Some(flag) = matches.value_of(FLAG) {
        list = list.flag(flag);
    }

    let final_value = list.execute().unwrap();
    println!("{}", final_value);
}
