use std::{io, env};
use std::io::{BufWriter, Write};
use std::net::TcpStream;
use std::error::Error;
use std::time;
use prost;
use prost::Message;
use prost_types;
use rand::random_range;


// mod data {
//     include!(concat!(env!("OUT_DIR"), "/data.rs"));
// }

// pub fn establish_connection() -> Result<TcpStream, Box<dyn Error>> {
//     let adress = "127.0.0.1:443";
//     let stream = TcpStream::connect(adress);
//     match stream {
//         Ok(stream) => {
//             Ok(stream)
//         }
//         Err(e) => {
//             eprintln!("{}", e);
//             Err(e)
//         }
//     }
// }

// pub fn send_data(connector: &TcpStream, data: &data::Event) -> Result<(), Box<dyn Error>> {
//     let mut buf_writer = BufWriter::new(connector);
//     let mut buf = vec![];
//     match data.encode(&mut buf) {
//         Ok(()) => {
//             let buf_len = buf.len().to_string();
//             println!("buf len: {buf_len:#?}")
//         }
//         Err(e) => {
//             eprintln!("Encoding data error: {e:#?}")
//         }
//     }
//     match buf_writer.write_all(&buf) {
//         Ok(()) => {
//         }
//         Err(e) => {
//             eprintln!("Error writing data to stream: {e:#?}");
//         }
//     }
//     buf_writer.flush()
// }
