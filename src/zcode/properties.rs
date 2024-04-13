use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TaxData {
    pub state: String,
    pub zipcode: String,
    pub agi_stub: i32,
    pub n1: f64,
    pub mars1: f64,
    pub mars2: f64,
    pub mars4: f64,
    pub elf: f64,
    pub cprep: f64,
    pub prep: f64,
    pub dir_dep: f64,
    pub elderly: f64,
    pub a00100: f64,
    pub n02650: f64,
    pub a02650: f64,
    pub n00200: f64,
    pub a00200: f64,
}

#[allow(dead_code)]
impl TaxData {
    pub fn format(&self) -> String {
        format!("{:#?}", self)
    }
}
