use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use csv::ReaderBuilder;

pub struct CountryData {
    pub country: String,
    pub region: String,
    pub subregion: String,
    pub population_density: f64,
    pub urban_population_growth: f64,
}


pub fn load_data(file_path: &str) -> Result<Vec<CountryData>, io::Error> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    let mut data = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let country_data = CountryData {
            country: record[1].to_string(),
            region: record[2].to_string(),
            subregion: record[3].to_string(),
            population_density: record[6].parse().unwrap_or_default(),
            urban_population_growth: record[7].parse().unwrap_or_default(),
        };

        data.push(country_data);
    }

    Ok(data)
}