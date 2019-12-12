use clap::App;
use commands::ux::prompt::*;

fn main() {
    let matches = App::new("Bash SDK")
        .version("1.0")
        .author("Hugo Dorea <hugo@cto.ai>")
        .about("CLI for creating ops with using Bash.")
        .subcommand(input::init_cli_command())
        .subcommand(editor::init_cli_command())
        .subcommand(confirm::init_cli_command())
        .subcommand(autocomplete::init_cli_command())
        .subcommand(checkbox::init_cli_command())
        .subcommand(list::init_cli_command())
        .subcommand(datetime::init_cli_command())
        .subcommand(number::init_cli_command())
        .subcommand(password::init_cli_command())
        .subcommand(secret::init_cli_command())
        .get_matches();

    match matches.subcommand() {
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
        _ => println!("Ops. No command found"),
    }
}
