use clap::{App, Arg};
use cto_ai::ux::prompt::Number;

// Init the cli commands for the number prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("number")
        .about("It starts a new number prompt.")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Name of the number.")
                .value_name("NAME")
                .required(true),
        )
        .arg(
            Arg::with_name("message")
                .long("message")
                .short("m")
                .help("Message to be displayed.")
                .required(true)
                .value_name("MESSAGE"),
        )
        .arg(
            Arg::with_name("default value")
                .long("default-value")
                .short("d")
                .help("Sets default number.")
                .value_name("DEFAULT VALUE"),
        )
        .arg(
            Arg::with_name("min")
                .long("min")
                .help("Sets minimum value for number.")
                .value_name("MIN"),
        )
        .arg(
            Arg::with_name("max")
                .long("max")
                .help("Sets maximum value for number.")
                .value_name("MAX"),
        )
}

// Runs the number prompt
pub fn run(number_matches: &clap::ArgMatches) {
    let name = number_matches.value_of("name").unwrap();
    let message = number_matches.value_of("message").unwrap();

    let mut number = Number::new(name, message);

    if number_matches.is_present("default value") {
        let parsed_number = parse_number_for(number_matches, "default value");
        number = number.default_value(parsed_number);
    }

    if number_matches.is_present("min") {
        let parsed_number = parse_number_for(number_matches, "min");
        number = number.min(parsed_number);
    }

    if number_matches.is_present("max") {
        let parsed_number = parse_number_for(number_matches, "max");
        number = number.max(parsed_number);
    }

    let final_value = number.execute().unwrap();
    println!("{}", final_value);
}

fn parse_number_for(matches: &clap::ArgMatches, cmd: &str) -> i32 {
    let value: i32 = matches
        .value_of(cmd)
        .unwrap()
        .parse()
        .expect("Could not read number.");
    value
}
