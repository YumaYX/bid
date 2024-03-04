use std::error::Error;
use clap::{App, Arg};

pub fn binary_digit(num: u8) -> String {
    format!("{:08b}", num)
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    let matches = App::new("bid")
        .about("Binary digits")
        .arg(
            Arg::with_name("decimal_number")
                .value_name("DECIMAL NUMBER(0-255)")
                .required(false)
                .max_values(1),
        )
        .get_matches();

    if let Some(number_str) = matches.value_of("decimal_number") {
        if let Ok(value) = number_str.parse::<u8>() {
            println!("{}", binary_digit(value));
            return Ok(());
        }
    } else {
        for n in 0..=255 {
            println!("{} {}", n, binary_digit(n));
        }
        return Ok(());
    }

    eprintln!("{}", matches.usage());
    Err("Invalid input or missing decimal number".into())
}
