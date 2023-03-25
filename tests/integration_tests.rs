use assert_cmd::Command;
use predicates::prelude::*;

fn cmd() -> Command {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.current_dir("tests/testdata");
    cmd
}

#[test]
fn test_unknown_file_format() {
    let mut cmd = cmd();
    cmd.arg("testdata.unknown")
        .arg("record_template.hbs")
        .arg("global_template.hbs");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unsupported file format: unknown"));
}

#[test]
fn test_json_input() {
    let mut cmd = cmd();
    cmd.arg("testdata.json")
        .arg("record_template.hbs")
        .arg("global_template.hbs")
        .arg("-o")
        .arg("-");

    cmd.assert()
        .success()
        .stdout(
            predicate::str::contains("Alice").and(predicate::str::contains("30").and(
                predicate::str::contains("New York").and(predicate::str::contains("Bob").and(
                    predicate::str::contains("25").and(predicate::str::contains("San Francisco")),
                )),
            )),
        );
}

#[test]
fn test_csv_input() {
    let mut cmd = cmd();
    cmd.arg("testdata.csv")
        .arg("record_template.hbs")
        .arg("global_template.hbs")
        .arg("-o")
        .arg("-");

    cmd.assert()
        .success()
        .stdout(
            predicate::str::contains("Alice").and(predicate::str::contains("30").and(
                predicate::str::contains("New York").and(predicate::str::contains("Bob").and(
                    predicate::str::contains("25").and(predicate::str::contains("San Francisco")),
                )),
            )),
        );
}
