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
