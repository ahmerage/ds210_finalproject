use crate::CountryData;
use plotlib::scatter::Scatter;
use plotlib::view::{ContinuousView, View};
use plotlib::style::{Style, Marker};

/// Scatter plot
/// # Arguments
/// * `data` - A reference to a vector of CountryData
pub fn visualize_data(data: &[CountryData]) {
    let scatter = Scatter::from_slice(&data.iter().map(|d| (d.population_density, d.urban_population_growth)).collect::<Vec<_>>())
        .style(&Style::new().marker(Marker::Circle).size(2.0)); // Correctly using Style with Marker and size

    let view = ContinuousView::new()
        .add(&scatter)
        .x_label("Population Density")
        .y_label("Urban Population Growth")
        .title("Socioeconomic Analysis"); // Assuming there is a 'title' method in the correct documentation

