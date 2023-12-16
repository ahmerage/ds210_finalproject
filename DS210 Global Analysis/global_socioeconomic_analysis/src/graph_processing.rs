use std::collections::{HashMap, HashSet, VecDeque};
use crate::data_loading::CountryData;

#[derive(Debug)]
pub struct CountryGraph {
    pub adjacency_list: HashMap<String, HashSet<String>>,
}


/// * `data` - A reference to a vector of CountryData
pub fn build_graph(data: &[CountryData]) -> CountryGraph {
    let mut graph = CountryGraph {
        adjacency_list: HashMap::new(),
    };

    for country_data in data {
        let entry = graph.adjacency_list.entry(country_data.country.clone()).or_insert(HashSet::new());

        for neighbor in data.iter().filter(|&d| are_countries_nearby(country_data, d)) {
            entry.insert(neighbor.country.clone());
        }
    }

    graph
}

/// Check if two countries are nearby based on their subregion
/// # Arguments
/// * `country1` - A reference to the first CountryData
/// * `country2` - A reference to the second CountryData
pub fn are_countries_nearby(country1: &CountryData, country2: &CountryData) -> bool {
    country1.subregion == country2.subregion
}

/// Six Degrees of Separation analysis starting from specific country
/// # Arguments
/// * `graph` - A reference to the CountryGraph
/// * `start_country` - The name of the starting country
/// * `max_degrees` - The maximum number of degrees to explore
pub fn six_degrees_of_separation(graph: &CountryGraph, start_country: &str, max_degrees: usize) {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut degrees = HashMap::new();

    queue.push_back(start_country.to_string());
    visited.insert(start_country.to_string());
    degrees.insert(start_country.to_string(), 0);

    while let Some(current_country) = queue.pop_front() {
        let current_degree = *degrees.get(&current_country).unwrap_or(&0);

        if current_degree > max_degrees {
            break; // Stop if we exceed the maximum degrees
        }

        println!("Degree {}: {}", current_degree, current_country);

        if let Some(neighbors) = graph.adjacency_list.get(&current_country) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back(neighbor.clone());
                    degrees.insert(neighbor.clone(), current_degree + 1);
                }
            }
        }
    }
}