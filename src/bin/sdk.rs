use clap::{crate_authors, crate_version, App};
use commands::ux::events;
use commands::ux::secrets;
use commands::ux::track;
use commands::ux::start;
use commands::{config, state};

fn main() {
    let matches = App::new("CTO.ai Shell Client - SDK")
        .version(crate_version!())
        .author(crate_authors!())
        .about(commands::APP)
        .subcommand(config::init_cli_command())
        .subcommand(events::init_cli_command())
        .subcommand(secrets::init_cli_command())
        .subcommand(state::init_cli_command())
        .subcommand(track::init_cli_command())
        .subcommand(start::init_cli_command())
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    match matches.subcommand() {
        (config::CMD, Some(config_matches)) => config::run(config_matches),
        (events::CMD, Some(events_matches)) => events::run(events_matches),
        (secrets::CMD, Some(secrets_matches)) => secrets::run(secrets_matches),
        (state::CMD, Some(state_matches)) => state::run(state_matches),
        (track::CMD, Some(track_matches)) => track::run(track_matches),
        (start::CMD, Some(start_matches)) => start::run(start_matches),
        _ => unreachable!(),
    }
}
