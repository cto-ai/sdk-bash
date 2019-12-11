use clap::{App, Arg};
use ops_sdk::ux::prompt::Autocomplete;

// Init the cli commands fot the Autocomplete prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("autocomplete")
        .about("It starts a new autocomplete prompt")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Name of the autocomplete")
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
                .help("Sets default value for autocomplete")
                .value_name("DEFAULT VALUE"),
        )
}

// Runs the Autocomplete prompt
pub fn run(autocomplete_matches: &clap::ArgMatches) {
    let name = autocomplete_matches.value_of("name").unwrap();
    let message = autocomplete_matches.value_of("message").unwrap();
    let choices: Vec<String> = autocomplete_matches
        .values_of("choices")
        .unwrap()
        .map(|val| String::from(val))
        .collect();

    let mut autocomplete = Autocomplete::new(name, message, choices);

    if autocomplete_matches.is_present("default value") {
        let default_value = autocomplete_matches.value_of("default value").unwrap();
        autocomplete = autocomplete.default_value(default_value);
    }

    let final_value = autocomplete.execute();
    println!("{}", final_value);
}
