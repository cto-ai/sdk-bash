use clap::{crate_authors, crate_version, App};
use commands::ux::events;
use commands::ux::print;
use commands::ux::progressbar;
use commands::ux::prompt;
use commands::ux::secrets;
use commands::ux::spinner;
use commands::ux::track;
use commands::{config, state};

fn main() {
    let matches = App::new("CTO.ai Shell SDK")
        .version(crate_version!())
        .author(crate_authors!())
        .about(commands::APP)
        .subcommand(config::init_cli_command())
        .subcommand(events::init_cli_command())
        .subcommand(print::init_cli_command())
        .subcommand(progressbar::init_cli_command())
        .subcommand(prompt::init_cli_command())
        .subcommand(secrets::init_cli_command())
        .subcommand(spinner::init_cli_command())
        .subcommand(state::init_cli_command())
        .subcommand(track::init_cli_command())
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    match matches.subcommand() {
        ("config", Some(config_matches)) => config::run(config_matches),
        ("events", Some(events_matches)) => events::run(events_matches),
        ("print", Some(print_matches)) => print::run(print_matches),
        ("progressbar", Some(progressbar_matches)) => progressbar::run(progressbar_matches),
        ("prompt", Some(prompt_matches)) => prompt::run(prompt_matches),
        ("secrets", Some(secrets_matches)) => secrets::run(secrets_matches),
        ("spinner", Some(spinner_matches)) => spinner::run(spinner_matches),
        ("state", Some(state_matches)) => state::run(state_matches),
        ("track", Some(track_matches)) => track::run(track_matches),
        _ => unreachable!(),
    }
}
