pub mod addressbook {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}

use std::{fs, time};
use std::collections::HashMap;
use std::error::Error;
use std::io::{BufReader, Read, Write, BufWriter};
use prost::Message;
use prost_types::Timestamp;

const DB_FILE_PATH: &str = "addressbook.db";

pub struct Config {
    pub command: String,
    pub params: HashMap<String, String>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        args.next();

        let command = match args.next() {
            Some(command) => command,
            None => return Err("Command not found"),
        };

        let mut params: HashMap<String, String> = HashMap::new();
        while let Some(arg) = args.next() {
            if arg.starts_with("--") {
                match args.next() {
                    Some(param) => params.insert(arg, param),
                    None => return Err("Missing parameter after `--arg`"),
                };
            } else {return Err("Expected arg starts with `--`");}
        };
        Ok(Config {command, params})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = open_db_file(DB_FILE_PATH);
    
    match config.command.as_ref() {
        "add" => {
            println!("{:#?}", config.params);
            if config.params["--kind"] == "per" || config.params["--kind"] == "person" {
                add_person(&mut f, &config.params["--name"], &config.params["--email"], &config.params["--phone"], &config.params["--type"]);
            }
            else if config.params["--kind"] == "cie" || config.params["--kind"] == "company" {
                add_company(&mut f, &config.params["--name"], &config.params["--email"], &config.params["--dep"], &config.params["--phone"], &config.params["--type"]);
            }
            Ok(())
        },
        "list" => { 
            list_contacts(&mut f, &config.params["--redact"]);
            Ok(())
        },
        _ => Err("Command not found")?
    }
}

fn open_db_file(file_path: &str) -> fs::File {
    fs::OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(file_path)
    .unwrap()
}

fn read_from_db(f: &mut fs::File) -> addressbook::AddressBook {
    let mut buf_reader = BufReader::new(f);
    let mut contents = Vec::new();

    buf_reader.read_to_end(&mut contents).unwrap();
    addressbook::AddressBook::decode(contents.as_slice()).unwrap()
}

fn write_to_db(f: &mut fs::File, book: addressbook::AddressBook) {
    let mut buf_writer = BufWriter::new(f);
    let contents = book.encode_to_vec();
    buf_writer.write_all(&contents).unwrap();
    buf_writer.flush().unwrap();
}

fn str_to_phone_type(s: &str) -> i32 {
    match s {
        "mobile" => 1,
        "home" => 2,
        "work" => 3,
        _ => 0
    }
}

fn str_to_deps(s: &str) -> i32 {
    match s {
        "hr" => 1,
        "cs" => 2,
        _ => 0
    }
}

fn redact_private_info(contact: &addressbook::Contact) -> addressbook::Contact {
    
    match contact.kind.clone().unwrap() {
        addressbook::contact::Kind::Person(person) => {
            let mut new_contact = contact.clone();

            let phones = person.clone().phones;

            let mut new_person = person.clone();
            let mut new_phones = Vec::new();
            
            for mut phone in phones {
                let length = phone.number.len();
                phone.number = "*".repeat(length);
                new_phones.push(phone);
            }

            new_person.phones = new_phones;
            new_contact.kind = Some(addressbook::contact::Kind::Person(new_person));

            return new_contact          
        },
        addressbook::contact::Kind::Company(company) => {
            let mut new_contact = contact.clone();

            let phones = company.clone().phones;
            let emails = company.clone().emails;

            let mut new_company = company.clone();
            let mut new_phones = Vec::new();
            let mut new_emails = Vec::new();

            for mut phone in phones {
                let length = phone.number.len();
                phone.number = "*".repeat(length);
                new_phones.push(phone);
            }

            for mut email in emails {
                let length = email.email.len();
                email.email = "*".repeat(length);
                new_emails.push(email);
            }

            new_company.phones = new_phones;
            new_company.emails = new_emails;
            new_contact.kind = Some(addressbook::contact::Kind::Company(new_company));

            return new_contact
        }
    };
    
}

fn add_person(f: &mut fs::File, name: &str, email: &str, phone: &str, phone_type: &str) {
    let mut book = read_from_db(f);
    let mut person: addressbook::Person;

    if book.contacts.contains_key(name) {
        let kind = book.contacts.get(name).unwrap().kind.clone().unwrap();

        match kind {
            addressbook::contact::Kind::Person(p) => {person = p},
            addressbook::contact::Kind::Company(_) => {panic!("Company")}
        }
    }
    else {
        person = addressbook::Person::default();
    }

    person.email = email.to_string();
    let mut nb = addressbook::person::PhoneNumber::default();
    nb.number = phone.to_string();
    nb.r#type = str_to_phone_type(phone_type);
    person.phones.push(nb);

    let mut contact = addressbook::Contact::default();
    let mut update_ts = Timestamp::default();
    let duration = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
    update_ts.seconds = duration.as_secs() as i64;
    update_ts.nanos = duration.subsec_nanos() as i32;

    contact.last_updated = Some(update_ts);
    contact.kind = Some(addressbook::contact::Kind::Person(person));
    book.contacts.insert(String::from(name), contact);

    write_to_db(f, book);
}

fn add_company(f: &mut fs::File, name: &str, email: &str, email_dep: &str, phone: &str, phone_dep: &str) {
    let mut book = read_from_db(f);

    let mut company: addressbook::Company;
    if book.contacts.contains_key(name) {
        let kind: addressbook::contact::Kind = book.contacts.get(name).unwrap().kind.clone().unwrap();
        match kind {
            addressbook::contact::Kind::Company(comp) => {company = comp},
            addressbook::contact::Kind::Person(_) => {panic!("Person")}
        }
    }
    else {
        company = addressbook::Company::default();
    }

    let mut addr = addressbook::company::EmailAddress::default();
    addr.email = email.to_string();
    addr.department = str_to_deps(email_dep);
    company.emails.push(addr);

    let mut nb = addressbook::company::PhoneNumber::default();
    nb.number = phone.to_string();
    nb.department = str_to_deps(phone_dep);
    company.phones.push(nb);

    let mut contact = addressbook::Contact::default();
    let mut update_ts = Timestamp::default();
    let duration = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
    update_ts.seconds = duration.as_secs() as i64;
    update_ts.nanos = duration.subsec_nanos() as i32;

    contact.last_updated = Some(update_ts);
    contact.kind = Some(addressbook::contact::Kind::Company(company));
    book.contacts.insert(String::from(name), contact);

    write_to_db(f, book);
}

fn list_contacts(f: &mut fs::File, _redact: &str) {
    let book = read_from_db(f);

    let mut keys: Vec<&String> = book.contacts.keys().collect();
    keys.sort();

    for name in keys {
        let contact = book.contacts.get(name).unwrap();

        if _redact == "true"{
            let private_contact = redact_private_info(contact);
            println!("name: {}", name);
            println!("last updated: {:?}", private_contact.last_updated.unwrap());
            println!("{:#?}", private_contact);
            println!("----------------------------------");
        }
        else {
            println!("name: {}", name);
            println!("last updated: {:?}", contact.last_updated.unwrap());
            println!("{:#?}", contact);
            println!("----------------------------------");
        }
        
    }
}
