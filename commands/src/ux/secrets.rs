mod get {
    use crate::descriptions::secrets as descriptions;
    use clap::{App, Arg};
    use cto_ai::ux::secrets;

    pub const CMD: &str = "get";

    static KEY: &str = "key";

    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new(CMD).about(descriptions::GET).arg(
            Arg::with_name(KEY)
                .index(1)
                .help("The key of the desired secret in the secret store")
                .value_name("KEY")
                .required(true),
        )
    }

    pub fn run(matches: &clap::ArgMatches) {
        let final_value = secrets::Secrets::new()
            .get(matches.value_of(KEY).unwrap())
            .unwrap();
        println!("{}", final_value)
    }
}

mod set {
    use crate::descriptions::secrets as descriptions;
    use clap::{App, Arg};
    use cto_ai::ux::secrets;

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
                    .help("The key to set the secret under in the secret store")
                    .value_name("KEY")
                    .required(true),
            )
            .arg(
                Arg::with_name(VALUE)
                    .short("v")
                    .long("value")
                    .help("The value to set in the secret store")
                    .value_name("VALUE")
                    .required(true),
            )
    }

    pub fn run(matches: &clap::ArgMatches) {
        let final_value = secrets::Secrets::new()
            .set(
                matches.value_of(KEY).unwrap(),
                matches.value_of(VALUE).unwrap(),
            )
            .unwrap();
        println!("{}", final_value)
    }
}

use crate::descriptions;
use clap::{App, ArgMatches};

pub const CMD: &str = "secret";

pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD)
        .about(descriptions::SECRETS)
        .subcommand(get::init_cli_command())
        .subcommand(set::init_cli_command())
}

pub fn run(matches: &ArgMatches) {
    match matches.subcommand() {
        (get::CMD, Some(get_matches)) => get::run(get_matches),
        (set::CMD, Some(set_matches)) => set::run(set_matches),
        _ => println!("Oops. No secrets command found"),
    }
}
