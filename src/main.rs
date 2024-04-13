#[macro_use]
extern crate serde_derive;

extern crate serde;

mod zcode;
use std::{
    fs::File,
    io::{self, Write},
};

use crate::zcode::{print_report::print_tax_data_report, reader::get_csv_data};

fn main() {
    // Prompt the user for input
    print!("Enter a five-digit ZIP code: ");
    io::stdout().flush().unwrap();

    // Read user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Remove trailing newline character
    let zip_code = input.trim();

    // Validate the ZIP code
    if zip_code.len() != 5 || !zip_code.chars().all(char::is_numeric) {
        println!("Invalid ZIP code. Please enter a five-digit numeric ZIP code.");
        return;
    }

    let zipcode_tax_data = get_csv_data(zip_code);
    if zipcode_tax_data.is_err() {
        println!("Error getting tax data");
    }

    if zipcode_tax_data.is_ok() {
        let report = print_tax_data_report(zipcode_tax_data.unwrap());
        println!("{:#?}", report)
    }
}
