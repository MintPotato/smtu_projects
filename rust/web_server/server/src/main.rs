use core::time;
use std::thread::sleep;
use postgres::Client;
use server::{establish_connection, handle_con, db_create_table, db_connection};

fn main() {
    let listener_con = establish_connection();
    let mut db_client: Client;
    
    let db_con = db_connection();

    match db_con {
        Ok(client) => {
            db_client = client;
            break
        }
        Err(e) => {eprintln!("{:#?}", e)}
    }
    sleep(time::Duration::from_secs(3))

    
    match db_create_table(&mut db_client) {
        Ok(()) => {println!("Создал таблицу х2"); break}
        Err(e) => {eprintln!("{:#?}", e)}
    }
    sleep(time::Duration::from_secs(3));

    // println!("1");
    match &listener_con {
        Ok(listener) => {
            for stream in listener.incoming() {
                    // println!("{stream:#?}");
                    let stream = stream.unwrap();
                    handle_con(stream, &mut db_client);
                }
            }
        Err(e) => {eprintln!("{:#?}", e)}
        }
        sleep(time::Duration::from_secs(3));
}
