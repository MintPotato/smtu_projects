use std::{io, env};
use std::io::{BufWriter, Write};
use std::net::TcpStream;
use std::error::Error;
use std::time;
use prost;
use prost::Message;
use prost_types;
use rand::random_range;


mod data {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}

pub fn establish_connection() -> Result<TcpStream, Box<dyn Error>> {
    let adress = "127.0.0.1:443";
    let stream = TcpStream::connect(adress);
    match stream {
        Ok(stream) => {
            Ok(stream)
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(Box::new(e))
        }
    }
}

pub fn send_data(connector: &TcpStream, data: &data::Event) -> Result<(), io::Error> {
    let mut buf_writer = BufWriter::new(connector);
    let mut buf = vec![];
    match data.encode(&mut buf) {
        Ok(()) => {
            let buf_len = buf.len().to_string();
            println!("buf len: {buf_len:#?}")
        }
        Err(e) => {
            eprintln!("Encoding data error: {e:#?}")
        }
    }
    match buf_writer.write_all(&buf) {
        Ok(()) => {
        }
        Err(e) => {
            eprintln!("Error writing data to stream: {e:#?}");
        }
    }
    buf_writer.flush()
}

pub fn gen_data(event_id: &i32) -> data::Event {
    let device_id = 1488;
    
    let humidity = random_range(40..70);
    let temperature = random_range(10..30);
    let mut timestamp = prost_types::Timestamp::default();

    let dur = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();

    timestamp.seconds = dur.as_secs() as i64;
    timestamp.nanos = dur.as_nanos() as i32;

    let mut data = data::event::Data::default();

    data.device_id = device_id;
    data.humidity = humidity;
    data.temperature = temperature;

    let mut event = data::Event::default();
    event.event_id = *event_id;
    event.read_time = Some(timestamp);
    event.event_data = Some(data);

    event
}
