mod get {
    use crate::descriptions::config as descriptions;
    use clap::{App, Arg};
    use cto_ai::sdk::get_config;

    static KEY: &str = "key";

    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new("get").about(descriptions::GET).arg(
            Arg::with_name(KEY)
                .index(1)
                .help("The key of the desired value in the user's config")
                .value_name("KEY")
                .required(true),
        )
    }

    pub fn run(matches: &clap::ArgMatches) {
        let final_value: Option<String> = get_config(matches.value_of(KEY).unwrap()).unwrap();
        println!("{}", final_value.unwrap_or_default())
    }
}

mod set {
    use crate::descriptions::config as descriptions;
    use clap::{App, Arg};
    use cto_ai::sdk::set_config;

    static KEY: &str = "key";
    static VALUE: &str = "value";

    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new("set")
            .about(descriptions::SET)
            .arg(
                Arg::with_name(KEY)
                    .short("k")
                    .long(KEY)
                    .alias("name")
                    .help("The key to set the value under in the workflow config")
                    .value_name("KEY")
                    .required(true),
            )
            .arg(
                Arg::with_name(VALUE)
                    .short("v")
                    .long("value")
                    .help("The value to set in the user's config")
                    .value_name("VALUE")
                    .required(true),
            )
    }

    pub fn run(matches: &clap::ArgMatches) {
        set_config(
            matches.value_of(KEY).unwrap(),
            matches.value_of(VALUE).unwrap(),
        )
        .unwrap();
    }
}

use crate::descriptions;
use clap::{App, ArgMatches};

pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("config")
        .about(descriptions::CONFIG)
        .subcommand(get::init_cli_command())
        .subcommand(set::init_cli_command())
}

pub fn run(matches: &ArgMatches) {
    match matches.subcommand() {
        ("get", Some(get_matches)) => get::run(get_matches),
        ("set", Some(set_matches)) => set::run(set_matches),
        _ => println!("Oops. No config command found"),
    }
}