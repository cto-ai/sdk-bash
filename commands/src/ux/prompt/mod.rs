mod checkbox;
mod confirm;
mod datetime;
mod editor;
mod input;
mod list;
mod number;
mod password;

mod secret;

pub(crate) static NAME: &str = "name";
pub(crate) static MESSAGE: &str = "message";
pub(crate) static DEFAULT: &str = "default";
pub(crate) static FLAG: &str = "flag";

use crate::descriptions;
use clap::{App, ArgMatches};

pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("prompt")
        .about(descriptions::PROMPT)
        .subcommand(input::init_cli_command())
        .subcommand(editor::init_cli_command())
        .subcommand(confirm::init_cli_command())
        .subcommand(checkbox::init_cli_command())
        .subcommand(list::init_cli_command())
        .subcommand(datetime::init_cli_command())
        .subcommand(number::init_cli_command())
        .subcommand(password::init_cli_command())
        .subcommand(secret::init_cli_command())
}

pub fn run(matches: &ArgMatches) {
    match matches.subcommand() {
        ("input", Some(input_matches)) => input::run(input_matches),
        ("editor", Some(editor_matches)) => editor::run(editor_matches),
        ("confirm", Some(confirm_matches)) => confirm::run(confirm_matches),
        ("checkbox", Some(checkbox_matches)) => checkbox::run(checkbox_matches),
        ("list", Some(list_matches)) => list::run(list_matches),
        ("datetime", Some(datetime_matches)) => datetime::run(datetime_matches),
        ("number", Some(number_matches)) => number::run(number_matches),
        ("password", Some(password_matches)) => password::run(password_matches),
        ("secret", Some(secret_matches)) => secret::run(secret_matches),
        _ => panic!("Oops. No prompt found"),
    }
}
