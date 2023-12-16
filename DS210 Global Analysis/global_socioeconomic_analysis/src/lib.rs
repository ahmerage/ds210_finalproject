pub use data_loading::{load_data, CountryData};
pub use graph_processing::{CountryGraph, six_degrees_of_separation, are_countries_nearby};
pub use visualization::visualize_data;

mod data_loading;
mod graph_processing;
mod visualization;