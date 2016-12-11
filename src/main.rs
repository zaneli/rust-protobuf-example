extern crate protobuf;

mod addressbook;
mod add_person;
mod list_people;

use std::{env, process};
use std::error::Error;
use std::io::{stderr, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    get_module_name(&args).and_then(|f| get_file_path(&args).and_then(f)).unwrap_or_else(|e| {
        let _ = stderr().write(&(e.to_string() + "\n").into_bytes());
        process::exit(-1);
    })
}

fn get_module_name(args: &Vec<String>) -> Result<fn(&str) -> Result<(), Box<Error>>, Box<Error>> {
    if args.len() >= 2 {
        match String::as_ref(&args[1]) {
            "add_person" => Ok(add_person::execute),
            "list_people" => Ok(list_people::execute),
            other => {
                Err(From::from(format!("Unexpected module name: {}. (expected 'add_person' or \
                                        'list_people')",
                                       other)))
            }
        }
    } else {
        Err(From::from("Usage: cargo run <module_name> <file_path>"))
    }
}

fn get_file_path(args: &Vec<String>) -> Result<&str, Box<Error>> {
    if args.len() >= 3 {
        Ok(&args[2])
    } else {
        Err(From::from("Usage: cargo run <module_name> <file_path>"))
    }
}
