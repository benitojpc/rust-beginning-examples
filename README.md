# rust-beginning-examples
Rust beginning Examples


 To compile examples execute: cargo build 
 To run each examples: cargo run --example <name_of_script>
     This name is defined in Cargo.toml, under the section [[example]] with the following structure:
        [[example]]
        name        = "polars-csv-main"
        path          = "src/examples/polars/polars-csv-main.rs"

        [[example]]
        name        = "csv-read-files"
        path          = "src/examples/csv/csv-read-files.rs"
