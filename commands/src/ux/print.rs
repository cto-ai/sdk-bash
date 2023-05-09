use crate::descriptions;
use clap::{App, Arg, ArgMatches};
use cto_ai::ux::print;

pub const CMD: &str = "print";
static TEXT: &str = "text";

pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD)
        .about(descriptions::PRINT)
        .arg(Arg::with_name(TEXT).takes_value(true).multiple(true))
}

// Runs the print command
pub fn run(matches: &ArgMatches) {
    let text: Vec<&str> = matches.values_of(TEXT).unwrap().collect();
    print(&text.join(" ")).unwrap();
}
