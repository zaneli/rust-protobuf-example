extern crate protobuf;

use addressbook::{AddressBook, Person_PhoneType as PhoneType};
use protobuf::{parse_from_reader, ProtobufResult};
use protobuf::error::ProtobufError;
use std::fs::File;
use std::path::Path;

pub fn execute(file_path: &str) -> ProtobufResult<()> {
    let mut file = File::open(&Path::new(file_path)).map_err(ProtobufError::IoError)?;
    let address = parse_from_reader::<AddressBook>(&mut file)?;
    print(&address);
    Ok(())
}

fn print(address: &AddressBook) {
    for person in address.get_people() {
        println!("Person ID: {}", person.get_id());
        println!("  Name: {}", person.get_name());
        if person.has_email() {
            println!("  E-mail address: {}", person.get_email());
        }
        for phone in person.get_phones() {
            let number = phone.get_number();
            match phone.get_field_type() {
                PhoneType::MOBILE => println!("  Mobile phone #: {}", number),
                PhoneType::HOME => println!("  Home phone #: {}", number),
                PhoneType::WORK => println!("  Work phone #: {}", number),
            }
        }
    }
}
