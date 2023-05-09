use super::{DEFAULT, FLAG, MESSAGE, NAME};
use crate::descriptions::prompt;
use clap::{App, Arg};
use cto_ai::ux::prompt::{Datetime, Prompt};

pub const CMD: &str = "datetime";

static MAX: &str = "max";
static MIN: &str = "min";
static DATE: &str = "date";
static TIME: &str = "time";

// Init the cli commands for the datetime prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD)
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
            Arg::with_name(FLAG)
                .long(FLAG)
                .short("f")
                .help("Command line flag alias associated with this prompt")
                .value_name("FLAG"),
        )
        .arg(
            Arg::with_name(DEFAULT)
                .long(DEFAULT)
                .short("d")
                .help(
                    "Sets default value for datetime. \
                     Formats: '2019-12-11T21:37:12-08:00' or '2019-12-11T13:39:37Z' for full datetime\
                              '2019-12-11' for date only\
                              '13:35:00' for time only",
                )
                .value_name("DEFAULT VALUE"),
        )
        .arg(
            Arg::with_name(MIN)
                .long(MIN)
                .help(
                    "Sets minimum value for datetime. \
                     Formats: '2019-12-11T21:37:12-08:00' or '2019-12-11T13:39:37Z' for full datetime\
                              '2019-12-11' for date only\
                              '13:35:00' for time only",
                )
                .value_name("MIN"),
        )
        .arg(
            Arg::with_name(MAX)
                .long(MAX)
                .help(
                    "Sets maximum value for datetime. \
                     Formats: '2019-12-11T21:37:12-08:00' or '2019-12-11T13:39:37Z' for full datetime\
                              '2019-12-11' for date only\
                              '13:35:00' for time only",
                )
                .value_name("MAX"),
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

    if let Some(default) = matches.value_of(DEFAULT) {
        datetime = datetime.default_value(default);
    }

    if let Some(min) = matches.value_of(MIN) {
        datetime = datetime.minimum(min);
    }

    if let Some(max) = matches.value_of(MAX) {
        datetime = datetime.maximum(max);
    }

    if matches.is_present(DATE) {
        datetime = datetime.variant(DATE);
    }

    if matches.is_present(TIME) {
        datetime = datetime.variant(TIME);
    }

    if let Some(flag) = matches.value_of(FLAG) {
        datetime = datetime.flag(flag);
    }

    let final_val = datetime.execute().unwrap();
    println!("{}", final_val);
}
