use clap::{App, Arg};
use cto_ai::ux::prompt::Input;

// Init the cli commands for the input prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("input")
        .about("It starts a new input prompt")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Name of the input")
                .value_name("NAME")
                .required(true),
        )
        .arg(
            Arg::with_name("message")
                .long("message")
                .short("m")
                .help("Message to be displayed")
                .required(true)
                .value_name("MESSAGE"),
        )
        .arg(
            Arg::with_name("default value")
                .long("default-value")
                .short("d")
                .help("Sets default value for input")
                .value_name("DEFAULT VALUE"),
        )
        .arg(
            Arg::with_name("allow empty")
                .long("allow-empty")
                .short("a")
                .help("Allows empty value on input"),
        )
}

// Runs the input prompt
pub fn run(input_matches: &clap::ArgMatches) {
    let name = input_matches.value_of("name").unwrap();
    let message = input_matches.value_of("message").unwrap();

    let mut input = Input::new(name, message);

    if input_matches.is_present("default value") {
        let default_value = input_matches.value_of("default value").unwrap();
        input = input.default_value(default_value);
    }

    if input_matches.is_present("allow empty") {
        input = input.allow_empty();
    }

    let final_value = input.execute().unwrap();
    println!("{}", final_value);
}
