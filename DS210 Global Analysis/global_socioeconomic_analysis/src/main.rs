use std::error::Error;
use crate::data_loading::load_data;
use crate::graph_processing::{build_graph, six_degrees_of_separation};
use crate::visualization::visualize_data;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "/Users/abbymerage/Desktop/netflix_titles.csv";

    let data = load_data(&file_path)?;
    let graph = build_graph(&data);
    let start_country = "Afghanistan";
    let max_degrees = 2; // Adjust as needed
    six_degrees_of_separation(&graph, start_country, max_degrees);

    visualize_data(&data);

    Ok(())
}