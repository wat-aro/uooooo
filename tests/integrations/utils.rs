use assert_cmd::{cargo::cargo_bin_cmd, Command};

#[cfg(test)]
pub fn uooooo() -> Command {
    let mut cmd = cargo_bin_cmd!("uooooo");
    cmd.current_dir("tests/examples");
    cmd
}
