extern crate protobuf;

mod addressbook;
mod add_person;
mod list_people;

use protobuf::error::ProtobufError;
use protobuf::ProtobufResult;
use std::io::{self, stderr, Write};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    get_module_name(&args)
        .map_err(ProtobufError::IoError)
        .and_then(|f| {
            get_file_path(&args)
                .map_err(ProtobufError::IoError)
                .and_then(f)
        })
        .unwrap_or_else(|e| {
            stderr().write_fmt(format_args!("{}\n", e)).unwrap();
            process::exit(-1);
        })
}

fn get_module_name(args: &Vec<String>) -> Result<fn(&str) -> ProtobufResult<()>, io::Error> {
    if args.len() >= 2 {
        match args[1].as_ref() {
            "add_person" => Ok(add_person::execute),
            "list_people" => Ok(list_people::execute),
            other => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Unexpected module name: {}. (expected \
                     'add_person' or 'list_people')",
                    other
                ),
            )),
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Usage: cargo run <module_name> <file_path>".to_string(),
        ))
    }
}

fn get_file_path(args: &Vec<String>) -> Result<&str, io::Error> {
    if args.len() >= 3 {
        Ok(&args[2])
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Usage: cargo run <module_name> <file_path>".to_string(),
        ))
    }
}
