use clap::{App, Arg};
use ops_sdk::ux::prompt::Checkbox;

// Init the cli commands for the Checkbox prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("checkbox")
        .about("It starts a new checkbox prompt")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Name of the checkbox")
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
}

// Runs the Checkbox prompt
pub fn run(checkbox_matches: &clap::ArgMatches) {
    let name = checkbox_matches.value_of("name").unwrap();
    let message = checkbox_matches.value_of("message").unwrap();
    let choices: Vec<String> = checkbox_matches
        .values_of("choices")
        .unwrap()
        .map(|val| String::from(val))
        .collect();

    let final_value = Checkbox::new(name, message, choices).execute();

    println!("{}", final_value);
}
