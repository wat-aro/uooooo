use assert_cmd::Command;

#[cfg(test)]
pub fn uooooo() -> Command {
    let mut cmd = Command::cargo_bin("uooooo").unwrap();
    cmd.current_dir("tests/examples");
    cmd
}
