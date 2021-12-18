use std::fs::{ OpenOptions, File };
use std::io::{ BufReader, Lines };
use std::io::prelude::*;
use std::str::FromStr;
use std::collections::{ HashMap };
use std::collections::hash_map::{ Entry, OccupiedEntry, VacantEntry };
use rust_decimal::Decimal;
use clap::{ Arg, App };

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
    // for line in reader.lines().map(|l| l.unwrap()) {
    //     // TODO: Add custom delimiter support
    //     let item_iter = line.split(",");
    //     for item in item_iter {
    //         if let Some(date) = parse_date(item) {
    //             println!("Found Date: {}", date);
    //         }
    //     }
    // }
    let mut reader = reader.lines();
    reader.next();
    let dup_amounts = find_duplicates(reader, 3);
    for (k, i) in dup_amounts {
        if i > 1 {
            println!("Reoccuring amount of `{}` seen {} times", k, i);
        }
    }
}

fn find_duplicates(reader: Lines<BufReader<File>>, col: usize) -> HashMap<Decimal, usize> {
    let mut hasmap: HashMap<Decimal, usize> = HashMap::new();
    let iter = reader
        .map(|l| l.unwrap())
        .filter_map(|l| l.split(",")
            .nth(col)
            .and_then(|i| Some(i.to_string()))
        ).map(|ref i| Decimal::from_str(i).unwrap());
    for item in iter {
        match hasmap.entry(item) {
            Entry::Occupied(mut e) =>  { let _ = e.insert(e.get() + 1); },
            Entry::Vacant(e) => { let _ = e.insert(1); }
        }
    }
    return hasmap
}

fn open_file(file_name: &str) -> Result<BufReader<File>, Error> {
    let file = OpenOptions::new()
        .read(true)
        .open(file_name)?;
    return Ok(BufReader::new(file));
}
