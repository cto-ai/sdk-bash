use clap::{App, Arg};
use ops_sdk::ux::prompt::Secret;

// Init the cli commands for the Secret prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("secret")
        .about("It starts a new secret prompt")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Name of the secret.")
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
}

// Runs the Secret prompt
pub fn run(matches: &clap::ArgMatches) {
    let name = matches.value_of("name").unwrap();
    let message = matches.value_of("message").unwrap();

    let final_value = Secret::new(name, message).execute();
    println!("{}", final_value);
}
