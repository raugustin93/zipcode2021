use core::str;
use std::{fs::File, io::Read};

use csv::Error;

use super::properties::TaxData;

pub fn get_csv_data(zip_code: &str) -> Result<Vec<TaxData>, Error> {
    let file_path = "data/2021.csv";
    let mut file = File::open(file_path)?;
    println!("You entered {} for your zip code", zip_code);

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lowercase_contents = contents.to_lowercase();
    let first_line: Vec<&str> = lowercase_contents.lines().take(1).collect();
    println!("{:?}", first_line.clone());
    let mut reader = csv::Reader::from_reader(lowercase_contents.as_bytes());
    let mut tax_returns: Vec<TaxData> = vec![];
    for returns in reader.deserialize() {
        let tax_return: TaxData = returns?;
        tax_returns.push(tax_return);
    }
    Ok(tax_returns)
}
