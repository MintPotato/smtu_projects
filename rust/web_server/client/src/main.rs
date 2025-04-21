
use std::thread::sleep;
use std::time::Duration;
use client::{establish_connection, gen_data, send_data};

fn main() {
    let mut event_id = 0;

    loop {
        let tcp_con = establish_connection();

        match &tcp_con {
            Ok(con) => {
                let data = gen_data(&event_id);
                event_id += 1;

                match send_data(con, &data) {
                    Ok(()) => {println!("Данные отправлены")}
                    Err(e) => {eprintln!("Данные не могут быть отправлены: {e:#?}")}
                }

                sleep(Duration::from_secs(5));
            }
            Err(e) => {
                eprintln!("Соединение не установлено: {e:#?}");
                sleep(Duration::from_secs(2));
            }
        }
    }
}