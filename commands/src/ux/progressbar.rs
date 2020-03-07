mod start {
    use crate::descriptions::progress;
    use crate::validate::{positive_number, positive_or_zero, NumericArg};
    use clap::{App, Arg};
    use cto_ai::ux::progress_bar;

    static LENGTH: &str = "length";
    static MESSAGE: &str = "message";
    static INITIAL: &str = "initial";

    // Init the cli commands for progressbar start
    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new("start")
            .about(progress::START)
            .arg(
                Arg::with_name(LENGTH)
                    .long(LENGTH)
                    .short("l")
                    .help("Sets the full length of the progress bar.")
                    .required(true)
                    .value_name("LENGTH")
                    .validator(positive_number),
            )
            .arg(
                Arg::with_name(MESSAGE)
                    .long(MESSAGE)
                    .short("m")
                    .help("Descriptive message accompanying the progress bar")
                    .value_name("MESSAGE"),
            )
            .arg(
                Arg::with_name(INITIAL)
                    .long(INITIAL)
                    .short("i")
                    .help("Initial filled length of the progress bar.")
                    .value_name("INITIAL")
                    .validator(positive_or_zero),
            )
    }

    // Runs the progress start
    pub fn run(matches: &clap::ArgMatches) {
        let mut pb = progress_bar::ProgressBar::new(matches.value_of_u64(LENGTH).unwrap());

        if let Some(message) = matches.value_of(MESSAGE) {
            pb = pb.message(message);
        }

        if let Some(init) = matches.value_of_u64(INITIAL) {
            pb = pb.initial(init);
        }

        pb.start().unwrap();
    }
}

mod advance {
    use crate::descriptions::progress;
    use crate::validate::{positive_number, NumericArg};
    use clap::{App, Arg};
    use cto_ai::ux::progress_bar;

    static INCREMENT: &str = "increment";

    // Init the cli commands for progressbar advance
    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new("advance").about(progress::ADVANCE).arg(
            Arg::with_name(INCREMENT)
                .help("Length to increment the progress bar by")
                .index(1)
                .takes_value(true)
                .validator(positive_number),
        )
    }

    // Runs the progress start
    pub fn run(matches: &clap::ArgMatches) {
        progress_bar::advance(matches.value_of_u64(INCREMENT)).unwrap();
    }
}

mod stop {
    use crate::descriptions::progress;
    use clap::{App, Arg};
    use cto_ai::ux::progress_bar;

    static MESSAGE: &str = "message";

    // Init the cli commands for progressbar stop
    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new("stop").about(progress::STOP).arg(
            Arg::with_name(MESSAGE)
                .long(MESSAGE)
                .short("m")
                .multiple(true)
                .help("Message to be displayed with the complete progress bar.")
                .takes_value(true),
        )
    }

    // Runs the stop command for progress bar
    pub fn run(matches: &clap::ArgMatches) {
        progress_bar::stop(
            matches
                .values_of(MESSAGE)
                .map(|values| {
                    let words: Vec<&str> = values.collect();
                    words.join(" ")
                })
                .as_deref(),
        )
        .unwrap();
    }
}

use crate::descriptions;
use clap::{App, ArgMatches};

pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("progressbar")
        .about(descriptions::PROGRESSBAR)
        .subcommand(start::init_cli_command())
        .subcommand(advance::init_cli_command())
        .subcommand(stop::init_cli_command())
}

pub fn run(matches: &ArgMatches) {
    match matches.subcommand() {
        ("start", Some(start_matches)) => start::run(start_matches),
        ("advance", Some(advance_matches)) => advance::run(advance_matches),
        ("stop", Some(stop_matches)) => stop::run(stop_matches),
        _ => panic!("Oops. No progress bar command found"),
    }
}
