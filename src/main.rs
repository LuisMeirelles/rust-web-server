mod app;
mod core;

use core::{address::Address, connection::Connection};

use std::env;
use std::ops::Index;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        let exec_path = args.index(0);

        eprintln!("Too few parameters");
        println!("Correct usage: {exec_path} <host> <port>");

        return ExitCode::FAILURE;
    }

    let address = Address {
        host: args.index(1).to_string(),
        port: args.index(2).to_string(),
    };

    Connection::listen(address);

    ExitCode::SUCCESS
}
