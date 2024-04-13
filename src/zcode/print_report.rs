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
        format!("Number of returns             {}", tax_data.n1),
        format!("Number of single returns             {}", tax_data.mars1),
        format!("Number of joint returns             {}", tax_data.mars2),
        format!(
            "Number of Head of Household returns             {}",
            tax_data.mars4
        ),
        format!(
            "Number of electronically filed returns             {}",
            tax_data.elf
        ),
        format!(
            "Number of computer prepared paper returns             {}",
            tax_data.cprep
        ),
        format!(
            "Number of returns with paid preparer's signature            {}",
            tax_data.prep
        ),
        format!(
            "Number of returns with direct deposit           {}",
            tax_data.dir_dep
        ),
        format!("Number of elderly returns             {}", tax_data.elderly),
        format!("Adjust gross income (AGI)             {}", tax_data.a00100),
        format!("Total income amount            {}", tax_data.n02650),
        format!(
            "Number of returns with salaries and wages             {}",
            tax_data.n00200
        ),
        format!("Salaries and wages amount             {}", tax_data.a00200),
    ];
    println!("\n");
    for description in descriptions {
        println!("{}", description)
    }
}
