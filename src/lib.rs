use std::error::Error;
use clap::{App, Arg};

pub fn binary_digit(num: u8) -> String {
    format!("{:08b}", num)
}

fn is_valid_range(value: String) -> Result<(), String> {
    match value.parse::<u8>() {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    let matches = App::new("bid")
        .about("Binary digits")
        .arg(
            Arg::with_name("decimal_number")
                .value_name("DECIMAL NUMBER(0-255)")
                .required(false)
                .max_values(1)
                .validator(is_valid_range)
        )
        .get_matches();

    if let Some(number_str) = matches.value_of("decimal_number") {
        if let Ok(value) = number_str.parse::<u8>() {
            println!("{}", binary_digit(value));
            return Ok(());
        }
        return Err("".into())
    }
    for n in 0..=255 {
        println!("{} {}", n, binary_digit(n));
    }
    Ok(())
}
