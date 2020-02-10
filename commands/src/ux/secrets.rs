pub mod get {
    use clap::{App, Arg};
    use ops_sdk::ux::secrets;

    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new("get")
            .about("It gets a secret from the user's vault")
            .arg(
                Arg::with_name("name")
                    .short("n")
                    .long("name")
                    .help("Name of the secret")
                    .value_name("NAME")
                    .required(true),
            )
    }

    pub fn run(matches: &clap::ArgMatches) {
        let name = matches.value_of("name").unwrap();
        let final_value = secrets::Secrets::new().get(name);
        println!("{}", final_value)
    }
}

pub mod set {
    use clap::{App, Arg};
    use ops_sdk::ux::secrets;

    pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
        App::new("set")
            .about("It sets a secret in the user's vault")
            .arg(
                Arg::with_name("name")
                    .short("n")
                    .long("name")
                    .help("Name of the secret")
                    .value_name("NAME")
                    .required(true),
            )
            .arg(
                Arg::with_name("secret")
                    .short("s")
                    .long("secret")
                    .help("Value of the secret")
                    .value_name("SECRET")
                    .required(true),
            )
    }

    pub fn run(matches: &clap::ArgMatches) {
        let name = matches.value_of("name").unwrap();
        let secret = matches.value_of("secret").unwrap();

        let final_value = secrets::Secrets::new().set(name, secret);
        println!("{}", final_value)
    }
}
