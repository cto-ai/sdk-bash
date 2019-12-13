use ops_sdk::ux::print;

// Runs the print command
pub fn run(matches: &clap::ArgMatches) {
    let text: Vec<&str> = matches.values_of("print_text").unwrap().collect();
    let fin = text.join(" ");
    print(&fin[..]);
}
