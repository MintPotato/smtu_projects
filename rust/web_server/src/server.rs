use std::net::TcpListener;

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