use std::error::Error;
use clap::{App, Arg};

pub fn binary_digit(num: u8) -> String {
    format!("{:08b}", num)
}

type MyResult<T> = Result<T, Box<dyn Error>>;

fn parse_u8(val: &str) -> MyResult<u8> {
    if let Ok(n) = val.parse() {
        Ok(n)
    } else {
        Err(From::from(val))
    }
}

pub fn run() -> MyResult<()> {
    let matches = App::new("bid")
        .about("Binary digits")
        .arg(
            Arg::with_name("decimal_number")
                .value_name("DECIMAL NUMBER(0-255)")
                .required(false)
        )
        .get_matches();

    if matches.is_present("decimal_number") {
        let dn: Option<u8> = matches
            .value_of("decimal_number")
            .map(parse_u8)
            .transpose()
            .map_err(|_e| format!("{}", matches.usage()))?;
        println!("{}", binary_digit(dn.unwrap()));
    } else {
        for n in 0..=255 {
            println!("{} {}", n, binary_digit(n));
        };
    }
    Ok(())
}
