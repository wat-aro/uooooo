use std::{
    fs::File,
    io::{stdin, Read},
    process::exit,
};

use atty::{self, Stream};
use clap::{builder::NonEmptyStringValueParser, Arg, Command};

fn main() {
    if is_pipe() {
        let mut buf = String::new();
        match stdin().read_to_string(&mut buf) {
            Ok(_) => match uooooo::run(buf) {
                Ok(_) => exit(0),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    } else {
        let matches = cli().get_matches();

        match matches.get_one::<String>("FILE") {
            Some(path) => {
                let mut f = match File::open(path) {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("'{}': No such file.", e);
                        exit(1);
                    }
                };
                let mut buf = String::new();
                match f.read_to_string(&mut buf) {
                    Ok(_) => match uooooo::run(buf) {
                        Ok(_) => exit(0),
                        Err(_) => todo!(),
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                        exit(1);
                    }
                }
            }
            None => {
                cli().print_help().unwrap();
            }
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
    !atty::is(Stream::Stdin)
}
