use rand::prelude::*;

use std::{fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, time::Duration};
use std::thread;
use std::error::Error;

fn main() {
    while true {
        if let Err(e) = try_to_connect("127.0.0.1:443") {
            println!("Karamba in trying to connect: {e:?}")
        };
        thread::sleep(Duration::from_millis(1000));
        
    }
}

fn try_to_connect(addr: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(addr).unwrap();

    for streams in listener.incoming() {
        // let stream = streams.unwrap();
        if let Err(e) = streams {
            println!("Karamba in trying ot get info from connections: {e:?}");
        } 
        else {
            let stream = streams.unwrap();
            handle_connection(stream);
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);    
    let req = buf_reader.lines().next().unwrap().unwrap();

    // TODO: Заменить возвращаемое значение с того что есть, на возвращение сгенерированного протобафом объекта с данными
    let (status, contents) = if req == "GET / HTTP/1.1" {
        let body = fs::read_to_string("./pages/main.html").unwrap();
        let (humidity, temperature) = generate_info();
        let contents = format!("{body}<p>Humidity: {humidity}</p><p>Temperature: {temperature}</p></body></html>");
        ("HTTP/1.1 200 OK", contents)
    } else {
        let contents = fs::read_to_string("./pages/404.html").unwrap();
        ("HTTP/1.1 404 NOT FOUND", contents)
    };
    
    let length = contents.len();
    // println!("{:?}", params);

    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn generate_info() -> (i32, i32){
    let mut rng = rand::rng();
    
    let humidity = rng.random_range(60..90);
    let temperature = rng.random_range(5..30);

    return (humidity, temperature)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {

    }
}
