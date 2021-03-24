use clap::App;
use cto_ai::sdk::get_user;

pub fn init_cli_command<'a, 'b>() -> App<'a, 'b> {
    App::new("user")
        .about("Retrieves user information for user running the Op")
}

pub fn run() {
    let user = get_user().unwrap();
    println!("{}", serde_json::to_string_pretty(&user).unwrap())
}
