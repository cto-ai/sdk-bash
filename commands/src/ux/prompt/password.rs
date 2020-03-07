use super::{MESSAGE, NAME};
use crate::descriptions::prompt;
use clap::{App, Arg};
use cto_ai::ux::prompt::Password;

static CONFIRM: &str = "confirm";

// Init the cli commands for the Password prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("password")
        .about(prompt::PASSWORD)
        .arg(
            Arg::with_name(NAME)
                .long(NAME)
                .short("n")
                .visible_alias("key")
                .help("Name of the password prompt. Used as a key to match the secret store")
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
            Arg::with_name(CONFIRM)
                .long(CONFIRM)
                .help("Asks for password confirmation."),
        )
}

// Runs the Password prompt
pub fn run(matches: &clap::ArgMatches) {
    let mut password = Password::new(
        matches.value_of(NAME).unwrap(),
        matches.value_of(MESSAGE).unwrap(),
    );

    if matches.is_present(CONFIRM) {
        password = password.confirm();
    }

    let final_value = password.execute().unwrap();
    println!("{}", final_value);
}
