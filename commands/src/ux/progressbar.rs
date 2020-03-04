pub mod start {
    use clap::{App, Arg};
    use cto_ai::ux::progress_bar;

    // Init the cli commands for progressbar start
    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new("start")
            .about("It starts a Progress Bar")
            .arg(
                Arg::with_name("length")
                    .long("length")
                    .short("l")
                    .help("Sets the length of the Progress Bar.")
                    .required(true)
                    .value_name("LENGTH"),
            )
            .arg(
                Arg::with_name("message")
                    .long("message")
                    .short("m")
                    .help("Message to be displayed when the Progress Bar starts.")
                    .value_name("MESSAGE"),
            )
            .arg(
                Arg::with_name("initial")
                    .long("initial")
                    .short("i")
                    .help("Initial lenth of the Progress Bar.")
                    .value_name("<INITIAL>"),
            )
            .arg(
                Arg::with_name("increment by")
                    .long("increment-by")
                    .short("b")
                    .help("Step to increment the Progress Bar.")
                    .value_name("<INCREMENT BY>"),
            )
    }

    // Runs the progress start
    pub fn run(matches: &clap::ArgMatches) {
        let length = parse_number_for(matches, "length");
        let mut pb = progress_bar::ProgressBar::new(length);

        if matches.is_present("message") {
            let message = matches.value_of("message").unwrap();
            pb = pb.message(message);
        }

        if matches.is_present("initial") {
            let init = parse_number_for(matches, "initial");
            pb = pb.initial(init);
        }

        if matches.is_present("increment by") {
            let incr_by = parse_number_for(matches, "increment by");
            pb = pb.increment_by(incr_by);
        }

        pb.start();
    }

    fn parse_number_for(matches: &clap::ArgMatches, cmd: &str) -> u64 {
        let value: u64 = matches
            .value_of(cmd)
            .unwrap()
            .parse()
            .expect("Could not read number.");
        value
    }
}

pub mod advance {
    use clap::{App, Arg};
    use cto_ai::ux::progress_bar;

    // Init the cli commands for progressbar start
    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new("advance").about("It advances Progress Bar").arg(
            Arg::with_name("increment by")
                .help("Amount the progress will be incremented by.")
                .index(1)
                .takes_value(true),
        )
    }

    // Runs the progress start
    pub fn run(matches: &clap::ArgMatches) {
        let inc = if !matches.is_present("increment by") {
            None
        } else {
            let inc: u64 = matches
                .value_of("increment by")
                .unwrap()
                .parse()
                .expect("Could not read number.");
            Some(inc)
        };

        progress_bar::advance(inc);
    }
}

pub mod stop {
    use clap::{App, Arg};
    use cto_ai::ux::progress_bar;

    // Init the cli commands for progressbar start
    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new("stop").about("It stops Progress Bar").arg(
            Arg::with_name("message")
                .long("message")
                .short("m")
                .multiple(true)
                .help("Message to be displayed when the Progress Bar stops.")
                .takes_value(true),
        )
    }

    // Runs the stop command for progress bar
    pub fn run(matches: &clap::ArgMatches) {
        if !matches.is_present("message") {
            progress_bar::stop(None);
            return;
        }

        let text: Vec<&str> = matches.values_of("message").unwrap().collect();
        let message = text.join(" ");
        progress_bar::stop(Some(&message[..]));
    }
}
