pub mod start {
    use clap::{App, Arg};
    use ops_sdk::ux::spinner;

    // Init the cli commands for spinner start
    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new("start").about("It starts a spinner").arg(
            Arg::with_name("message")
                .long("message")
                .short("m")
                .help("Message to be displayed when the spinner starts.")
                .required(true)
                .value_name("MESSAGE"),
        )
    }

    // Runs the spinner start
    pub fn run(matches: &clap::ArgMatches) {
        let message = matches.value_of("message").unwrap();
        spinner::start(message);
    }
}

pub mod stop {
    use clap::{App, Arg};
    use ops_sdk::ux::spinner;

    // Init the cli commands for spinner stop
    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new("stop").about("It stops a spinner").arg(
            Arg::with_name("message")
                .long("message")
                .short("m")
                .help("Message to be displayed when the spinner stops.")
                .required(true)
                .value_name("MESSAGE"),
        )
    }

    // Runs the spinner stop
    pub fn run(matches: &clap::ArgMatches) {
        let message = matches.value_of("message").unwrap();
        spinner::stop(message);
    }
}
