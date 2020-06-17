pub fn positive_number(input: String) -> Result<(), String> {
    let value: i64 = input
        .parse()
        .or_else(|_| Err("Must be an integer".to_owned()))?;

    if value > 0 {
        Ok(())
    } else {
        Err("Must be positive".to_owned())
    }
}

pub fn positive_or_zero(input: String) -> Result<(), String> {
    let value: i64 = input
        .parse()
        .or_else(|_| Err("Must be an integer".to_owned()))?;

    if value >= 0 {
        Ok(())
    } else {
        Err("Must not be negative".to_owned())
    }
}

pub fn numeric(input: String) -> Result<(), String> {
    let _value: i64 = input
        .parse()
        .or_else(|_| Err("Must be an integer".to_owned()))?;
    Ok(())
}

pub fn datetime(input: String) -> Result<(), String> {
    let _value = chrono::DateTime::parse_from_rfc3339(&input)
        .or_else(|_| Err("Must be a valid RFC3339 timestamp".to_owned()))?;
    Ok(())
}

pub trait NumericArg {
    fn value_of_u64(&self, name: impl AsRef<str>) -> Option<u64>;
    fn value_of_i32(&self, name: impl AsRef<str>) -> Option<i32>;
}

impl<'a> NumericArg for clap::ArgMatches<'a> {
    fn value_of_u64(&self, name: impl AsRef<str>) -> Option<u64> {
        self.value_of(name)?.parse().ok()
    }

    fn value_of_i32(&self, name: impl AsRef<str>) -> Option<i32> {
        self.value_of(name)?.parse().ok()
    }
}
