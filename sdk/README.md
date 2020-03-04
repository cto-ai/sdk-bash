# sdk-rs

## How to use

```rust
extern crate cto_ai;

use chrono::Utc;
use cto_ai::ux::print;
use cto_ai::ux::spinner;
use cto_ai::ux::ProgressBar;
use std::{thread, time};
// will remove #[allow(unused_imports)] when we figure out how to use secret and password locally
#[allow(unused_imports)]
use cto_ai::ux::prompt::{
    Autocomplete, Checkbox, Confirm, Datetime, Editor, Input, List, Number, Password, Secret,
};

fn sleep(t: u64) {
    let sleep_time = time::Duration::from_millis(t);
    thread::sleep(sleep_time);
}

fn main() {
    // ProgressBar

    let pb = ProgressBar::new(5)
        .initial(1)
        .message("Doing my complicated thing")
        .start();

    for _ in 0..5 {
        sleep(1000);
        pb.advance();
    }

    pb.stop();

    // Spinner

    spinner::start("Processing");
    sleep(2000);
    spinner::stop("Process Done!");

    // Print

    print("Some print");

    // Prompts

    let input_val = Input::new("input", "Input").execute();
    println!("{}", input_val);

    // let password_val = Password::new("password", "Password").execute();
    // println!("{}", password_val);

    let list_val = List::new("list", "List", vec!["foo".to_owned(), "bar".to_owned()]).execute();
    println!("{}", list_val);

    let checkbox_val = Checkbox::new(
        "my_checkbox",
        "Checkbox",
        vec!["foo".to_owned(), "bar".to_owned()],
    )
    .execute();

    println!("{}", checkbox_val);

    let number_val = Number::new("number", "Number").min(1).max(20).execute();
    println!("{}", number_val);

    let confirm_val = Confirm::new("confirm", "Confirm").execute();
    println!("{}", confirm_val);

    let editor_val = Editor::new("editor", "Editor")
        .default_value("Just hit ctrl + x")
        .execute();

    println!("{}", editor_val);

    let dt_val = Datetime::new("date", "DateTime")
        .variant("time")
        .min(Utc::now())
        .max(Utc::now())
        .execute();

    println!("{}", dt_val);

    let autocomplete_val = Autocomplete::new(
        "autocomplete",
        "Autocomplete",
        vec!["foo".to_owned(), "bar".to_owned()],
    )
    .execute();

    println!("{}", autocomplete_val);

    // let secret_val = Secret::new("my_secret", "Secret").execute();
    // println!("{}", secret_val);
}

```
