use std::io::{stdin, stdout, Write, IsTerminal, Read};
use clap::Parser;
use log::LevelFilter;

mod token;
mod shutting_yard;
mod tokenize;
mod evaluate;

use evaluate::compute;

#[derive(Parser, Debug)]
struct Args {
    /// print debug symbols
    #[arg(short, default_value_t=false)]
    debug: bool,

    /// print trace symbols
    #[arg(short, default_value_t=false)]
    trace: bool,

    /// string to compute
    input: Option<String>
}

fn main() {
    let args = Args::parse();

    simple_logging::log_to_stderr(if args.trace {
        LevelFilter::Trace
    } else if args.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });

    if let Some(input) = args.input {
        match compute(input) {
            Err(e) => eprintln!("{}", e),
            Ok(e) => println!("{}", e)
        }
        return;
    }

    if stdin().is_terminal() {
        loop {
            print!("> ");
            stdout().flush().unwrap();
            let mut input = String::new();
            if let Ok(n) = stdin().read_line(&mut input) {
                if n == 0 {
                    break;
                }
            } else {
                break;
            }
            match compute(input) {
                Err(e) => println!("Error: {}", e),
                Ok(e) => println!("{}", e)
            }
        }
    } else {
        let mut input = String::new();
        if let Ok(n) = stdin().read_to_string(&mut input) {
            if n == 0 {
                return;
            }
        } else {
            return;
        };
        match compute(input) {
            Err(e) => eprintln!("{}", e),
            Ok(e) => println!("{}", e)
        }
    }
}