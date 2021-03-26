use clap::{crate_authors, crate_version, App};
use commands::ux::events;
use commands::ux::format;
use commands::ux::print;
use commands::ux::progressbar;
use commands::ux::prompt;
use commands::ux::secrets;
use commands::ux::spinner;
use commands::ux::track;
use commands::ux::start;
use commands::{config, state, user};

fn main() {
    let matches = App::new("CTO.ai Shell SDK")
        .version(crate_version!())
        .author(crate_authors!())
        .about(commands::APP)
        .subcommand(config::init_cli_command())
        .subcommand(events::init_cli_command())
        .subcommand(format::init_cli_command())
        .subcommand(print::init_cli_command())
        .subcommand(progressbar::init_cli_command())
        .subcommand(prompt::init_cli_command())
        .subcommand(secrets::init_cli_command())
        .subcommand(spinner::init_cli_command())
        .subcommand(state::init_cli_command())
        .subcommand(track::init_cli_command())
        .subcommand(start::init_cli_command())
        .subcommand(user::init_cli_command())
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    match matches.subcommand() {
        (config::CMD, Some(config_matches)) => config::run(config_matches),
        (events::CMD, Some(events_matches)) => events::run(events_matches),
        (format::CMD, Some(format_matches)) => format::run(format_matches),
        (print::CMD, Some(print_matches)) => print::run(print_matches),
        (progressbar::CMD, Some(progressbar_matches)) => progressbar::run(progressbar_matches),
        (prompt::CMD, Some(prompt_matches)) => prompt::run(prompt_matches),
        (secrets::CMD, Some(secrets_matches)) => secrets::run(secrets_matches),
        (spinner::CMD, Some(spinner_matches)) => spinner::run(spinner_matches),
        (state::CMD, Some(state_matches)) => state::run(state_matches),
        (track::CMD, Some(track_matches)) => track::run(track_matches),
        (start::CMD, Some(start_matches)) => start::run(start_matches),
        (user::CMD, _) => user::run(),
        _ => unreachable!(),
    }
}
