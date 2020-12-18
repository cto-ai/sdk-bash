use crate::descriptions;
use clap::{App, Arg, ArgMatches};
use cto_ai::sdk::StartOp;

static WORKFLOW_NAME: &str = "workflowName";


pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("start-op")
        .about(descriptions::START_OP)
        .arg(
            Arg::with_name(WORKFLOW_NAME)
                .index(1)
                .help("The workflow to start")
                .value_name("WORKFLOW_NAME")
                .required(true),
        )
}

// Runs the print command
pub fn run(matches: &ArgMatches) {
    let start_op = StartOp::new(&matches.value_of(WORKFLOW_NAME).unwrap());
    start_op.send().unwrap();
}
