use crate::descriptions;
use chrono::prelude::*;
use clap::{App, Arg, ArgMatches};
use cto_ai::sdk::events;

static START: &str = "start";
static END: &str = "end";

pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("events")
        .about(descriptions::EVENTS)
        .arg(
            Arg::with_name(START)
                .long(START)
                .short("s")
                .required(true)
                .takes_value(true)
                .help("The start time for the event lookup")
                .value_name("START_TIME"),
        )
        .arg(
            Arg::with_name(END)
                .help("The end time for the event lookup. Defaults to the current time")
                .takes_value(true)
                .value_name("END_TIME"),
        )
}

// Runs the print command
pub fn run(matches: &ArgMatches) {
    let start = matches.value_of(START).unwrap();
    let end = matches
        .value_of(END)
        .map(|e| e.to_owned())
        .unwrap_or_else(|| Local::now().to_rfc3339());

    let event_list = events(&start, &end).unwrap();

    println!("{}", serde_json::to_string_pretty(&event_list).unwrap())
}
