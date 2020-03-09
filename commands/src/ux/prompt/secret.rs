use super::{FLAG, MESSAGE, NAME};
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
        .arg(
            Arg::with_name(FLAG)
                .long(FLAG)
                .short("f")
                .help("Command line flag alias associated with this prompt")
                .value_name("FLAG"),
        )
}

// Runs the Secret prompt
pub fn run(matches: &clap::ArgMatches) {
    let mut secret = Secret::new(
        matches.value_of(NAME).unwrap(),
        matches.value_of(MESSAGE).unwrap(),
    );

    if let Some(flag) = matches.value_of(FLAG) {
        secret = secret.flag(flag);
    }

    let final_value = secret.execute().unwrap();
    println!("{}", final_value);
}
