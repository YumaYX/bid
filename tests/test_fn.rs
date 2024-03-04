#[cfg(test)]
mod tests {
    #[test]
    fn test_binary_digit() {
        assert_eq!(bid::binary_digit(0), "00000000");
        assert_eq!(bid::binary_digit(255), "11111111");
        assert_eq!(bid::binary_digit(170), "10101010");
        assert_eq!(bid::binary_digit(123), "01111011");
    }
}

use assert_cmd::Command;
use std::error::Error;

const PRG: &str = "bid";
type TestResult = Result<(), Box<dyn Error>>;

fn test_run(args: &[&str], expected: String) -> TestResult {
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn test_case_1() -> TestResult {
    test_run(&["0"], "00000000\n".to_string())
}


#[test]
fn test_case_2() -> TestResult {
    test_run(&["255"], "11111111\n".to_string())
}
