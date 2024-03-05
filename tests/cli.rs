use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

const PRG: &str = "bid";
type TestResult = Result<(), Box<dyn Error>>;

fn test_run(args: &[&str], expected: &'static str) -> TestResult {
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn test_case_1() -> TestResult {
    test_run(&["0"], "00000000\n")
}

#[test]
fn test_case_2() -> TestResult {
    test_run(&["255"], "11111111\n")
}

fn dies_invalid_args(args: &[&str], contain: &'static str) -> TestResult {
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .failure()
        .stderr(predicate::str::contains(contain));
    Ok(())
}

#[test]
fn test_case_3() -> TestResult {
    dies_invalid_args(&["-1"], "USAGE")
}

#[test]
fn test_case_4() -> TestResult {
    dies_invalid_args(&["256"], "USAGE")
}
