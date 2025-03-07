use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|e| {
        eprintln!("App error: {e}");
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("App error: {e}");
        process::exit(1)
    }
}

// fn run(config: Config) -> Result<(), Box<dyn Error>>{
//     let contents = fs::read_to_string(config.file_path).expect("msg");
//     println!("With text:\n {contents}");
//     Ok(())
// }

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();
//     return Config{ query, file_path};
// }

// pub struct Config {
//     query: String,
//     file_path: String
// }

// impl Config {
//     fn build(args: &[String]) -> Result<Config, &'static str>   {
//         if args.len() < 3 {return Err("Not enough args")}
//         let query = args[1].clone();
//         let file_path = args[2].clone();
//         return Ok(Config{ query, file_path});
//     }
//     fn new(args: &[String]) -> Config {
//         if args.len() < 3 {panic!("Nor enough args")}
//         let query = args[1].clone();
//         let file_path = args[2].clone();
//         return Config{ query, file_path}
//     }
// }
