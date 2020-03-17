use clap::{crate_authors, crate_version, App};
use commands::ux::secrets;
use commands::ux::track;
use commands::{config, state};

fn main() {
    let matches = App::new("CTO.ai Shell Client - SDK")
        .version(crate_version!())
        .author(crate_authors!())
        .about(commands::APP)
        .subcommand(config::init_cli_command())
        .subcommand(secrets::init_cli_command())
        .subcommand(state::init_cli_command())
        .subcommand(track::init_cli_command())
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    match matches.subcommand() {
        ("config", Some(config_matches)) => config::run(config_matches),
        ("secrets", Some(secrets_matches)) => secrets::run(secrets_matches),
        ("state", Some(state_matches)) => state::run(state_matches),
        ("track", Some(track_matches)) => track::run(track_matches),
        _ => unreachable!(),
    }
}
