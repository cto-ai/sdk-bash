use crate::descriptions;
use clap::{App, Arg, ArgMatches};
use cto_ai::sdk::Track;

pub const CMD: &str = "track";

static EVENT: &str = "event";
static ERROR: &str = "error";
static PARAMS: &str = "params";

pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new(CMD)
        .about(descriptions::TRACK)
        .arg(
            Arg::with_name(EVENT)
                .index(1)
                .help("The event name to track")
                .value_name("EVENT")
                .required(true),
        )
        .arg(
            Arg::with_name(ERROR)
                .long(ERROR)
                .short("e")
                .help("An error string to report")
                .value_name("ERROR"),
        )
        .arg(
            Arg::with_name(PARAMS)
                .help("Parameters for the tracking event. Takes key:value metadata entries or tags as strings without colons")
                .takes_value(true)
                .multiple(true)
                .value_name("PARAM")
                )
}

// Runs the print command
pub fn run(matches: &ArgMatches) {
    let mut track = Track::new(matches.value_of(EVENT).unwrap());

    if let Some(error) = matches.value_of(ERROR) {
        track = track.error(error);
    }

    if let Some(params) = matches.values_of(PARAMS) {
        for param in params {
            let pieces: Vec<&str> = param.splitn(2, ':').collect();
            track = if pieces.len() == 2 {
                track.metadata(pieces[0], pieces[1])
            } else {
                track.tag(pieces[0])
            }
        }
    }

    track.send().unwrap();
}
