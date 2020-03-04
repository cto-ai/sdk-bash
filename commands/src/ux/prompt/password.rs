use clap::{App, Arg};
use cto_ai::ux::prompt::Password;

// Init the cli commands for the Password prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("password")
        .about("It starts a new password prompt")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Name of the password.")
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
            Arg::with_name("confirm")
                .long("confirm")
                .help("Asks for password confirmation."),
        )
}

// Runs the Password prompt
pub fn run(matches: &clap::ArgMatches) {
    let name = matches.value_of("name").unwrap();
    let message = matches.value_of("message").unwrap();

    let mut password = Password::new(name, message);

    if matches.is_present("confirm") {
        password = password.confirm();
    }

    let final_value = password.execute().unwrap();
    println!("{}", final_value);
}
