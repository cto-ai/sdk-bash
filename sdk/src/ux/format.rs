use crate::sdk::get_interface_type;

pub fn bold(text: &str) -> String {
    if get_interface_type() == "slack" {
        format!("*{}*", text)
    } else {
        format!("\u{001b}[1m{}\u{001b}[0m", text)
    }
}

pub fn italic(text: &str) -> String {
    if get_interface_type() == "slack" {
        format!("_{}_", text)
    } else {
        format!("\u{001b}[3m{}\u{001b}[23m", text)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bold() {
        std::env::set_var("SDK_INTERFACE_TYPE", "slack");
        assert_eq!(bold("my test string"), "*my test string*",);

        std::env::set_var("SDK_INTERFACE_TYPE", "terminal");
        assert_eq!(
            bold("my test string"),
            "\u{001b}[1mmy test string\u{001b}[0m",
        );
    }

    #[test]
    fn test_italic() {
        std::env::set_var("SDK_INTERFACE_TYPE", "slack");
        assert_eq!(italic("my test string"), "_my test string_",);

        std::env::set_var("SDK_INTERFACE_TYPE", "terminal");
        assert_eq!(
            italic("my test string"),
            "\u{001b}[3mmy test string\u{001b}[23m",
        );
    }
}
