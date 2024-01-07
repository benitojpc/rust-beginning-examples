/*
    To run this script: cargo run --example csv-read-files
*/

use std::error::Error;
use std::fs::File;
use std::path::Path;

fn read_csv <P: AsRef<Path>>( filename: P ) -> Result<(), Box<dyn Error>> {
    let file = File::open( filename )?;
    let mut rdr = csv::Reader::from_reader( file );

    for result in rdr.records() {
        let record = result?;
        println!( "{:?}", record );
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "csvs/salaries.csv";
    read_csv( filename )
}