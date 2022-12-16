mod utils;

use utils::uooooo;

#[test]
fn basic() {
    uooooo().arg("ABC.uooooo").assert().success().stdout("ABC");
}

#[test]
fn abc_and_newline() {
    uooooo()
        .arg("ABC_and_newline.uooooo")
        .assert()
        .success()
        .stdout("ABC\n");
}

#[test]
fn abc_and_newline_next_ptr() {
    uooooo()
        .arg("ABC_and_newline_next_ptr.uooooo")
        .assert()
        .success()
        .stdout("ABC\n");
}

#[test]
fn abc_abc() {
    uooooo()
        .arg("ABCABC.uooooo")
        .assert()
        .success()
        .stdout("ABC\nABC\n");
}

#[test]
fn hello_world() {
    uooooo()
        .arg("hello_world.uooooo")
        .assert()
        .success()
        .stdout("Hello World!\n");
}
