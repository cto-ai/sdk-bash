use clap::{App, Arg};
use commands::ux::print;
use commands::ux::progressbar;
use commands::ux::prompt::*;
use commands::ux::spinner;

fn main() {
    let matches = App::new("Bash SDK")
        .version("1.0")
        .author("Hugo Dorea <hugo@cto.ai>")
        .about("CLI for creating ops with using Bash.")
        .subcommand(
            App::new("prompt")
                .about("Prompt Commands")
                .subcommand(input::init_cli_command())
                .subcommand(editor::init_cli_command())
                .subcommand(confirm::init_cli_command())
                .subcommand(autocomplete::init_cli_command())
                .subcommand(checkbox::init_cli_command())
                .subcommand(list::init_cli_command())
                .subcommand(datetime::init_cli_command())
                .subcommand(number::init_cli_command())
                .subcommand(password::init_cli_command())
                .subcommand(secret::init_cli_command()),
        )
        .subcommand(
            App::new("spinner")
                .about("Spinner Commands.")
                .subcommand(spinner::start::init_cli_command())
                .subcommand(spinner::stop::init_cli_command()),
        )
        .subcommand(
            App::new("progressbar")
                .about("Progress Bar Commands.")
                .subcommand(progressbar::start::init_cli_command())
                .subcommand(progressbar::advance::init_cli_command())
                .subcommand(progressbar::stop::init_cli_command()),
        )
        .subcommand(
            App::new("print").about("Print Command.").arg(
                Arg::with_name("print_text")
                    .takes_value(true)
                    .multiple(true),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        ("prompt", Some(prompt_matches)) => match prompt_matches.subcommand() {
            ("input", Some(input_matches)) => input::run(input_matches),
            ("editor", Some(editor_matches)) => editor::run(editor_matches),
            ("confirm", Some(confirm_matches)) => confirm::run(confirm_matches),
            ("autocomplete", Some(autocomplete_matches)) => autocomplete::run(autocomplete_matches),
            ("checkbox", Some(checkbox_matches)) => checkbox::run(checkbox_matches),
            ("list", Some(list_matches)) => list::run(list_matches),
            ("datetime", Some(datetime_matches)) => datetime::run(datetime_matches),
            ("number", Some(number_matches)) => number::run(number_matches),
            ("password", Some(password_matches)) => password::run(password_matches),
            ("secret", Some(secret_matches)) => secret::run(secret_matches),
            _ => println!("Oops. No prompt found"),
        },
        ("spinner", Some(spinner_matches)) => match spinner_matches.subcommand() {
            ("start", Some(start_matches)) => spinner::start::run(start_matches),
            ("stop", Some(stop_matches)) => spinner::stop::run(stop_matches),
            _ => println!("Oops. No spinner command found"),
        },
        ("progressbar", Some(progressbar_matches)) => match progressbar_matches.subcommand() {
            ("start", Some(start_matches)) => progressbar::start::run(start_matches),
            ("advance", Some(_)) => progressbar::advance::run(),
            ("stop", Some(_)) => progressbar::stop::run(),
            _ => println!("Oops. No progress bar command found"),
        },
        ("print", Some(print_matches)) => print::run(print_matches),
        _ => println!("Oops. No command found"),
    }
}
