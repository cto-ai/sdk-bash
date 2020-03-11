use super::{DEFAULT, FLAG, MESSAGE, NAME};
use crate::descriptions::prompt;
use clap::{App, Arg};
use cto_ai::ux::prompt::{Editor, Prompt};

// Init the cli commands for the Editor prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("editor")
        .about(prompt::EDITOR)
        .arg(
            Arg::with_name(NAME)
                .long(NAME)
                .short("n")
                .help("Name of the editor prompt")
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
                .help("Sets initial value for in the editor")
                .value_name("DEFAULT"),
        )
        .arg(
            Arg::with_name(FLAG)
                .long(FLAG)
                .short("f")
                .help("Command line flag alias associated with this prompt")
                .value_name("FLAG"),
        )
}

// Runs the Editor prompt
pub fn run(matches: &clap::ArgMatches) {
    let mut editor = Editor::new(
        matches.value_of(NAME).unwrap(),
        matches.value_of(MESSAGE).unwrap(),
    );

    if let Some(default) = matches.value_of(DEFAULT) {
        editor = editor.default_value(default);
    }

    if let Some(flag) = matches.value_of(FLAG) {
        editor = editor.flag(flag);
    }

    let final_value = editor.execute().unwrap();
    println!("{}", final_value);
}
