mod start {
    use crate::descriptions::spinner as descriptions;
    use clap::{App, Arg};
    use cto_ai::ux::spinner;

    pub const CMD: &str = "start";

    static MESSAGE: &str = "message";

    // Init the cli commands for spinner start
    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new(CMD).about(descriptions::START).arg(
            Arg::with_name(MESSAGE)
                .long(MESSAGE)
                .short("m")
                .help("Message to be displayed with the spinner")
                .required(true)
                .value_name("MESSAGE"),
        )
    }

    // Runs the spinner start
    pub fn run(matches: &clap::ArgMatches) {
        spinner::start(matches.value_of(MESSAGE).unwrap()).unwrap();
    }
}

mod stop {
    use crate::descriptions::spinner as descriptions;
    use clap::{App, Arg};
    use cto_ai::ux::spinner;

    pub const CMD: &str = "stop";

    static MESSAGE: &str = "message";

    // Init the cli commands for spinner stop
    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new(CMD).about(descriptions::STOP).arg(
            Arg::with_name(MESSAGE)
                .long(MESSAGE)
                .short("m")
                .help("Message to be displayed with the stopped spinner")
                .value_name("MESSAGE")
                .default_value("Done!"),
        )
    }

    // Runs the spinner stop
    pub fn run(matches: &clap::ArgMatches) {
        spinner::stop(matches.value_of(MESSAGE).unwrap()).unwrap();
    }
}

use crate::descriptions;
use clap::{App, ArgMatches};

pub const CMD: &str = "spinner";

pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD)
        .about(descriptions::SPINNER)
        .subcommand(start::init_cli_command())
        .subcommand(stop::init_cli_command())
}

pub fn run(matches: &ArgMatches) {
    match matches.subcommand() {
        (start::CMD, Some(start_matches)) => start::run(start_matches),
        (stop::CMD, Some(stop_matches)) => stop::run(stop_matches),
        _ => println!("Oops. No spinner command found"),
    }
}
