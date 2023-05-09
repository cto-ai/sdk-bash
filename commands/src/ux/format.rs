mod bold {
    use crate::descriptions::format as descriptions;
    use clap::{App, Arg};
    use cto_ai::ux::format;

    pub const CMD: &str = "bold";

    static TEXT: &str = "text";

    // Init the cli commands for bold format
    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new(CMD)
            .about(descriptions::BOLD)
            .arg(Arg::with_name(TEXT).takes_value(true).multiple(true))
    }

    // Runs the bold format
    pub fn run(matches: &clap::ArgMatches) {
        let text: Vec<&str> = matches.values_of(TEXT).unwrap().collect();
        println!("{}", format::bold(&text.join(" ")));
    }
}

mod italic {
    use crate::descriptions::format as descriptions;
    use clap::{App, Arg};
    use cto_ai::ux::format;

    pub const CMD: &str = "italic";

    static TEXT: &str = "text";

    // Init the cli commands for italic format
    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new(CMD)
            .about(descriptions::ITALIC)
            .arg(Arg::with_name(TEXT).takes_value(true).multiple(true))
    }

    // Runs the italic format
    pub fn run(matches: &clap::ArgMatches) {
        let text: Vec<&str> = matches.values_of(TEXT).unwrap().collect();
        println!("{}", format::italic(&text.join(" ")));
    }
}

use crate::descriptions;
use clap::{App, ArgMatches};

pub const CMD: &str = "format";

pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD)
        .about(descriptions::FORMAT)
        .subcommand(bold::init_cli_command())
        .subcommand(italic::init_cli_command())
}

pub fn run(matches: &ArgMatches) {
    match matches.subcommand() {
        (bold::CMD, Some(bold_matches)) => bold::run(bold_matches),
        (italic::CMD, Some(italic_matches)) => italic::run(italic_matches),
        _ => println!("Oops. No format command found"),
    }
}
