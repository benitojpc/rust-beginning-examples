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

#[allow(dead_code)]
#[allow(unused_variables)]
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

// using serde
#[allow(dead_code)]
#[allow(unused_variables)]
fn example_02() -> Result< (), Box<dyn std::error::Error> >  {
    let ruta_file : String = String::from("csvs/salaries.csv") ; 
    let fopen = std::fs::File::open( ruta_file )?;
    let mut rdr = csv::Reader::from_reader( fopen );

    let mut contador = 0;
    let tope = 5;
    for record in rdr.deserialize() {
        if contador < tope {
            let record : SalaryRecord = record?;
            println!( "{} {} {} {} {} {} {} {} {} {} {} ",
                        record.work_year,
                        record.experience_level,
                        record.employment_type,
                        record.job_title,
                        record.salary,
                        record.salary_currency,
                        record.salary_in_usd,
                        record.employee_residence,
                        record.remote_ratio,
                        record.company_location,
                        record.company_size);
            contador += 1;
        }
    }

    Ok(())
}

fn main() {
    //let result = example_01();
    let result = example_02();
    println!( "{:?}", result );
}