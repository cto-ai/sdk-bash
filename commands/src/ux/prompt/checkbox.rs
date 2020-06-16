use super::{DEFAULT, FLAG, MESSAGE, NAME};
use crate::descriptions::prompt;
use clap::{App, Arg};
use cto_ai::ux::prompt::{Checkbox, Prompt};

pub const CMD: &str = "checkbox";

static CHOICES: &str = "choices";

// Init the cli commands for the Checkbox prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD)
        .about(prompt::CHECKBOX)
        .arg(
            Arg::with_name(NAME)
                .long(NAME)
                .short("n")
                .help("Name of the checkbox prompt")
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
                .help("List of choices to present to the user")
                .required(true)
                .value_name("CHOICES")
                .multiple(true),
        )
        .arg(
            Arg::with_name(DEFAULT)
                .long(DEFAULT)
                .short("d")
                .multiple(true)
                .help("Value or values to be selected in the list by default")
                .value_name("DEFAULT VALUE"),
        )
}

// Runs the Checkbox prompt
pub fn run(matches: &clap::ArgMatches) {
    let name = matches.value_of(NAME).unwrap();
    let message = matches.value_of(MESSAGE).unwrap();
    let choices = matches
        .values_of(CHOICES)
        .unwrap()
        .map(String::from)
        .collect();

    let mut checkbox = Checkbox::new(name, message, choices);

    if let Some(defaults) = matches.values_of(DEFAULT) {
        let default_vec = defaults.map(String::from).collect();
        checkbox = checkbox.default(default_vec);
    }

    if let Some(flag) = matches.value_of(FLAG) {
        checkbox = checkbox.flag(flag);
    }

    let final_value = checkbox.execute().unwrap();

    for result in final_value.iter() {
        println!("{}", result);
    }
}
