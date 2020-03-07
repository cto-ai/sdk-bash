use super::{MESSAGE, NAME};
use crate::descriptions::prompt;
use clap::{App, Arg};
use cto_ai::ux::prompt::Secret;

// Init the cli commands for the Secret prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("secret")
        .about(prompt::SECRET)
        .arg(
            Arg::with_name(NAME)
                .long(NAME)
                .short("n")
                .visible_alias("key")
                .help("Name of the secret prompt. Used as a key to match the secret store")
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
}

// Runs the Secret prompt
pub fn run(matches: &clap::ArgMatches) {
    let final_value = Secret::new(
        matches.value_of(NAME).unwrap(),
        matches.value_of(MESSAGE).unwrap(),
    )
    .execute()
    .unwrap();
    println!("{}", final_value);
}
