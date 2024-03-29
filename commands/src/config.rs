mod get {
    use crate::descriptions::config as descriptions;
    use clap::{App, Arg};
    use cto_ai::sdk::{get_all_config, get_config};

    pub const CMD: &str = "get";

    static KEY: &str = "key";
    static ALL: &str = "all";

    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new(CMD)
            .about(descriptions::GET)
            .arg(
                Arg::with_name(KEY)
                    .index(1)
                    .help("The key of the desired value in the configuration store")
                    .value_name("KEY")
                    .required_unless(ALL),
            )
            .arg(
                Arg::with_name(ALL)
                    .long(ALL)
                    .short("a")
                    .help("Get a JSON representation of the full config"),
            )
    }

    pub fn run(matches: &clap::ArgMatches) {
        if matches.is_present(ALL) {
            let full_config = get_all_config().unwrap();
            println!("{}", serde_json::to_string_pretty(&full_config).unwrap());
        } else {
            let final_value: Option<String> = get_config(matches.value_of(KEY).unwrap()).unwrap();
            match final_value {
                None => println!(),
                Some(s) => println!("{}", s),
            }
        }
    }
}

mod set {
    use crate::descriptions::config as descriptions;
    use clap::{App, Arg};
    use cto_ai::sdk::set_config;

    pub const CMD: &str = "set";

    static KEY: &str = "key";
    static VALUE: &str = "value";

    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new(CMD)
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
                    .help("The value to set in the configuration store")
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

mod delete {
    use crate::descriptions::config as descriptions;
    use clap::{App, Arg};
    use cto_ai::sdk::delete_config;

    pub const CMD: &str = "delete";

    static KEY: &str = "key";

    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new(CMD)
            .about(descriptions::DELETE)
            .arg(
                Arg::with_name(KEY)
                    .index(1)
                    .help("The key of the desired value to be removed from the configuration store")
                    .value_name("KEY")
                    .required(true)
            )
    }

    pub fn run(matches: &clap::ArgMatches) {
        let final_value = delete_config(matches.value_of(KEY).unwrap()).unwrap();
        println!("{}", final_value)
    }
}

use crate::descriptions;
use clap::{App, ArgMatches};

pub const CMD: &str = "config";

pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD)
        .about(descriptions::CONFIG)
        .subcommand(get::init_cli_command())
        .subcommand(set::init_cli_command())
        .subcommand(delete::init_cli_command())
}

pub fn run(matches: &ArgMatches) {
    match matches.subcommand() {
        (get::CMD, Some(get_matches)) => get::run(get_matches),
        (set::CMD, Some(set_matches)) => set::run(set_matches),
        (delete::CMD, Some(delete_matches)) => delete::run(delete_matches),
        _ => println!("Oops. No config command found"),
    }
}
