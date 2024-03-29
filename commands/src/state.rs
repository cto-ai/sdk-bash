// THIS MODULE IS DEPRECATED

mod get {
    use crate::descriptions::state as descriptions;
    use clap::{App, Arg};
    use cto_ai::sdk::{get_all_state, get_state};

    static KEY: &str = "key";
    static ALL: &str = "all";

    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new("get")
            .about(descriptions::GET)
            .arg(
                Arg::with_name(KEY)
                    .index(1)
                    .help("The key of the desired value in the workflow state")
                    .value_name("KEY")
                    .required_unless(ALL),
            )
            .arg(
                Arg::with_name(ALL)
                    .long(ALL)
                    .short("a")
                    .help("Get a JSON representation of the full state"),
            )
    }

    pub fn run(matches: &clap::ArgMatches) {
        if matches.is_present(ALL) {
            let full_state = get_all_state().unwrap();
            println!("{}", serde_json::to_string_pretty(&full_state).unwrap());
        } else {
            let final_value: Option<serde_json::Value> =
                get_state(matches.value_of(KEY).unwrap()).unwrap();
            match final_value {
                None => println!(),
                Some(serde_json::Value::String(s)) => println!("{}", s),
                Some(v) => println!("{}", v),
            }
        }
    }
}

mod set {
    use crate::descriptions::state as descriptions;
    use clap::{App, Arg};
    use cto_ai::sdk::set_state;

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
                    .help("The key to set the value under in the workflow state")
                    .value_name("KEY")
                    .required(true),
            )
            .arg(
                Arg::with_name(VALUE)
                    .short("v")
                    .long("value")
                    .help("The value to set in the workflow state")
                    .value_name("VALUE")
                    .required(true),
            )
    }

    pub fn run(matches: &clap::ArgMatches) {
        set_state(
            matches.value_of(KEY).unwrap(),
            matches.value_of(VALUE).unwrap(),
        )
        .unwrap();
    }
}

use crate::descriptions;
use clap::{App, ArgMatches};

pub const CMD: &str = "state";

pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD)
        .about(descriptions::STATE)
        .subcommand(get::init_cli_command())
        .subcommand(set::init_cli_command())
}

pub fn run(matches: &ArgMatches) {
    match matches.subcommand() {
        ("get", Some(get_matches)) => get::run(get_matches),
        ("set", Some(set_matches)) => set::run(set_matches),
        _ => println!("Oops. No state command found"),
    }
}
