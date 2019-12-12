use chrono;
use clap::{App, Arg};
use ops_sdk::ux::prompt::Datetime;

// Init the cli commands for the datetime prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("datetime")
        .about("It starts a new datetime prompt")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Name of the datetime")
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
            Arg::with_name("default value")
                .long("default-value")
                .short("d")
                .help(
                    "Sets default value for datetime. \
                     Formats: '2019-12-11T21:37:12-08:00' or '2019-12-11T13:39:37Z'",
                )
                .value_name("DEFAULT VALUE"),
        )
        .arg(
            Arg::with_name("min")
                .long("min")
                .help(
                    "Sets minimum value for datetime. \
                     Formats: '2019-12-11T21:37:12-08:00' or '2019-12-11T13:39:37Z'",
                )
                .value_name("MIN"),
        )
        .arg(
            Arg::with_name("max")
                .long("max")
                .help(
                    "Sets maximum value for datetime. \
                     Formats: '2019-12-11T21:37:12-08:00' or '2019-12-11T13:39:37Z'",
                )
                .value_name("MAX"),
        )
        .arg(
            Arg::with_name("variant")
                .long("variant")
                .help(
                    "Specifies which time information to prompt for, either \
                     a date (day/month/year) or a time (hour/minute/second), or both.\
                     Values: ('date' | 'time' | 'datetime' ). Default is 'datetime'",
                )
                .value_name("MAX"),
        )
}

// Runs the datetime prompt
pub fn run(datetime_matches: &clap::ArgMatches) {
    let name = datetime_matches.value_of("name").unwrap();
    let message = datetime_matches.value_of("message").unwrap();

    let mut datetime = Datetime::new(name, message);

    if datetime_matches.is_present("default value") {
        let dt = parse_datetime_for(datetime_matches, "default value");
        datetime = datetime.default_value(dt);
    }

    if datetime_matches.is_present("min") {
        let dt = parse_datetime_for(datetime_matches, "min");
        datetime = datetime.min(dt);
    }

    if datetime_matches.is_present("max") {
        let dt = parse_datetime_for(datetime_matches, "max");
        datetime = datetime.max(dt);
    }

    if datetime_matches.is_present("variant") {
        let variant = datetime_matches.value_of("variant").unwrap();
        datetime = datetime.variant(variant);
    }

    let final_val = datetime.execute();
    println!("{}", final_val);
}

fn parse_datetime_for(matches: &clap::ArgMatches, cmd: &str) -> chrono::DateTime<chrono::Utc> {
    let value = matches.value_of(cmd).unwrap();
    let dt = chrono::DateTime::parse_from_rfc3339(value)
        .unwrap()
        .naive_utc();
    chrono::DateTime::from_utc(dt, chrono::Utc)
}
