use std::io::{BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::env;
use std::error::Error;
use std::time;
use prost::Message;
use postgres::{Client, NoTls};

mod data {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}

pub fn establish_connection() -> Result<TcpListener, Box<dyn Error>> {
    let adress = "0.0.0.0:443";
    let listener = TcpListener::bind(adress);

    match listener {
        Ok(listener) => {Ok(listener)}
        Err(e) => {eprintln!("{:#?}", e); Err(Box::new(e))}
    }
}

pub fn handle_con(stream: TcpStream, client: &mut Client) {
    let mut buf_reader = BufReader::new(&stream);
    let mut buf = vec![];
    let _ = buf_reader.read_to_end(&mut buf);

    let msg = data::Event::decode(&*buf);

    match msg {
        Ok(event) => {
            let event_id = event.event_id;
            let event_time = time::UNIX_EPOCH + time::Duration::new(event.read_time.unwrap().seconds as u64, event.read_time.unwrap().nanos as u32);

            let humidity = event.event_data.unwrap().humidity;
            let temperature = event.event_data.unwrap().temperature;
            let device_id = event.event_data.unwrap().device_id;

            println!("ID устройства: {device_id}\n ID события: {event_id:#?}\n Время события: {event_time:#?}\n Температура: {temperature:#?}\n Влажность: {humidity:#?}");
            db_add_data(client, &event).unwrap()
        }
        Err(e) => {eprintln!("Ошибка разкодировки {e:#?}")}
    }
}

pub fn db_connection() -> Result<Client, Box<dyn Error>> {
    let client = Client::connect("postgresql://postgres:1337@db:5432/rust_db", NoTls);
    
    match client {
        Ok(client) => {println!("Подключение к БД установлено"); Ok(client)}
        Err(e) => {eprintln!("Невозможно подключиться к БД: {:#?}", e); Err(Box::new(e))}
    }
}

pub fn db_create_table(client: &mut Client) -> Result<(), Box<dyn Error>> {

    match  client.batch_execute("CREATE TABLE IF NOT EXISTS DATA (
        id SERIAL PRIMARY KEY,
        device_id INTEGER,
        event_id INTEGER,
        event_time TIMESTAMP,
        humidity INTEGER,
        temperature INTEGER
    )"){
        Ok(_) => {Ok(())}
        Err(e) => {eprintln!("Ошибка создания таблицы: {:#?}", e); Err(Box::new(e))}
    }
}

pub fn db_add_data(client: &mut Client, data: &data::Event) -> Result<(), Box<dyn Error>> {
    let time = time::UNIX_EPOCH + time::Duration::new(data.read_time.unwrap().seconds as u64, data.read_time.unwrap().nanos as u32);

    match client.execute("INSERT INTO DATA (device_id, event_id, event_time, humidity, temperature) VALUES ($1, $2, $3, $4, $5)",
    &[&data.event_data.unwrap().device_id, &data.event_id, &time, &data.event_data.unwrap().humidity, &data.event_data.unwrap().temperature]) {
        Ok(_) => {println!("Добавил данные"); Ok(())}
        Err(e) => {eprintln!("Ошибка добавления данных в БД: {:#?}", e); Err(Box::new(e))}
        
    }
}