use clap::{crate_authors, crate_version, App};
use commands::ux::print;
use commands::ux::progressbar;
use commands::ux::prompt;
use commands::ux::secrets;
use commands::ux::spinner;
use commands::ux::track;

fn main() {
    let matches = App::new("CTO.ai Shell SDK")
        .version(crate_version!())
        .author(crate_authors!())
        .about(commands::APP)
        .subcommand(print::init_cli_command())
        .subcommand(progressbar::init_cli_command())
        .subcommand(prompt::init_cli_command())
        .subcommand(secrets::init_cli_command())
        .subcommand(spinner::init_cli_command())
        .subcommand(track::init_cli_command())
        .get_matches();

    match matches.subcommand() {
        ("print", Some(print_matches)) => print::run(print_matches),
        ("progressbar", Some(progressbar_matches)) => progressbar::run(progressbar_matches),
        ("prompt", Some(prompt_matches)) => prompt::run(prompt_matches),
        ("secrets", Some(secrets_matches)) => secrets::run(secrets_matches),
        ("spinner", Some(spinner_matches)) => spinner::run(spinner_matches),
        ("track", Some(track_matches)) => track::run(track_matches),
        _ => println!("Oops. No command found"),
    }
}
