mod utils;

use utils::uooooo;

#[test]
fn basic() {
    uooooo().arg("ABC.bf").assert().success().stdout("ABC");
}

#[test]
fn stdin() {
    uooooo()
        .write_stdin("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.+.\n")
        .assert()
        .success()
        .stdout("ABC");
}

#[test]
fn abc_and_newline() {
    uooooo()
        .arg("ABC_and_newline.bf")
        .assert()
        .success()
        .stdout("ABC\n");
}

#[test]
fn abc_and_newline_next_ptr() {
    uooooo()
        .arg("ABC_and_newline_next_ptr.bf")
        .assert()
        .success()
        .stdout("ABC\n");
}

#[test]
fn abc_abc() {
    uooooo()
        .arg("ABCABC.bf")
        .assert()
        .success()
        .stdout("ABC\nABC\n");
}
