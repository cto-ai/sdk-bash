use clap::{crate_authors, crate_version, App};
use commands::ux::print;
use commands::ux::progressbar;
use commands::ux::prompt;
use commands::ux::spinner;

fn main() {
    let matches = App::new("CTO.ai Shell Client - UX")
        .version(crate_version!())
        .author(crate_authors!())
        .about(commands::APP)
        .subcommand(print::init_cli_command())
        .subcommand(progressbar::init_cli_command())
        .subcommand(prompt::init_cli_command())
        .subcommand(spinner::init_cli_command())
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    match matches.subcommand() {
        (print::CMD, Some(print_matches)) => print::run(print_matches),
        (progressbar::CMD, Some(progressbar_matches)) => progressbar::run(progressbar_matches),
        (prompt::CMD, Some(prompt_matches)) => prompt::run(prompt_matches),
        (spinner::CMD, Some(spinner_matches)) => spinner::run(spinner_matches),
        _ => unreachable!(),
    }
}
