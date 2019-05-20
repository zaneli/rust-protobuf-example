use crate::addressbook::{
    AddressBook, Person, Person_PhoneNumber as PhoneNumber, Person_PhoneType as PhoneType,
};
use protobuf::error::ProtobufError;
use protobuf::{CodedInputStream, CodedOutputStream, Message, ProtobufResult, RepeatedField};
use std::fs::File;
use std::io::{self, stdin, BufRead, BufReader};
use std::path::Path;

pub fn execute(file_path: &str) -> ProtobufResult<()> {
    let mut address = AddressBook::new();

    let path = Path::new(file_path);
    if path.exists() {
        let file = File::open(&path).map_err(ProtobufError::IoError)?;
        let mut br = BufReader::new(file);
        let mut cis = CodedInputStream::from_buffered_reader(&mut br);
        address.merge_from(&mut cis)?;
    }

    let person = prompt_for_address()?;

    let mut people = address.take_people();
    people.push(person);
    address.set_people(people);

    let mut file = File::create(&path).map_err(ProtobufError::IoError)?;
    let mut cos = CodedOutputStream::new(&mut file);
    address.write_to(&mut cos)?;
    cos.flush()?;
    Ok(())
}

fn prompt_for_address() -> ProtobufResult<Person> {
    let mut person = Person::new();
    let stdin = stdin();

    println!("Enter person ID: ");
    let mut id = String::new();
    stdin
        .lock()
        .read_line(&mut id)
        .map_err(ProtobufError::IoError)?;
    person.set_id(
        id.trim()
            .parse()
            .map_err(|e| ProtobufError::IoError(io::Error::new(io::ErrorKind::InvalidInput, e)))?,
    );

    println!("Enter neme: ");
    let mut name = String::new();
    stdin
        .lock()
        .read_line(&mut name)
        .map_err(ProtobufError::IoError)?;
    person.set_name(name.trim().to_string());

    println!("Enter email address (blank for none): ");
    let mut email = String::new();
    stdin
        .lock()
        .read_line(&mut email)
        .map_err(ProtobufError::IoError)?;
    if !email.trim().is_empty() {
        person.set_email(email.trim().to_string());
    }

    let mut phones: RepeatedField<PhoneNumber> = RepeatedField::new();
    loop {
        let mut phone = PhoneNumber::new();

        println!("Enter a phone number (or leave blank to finish): ");
        let mut number = String::new();
        stdin
            .lock()
            .read_line(&mut number)
            .map_err(ProtobufError::IoError)?;
        if number.trim().is_empty() {
            break;
        }
        phone.set_number(number.trim().to_string());

        println!("Is this a mobile, home, or work phone? ");
        let mut phone_type = String::new();
        stdin
            .lock()
            .read_line(&mut phone_type)
            .map_err(ProtobufError::IoError)?;
        match phone_type.trim() {
            "mobile" => phone.set_field_type(PhoneType::MOBILE),
            "home" => phone.set_field_type(PhoneType::HOME),
            "work" => phone.set_field_type(PhoneType::WORK),
            _ => println!("Unknown phone type.  Using default."),
        }

        phones.push(phone);
    }

    person.set_phones(phones);
    Ok(person)
}
