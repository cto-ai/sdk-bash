use super::{DEFAULT, MESSAGE, NAME};
use crate::descriptions::prompt;
use crate::validate::{datetime, DatetimeArg};
use clap::{App, Arg};
use cto_ai::ux::prompt::Datetime;

static MAX: &str = "max";
static MIN: &str = "min";
static DATE: &str = "date";
static TIME: &str = "time";

// Init the cli commands for the datetime prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("datetime")
        .about(prompt::DATETIME)
        .arg(
            Arg::with_name(NAME)
                .long(NAME)
                .short("n")
                .help("Name of the datetime prompt")
                .value_name("NAME")
                .required(true),
        )
        .arg(
            Arg::with_name(MESSAGE)
                .long(MESSAGE)
                .short("m")
                .help("Message to be displayed to the user")
                .required(true)
                .value_name("MESSAGE"),
        )
        .arg(
            Arg::with_name(DEFAULT)
                .long(DEFAULT)
                .short("d")
                .help(
                    "Sets default value for datetime. \
                     Formats: '2019-12-11T21:37:12-08:00' or '2019-12-11T13:39:37Z'",
                )
                .value_name("DEFAULT VALUE")
                .validator(datetime),
        )
        .arg(
            Arg::with_name(MIN)
                .long(MIN)
                .help(
                    "Sets minimum value for datetime. \
                     Formats: '2019-12-11T21:37:12-08:00' or '2019-12-11T13:39:37Z'",
                )
                .value_name("MIN")
                .validator(datetime),
        )
        .arg(
            Arg::with_name(MAX)
                .long(MAX)
                .help(
                    "Sets maximum value for datetime. \
                     Formats: '2019-12-11T21:37:12-08:00' or '2019-12-11T13:39:37Z'",
                )
                .value_name("MAX")
                .validator(datetime),
        )
        .arg(
            Arg::with_name(DATE)
                .long(DATE)
                .short("D")
                .help("Only get a date, with no associated time"),
        )
        .arg(
            Arg::with_name(TIME)
                .long(TIME)
                .short("t")
                .conflicts_with(DATE)
                .help("Only get a time, with no associated date"),
        )
}

// Runs the datetime prompt
pub fn run(matches: &clap::ArgMatches) {
    let mut datetime = Datetime::new(
        matches.value_of(NAME).unwrap(),
        matches.value_of(MESSAGE).unwrap(),
    );

    if let Some(default) = matches.value_of_datetime(DEFAULT) {
        datetime = datetime.default_value(default);
    }

    if let Some(min) = matches.value_of_datetime(MIN) {
        datetime = datetime.min(min);
    }

    if let Some(max) = matches.value_of_datetime(MAX) {
        datetime = datetime.max(max);
    }

    if matches.is_present(DATE) {
        datetime = datetime.variant(DATE);
    }

    if matches.is_present(TIME) {
        datetime = datetime.variant(TIME);
    }

    let final_val = datetime.execute().unwrap();
    println!("{}", final_val);
}
