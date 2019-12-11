use clap::{App, Arg};
use ops_sdk::ux::prompt::Confirm;

// Init the cli commands fot the Confirm prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("confirm")
        .about("It starts a new confirm prompt")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Name of the confirm")
                .value_name("NAME")
                .required(true),
        )
        .arg(
            Arg::with_name("message")
                .long("message")
                .short("m")
                .help("Message to be displayed")
                .required(true)
                .value_name("MESSAGE"),
        )
        .arg(
            Arg::with_name("default true")
                .long("default-true")
                .short("t")
                .help("Sets true as default value for confirm"),
        )
        .arg(
            Arg::with_name("default false")
                .long("default-false")
                .short("f")
                .help("Sets false as default value for confirm"),
        )
}

// Runs the Confirm prompt
pub fn run(confirm_matches: &clap::ArgMatches) {
    let name = confirm_matches.value_of("name").unwrap();
    let message = confirm_matches.value_of("message").unwrap();

    let confirm_true = confirm_matches.is_present("default true");
    let confirm_false = confirm_matches.is_present("default false");

    if confirm_true && confirm_false {
        panic!("Please select either --default-true or --default-false");
    }

    let mut confirm = Confirm::new(name, message);

    if confirm_false {
        confirm = confirm.default_value(false);
    }

    if confirm_true {
        confirm = confirm.default_value(true);
    }

    let final_value = confirm.execute();
    println!("{}", final_value);
}
