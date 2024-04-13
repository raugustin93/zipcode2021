use core::str;

use super::properties::TaxData;

pub fn print_tax_data_report(tax_data_vector: Vec<TaxData>) -> String {
    let mut report = String::new();
    let zip_code = &tax_data_vector[0].zipcode;
    println!("--- 2021 Tax Data for Zipcode: {} ---", zip_code);
    for tax_data in tax_data_vector {
        print_section(tax_data);
    }
    report
}

fn print_section(tax_data: TaxData) {
    let agi_stub_description = match &tax_data.agi_stub {
        1 => "$1 under $25,000",
        2 => "$25,000 under $50,000",
        3 => "$50,000 under $75,000",
        4 => "$75,000 under $100,000",
        5 => "$100,000 under $200,000",
        6 => "$200,000 or more",
        _ => "Unknown",
    };
    let descriptions: Vec<String> = vec![
        format!(
            "The State associated with the ZIP code                 {}",
            &tax_data.state
        ),
        format!(
            "Size of adjusted gross income        {}",
            agi_stub_description
        ),
    ];
    println!("\n");
    for description in descriptions {
        println!("{}", description)
    }
}
