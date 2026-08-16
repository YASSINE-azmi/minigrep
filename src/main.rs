use command_line_rust_book::{search, search_case_insensitive};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let arg: Vec<String> = env::args().collect();

    if arg.len() < 3 {
        eprintln!("Usage: cargo run -- grep <file_path>");
        process::exit(1);
    }

    let config = Config::new(&arg);

    if let Err(error) = grep(config.file_path, config.query, config.ignore_case) {
        eprintln!("Application error: {error}");
        process::exit(1);
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn new(arg: &[String]) -> Config {
        let query = arg[1].clone();
        let file_path = arg[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Config {
            query,
            file_path,
            ignore_case,
        }
    }
}

fn grep(file_path: String, query: String, ignor_case: bool) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let result = if ignor_case {
        search_case_insensitive(&query, &contents)
    } else {
        search(&query, &contents)
    };
    for line in result {
        println!("{}", line);
    }
    Ok(())
}
