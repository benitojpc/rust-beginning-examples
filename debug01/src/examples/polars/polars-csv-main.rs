/*
    To run this script: cargo run --example polars-csv-main
*/

use polars::prelude::*;
use polars::frame::DataFrame;
use std::path::Path;

fn read_data_from_csv( csv_file_path: &Path ) -> DataFrame {
    CsvReader::from_path( csv_file_path )
                                    .expect( "Cannot open file." )
                                    .has_header( true )
                                    .finish() 
                                    .unwrap()
}

fn main()  {
    let data_file_path : &Path = Path::new( "csvs/salaries.csv" );
    let data_df = read_data_from_csv( data_file_path );

    println!( "{:?}", data_df );
}
