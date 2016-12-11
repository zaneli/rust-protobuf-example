extern crate protobuf;

use addressbook::{AddressBook, Person, Person_PhoneNumber as PhoneNumber,
                  Person_PhoneType as PhoneType};
use protobuf::{CodedInputStream, CodedOutputStream, Message, RepeatedField};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, stdin};
use std::path::Path;

pub fn execute(file_path: &str) -> Result<(), Box<Error>> {
    let mut address = AddressBook::new();

    let path = Path::new(file_path);
    if path.exists() {
        let file = try!(File::open(&path));
        let mut br = BufReader::new(file);
        let mut cis = CodedInputStream::from_buffered_reader(&mut br);
        try!(address.merge_from(&mut cis));
    }

    let person = try!(prompt_for_address());

    let mut people = address.take_people();
    people.push(person);
    address.set_people(people);

    let file = try!(File::create(&path));
    let mut bw = BufWriter::new(file);
    let mut cos = CodedOutputStream::new(&mut bw);
    try!(address.write_to(&mut cos));
    try!(cos.flush());
    Ok(())
}

fn prompt_for_address() -> Result<Person, Box<Error>> {
    let mut person = Person::new();
    let stdin = stdin();

    println!("Enter person ID: ");
    let mut id = String::new();
    try!(stdin.lock().read_line(&mut id));
    person.set_id(try!(id.trim().parse()));

    println!("Enter neme: ");
    let mut name = String::new();
    try!(stdin.lock().read_line(&mut name));
    person.set_name(name.trim().to_string());

    println!("Enter email address (blank for none): ");
    let mut email = String::new();
    try!(stdin.lock().read_line(&mut email));
    if !email.trim().is_empty() {
        person.set_email(email.trim().to_string());
    }

    let mut phones: RepeatedField<PhoneNumber> = RepeatedField::new();
    loop {
        let mut phone = PhoneNumber::new();

        println!("Enter a phone number (or leave blank to finish): ");
        let mut number = String::new();
        try!(stdin.lock().read_line(&mut number));
        if number.trim().is_empty() {
            break;
        }
        phone.set_number(number.trim().to_string());

        println!("Is this a mobile, home, or work phone? ");
        let mut phone_type = String::new();
        try!(stdin.lock().read_line(&mut phone_type));
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
