use clap::{App, Arg};
use ops_sdk::ux::prompt::List;

// Init the cli commands for the List prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("list")
        .about("It starts a new list prompt")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Name of the list")
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
            Arg::with_name("choices")
                .long("choices")
                .short("c")
                .help(
                    "List of choices to be loaded in the list. \
                    Example: -c val1 -c val2 OR -c val1 val2 OR -c 'Val 1' 'Val 2'
                ",
                )
                .required(true)
                .value_name("CHOICES")
                .multiple(true),
        )
        .arg(
            Arg::with_name("default value")
                .long("default-value")
                .short("d")
                .help("Sets default value for list")
                .value_name("DEFAULT VALUE"),
        )
}

// Runs the List prompt
pub fn run(list_matches: &clap::ArgMatches) {
    let name = list_matches.value_of("name").unwrap();
    let message = list_matches.value_of("message").unwrap();
    let choices: Vec<String> = list_matches
        .values_of("choices")
        .unwrap()
        .map(|val| String::from(val))
        .collect();

    let mut list = List::new(name, message, choices);

    if list_matches.is_present("default value") {
        let default_value = list_matches.value_of("default value").unwrap();
        list = list.default_value(default_value);
    }

    let final_value = list.execute();
    println!("{}", final_value);
}
