use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TaxData {
    state: String,
    zipcode: String,
    agi_stub: i32,
    n1: f64,
    mars1: f64,
    mars2: f64,
    mars4: f64,
    elf: f64,
    cprep: f64,
    prep: f64,
    dir_dep: f64,
    elderly: f64,
    a00100: f64,
    n02650: f64,
    a02650: f64,
    n00200: f64,
    a00200: f64,
}

#[allow(dead_code)]
impl TaxData {
    pub fn format(&self) -> String {
        format!("{:#?}", self)
    }
}
