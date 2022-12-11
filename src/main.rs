use std::{
    fs::File,
    io::{stdin, Read},
};

use clap::{builder::NonEmptyStringValueParser, Arg, Command};
use nix::{libc::STDIN_FILENO, unistd::isatty};

fn main() {
    if is_pipe() {
        let mut buf = String::new();
        match stdin().read_to_string(&mut buf) {
            Ok(_) => uooooo::run(buf),
            Err(_) => todo!(),
        }
    } else {
        let matches = cli().get_matches();

        match matches.get_one::<String>("FILE") {
            Some(path) => {
                let mut f = File::open(path).unwrap();
                let mut buf = String::new();
                match f.read_to_string(&mut buf) {
                    Ok(_) => uooooo::run(buf),
                    Err(_) => todo!(),
                }
            }
            None => cli().print_help().unwrap(),
        }
    }
}

fn cli() -> Command {
    Command::new("uooooo")
        .bin_name("uooooo")
        .version("0.1.0")
        .author("wat-aro <kazutas1008@gmail.com>")
        .about("A brainf**k like programming language.")
        .arg(
            Arg::new("FILE")
                .value_parser(NonEmptyStringValueParser::new())
                .help("program file"),
        )
}

fn is_pipe() -> bool {
    if let Ok(b) = isatty(STDIN_FILENO) {
        !b
    } else {
        false
    }
}
