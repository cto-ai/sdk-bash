use clap::{App, Arg};
use ops_sdk::ux::prompt::Editor;

// Init the cli commands for the Editor prompt
pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("editor")
        .about("It starts a new editor prompt")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Name of the editor")
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
                .help("Sets default value for editor")
                .value_name("DEFAULT VALUE"),
        )
}

// Runs the Editor prompt
pub fn run(editor_matches: &clap::ArgMatches) {
    let name = editor_matches.value_of("name").unwrap();
    let message = editor_matches.value_of("message").unwrap();

    let mut editor = Editor::new(name, message);

    if editor_matches.is_present("default value") {
        let default_value = editor_matches.value_of("default value").unwrap();
        editor = editor.default_value(default_value);
    }

    let final_value = editor.execute();
    println!("{}", final_value);
}
