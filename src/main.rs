use std::fs::{ OpenOptions, File };
use std::io::BufReader;
use std::io::prelude::*;
use clap::{ Arg, App };
use diligent_date_parser::parse_date;

mod error;
use error::Error;

fn main() {
    let args = App::new("NoNonSense CSV Trends")
        .version("1.0")
        .author("Nolan Rosen")
        .about("Concieved to help people identify reoccuring bills and/or subscriptions without 3rd party tooling")
        .arg(Arg::new("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1)
        ).get_matches();
    
    let file_name = args.value_of("INPUT").unwrap();
    let reader = match open_file(file_name) {
        Ok(f) => f,
        Err(e) => return eprintln!("{}", e)
    };
    for line in reader.lines().map(|l| l.unwrap()) {
        // TODO: Add custom delimiter support
        let item_iter = line.split(",");
        for item in item_iter {
            // dbg!(item);
            if let Some(date) = parse_date(item) {
                println!("Found Date: {}", date);
            }
        }
    }
}

fn open_file(file_name: &str) -> Result<BufReader<File>, Error> {
    let file = OpenOptions::new()
        .read(true)
        .open(file_name)?;
    return Ok(BufReader::new(file));
}
