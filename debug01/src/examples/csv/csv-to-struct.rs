/*
    To run this script: cargo run --example csv-to-struct
*/

#[derive( Debug, serde::Deserialize )]
#[allow( dead_code )]
struct SalaryRecord {
    work_year: i32,
    experience_level: String,
    employment_type: String,
    job_title: String,
    salary: f64,
    salary_currency: String,
    salary_in_usd: f64,
    employee_residence: String,
    remote_ratio: f64,
    company_location: String,
    company_size: String,
} 

fn example_01() -> Result< (), Box<dyn std::error::Error> > {
    let ruta_file : String = String::from("csvs/salaries.csv") ; 
    let fopen = std::fs::File::open( ruta_file )?;
    let mut rdr = csv::Reader::from_reader( fopen );

    let headers = rdr.headers()?.clone();
    //println!( "{:?}", headers );
    
    for result in rdr.records() {
        let record = result ?;
        println!( "{:?}", record );
    }
    
    Ok(())
}

fn main() {
    let result = example_01();
    println!( "{:?}", result );
}