pub struct CountryData {
    pub country: String,
    pub region: String,
    pub subregion: String,
    pub population_density: f64,
    pub urban_population_growth: f64,
}

impl CountryData {
    pub fn new(country: &str, region: &str, subregion: &str, population_density: f64, urban_population_growth: f64) -> Self {
        CountryData {
            country: country.to_string(),
            region: region.to_string(),
            subregion: subregion.to_string(),
            population_density,
            urban_population_growth,
        }
    }
}