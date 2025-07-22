use assert_cmd::Command;

fn run_test(input_args: &[&str], expected_file_name: &str) {
    let expected = std::fs::read_to_string(expected_file_name).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.args(input_args).assert().success().stdout(expected);
}

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));
}

#[test]
fn dies_conflict_s_d() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.args(&["-s", "-d,"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("error: the argument '-s'"));
}

#[test]
fn test_hello1() {
    run_test(&["Hello there"], "tests/expected/hello1.txt");
}

#[test]
fn test_hello2() {
    run_test(&["Hello", "there"], "tests/expected/hello2.txt");
}

#[test]
fn test_hello1n() {
    run_test(&["-n", "Hello  there"], "tests/expected/hello1.n.txt");
}

#[test]
fn test_hello2n() {
    run_test(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt");
}

#[test]
fn test_hello1s() {
    run_test(&["-s", "1", "2", "3", "4"], "tests/expected/hello1.s.txt");
}

#[test]
fn test_hello_delimiter() {
    run_test(
        &["-d,", "1", "2", "3", "4"],
        "tests/expected/hello.delim.txt",
    );
}
