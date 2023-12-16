use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct CountryGraph {
    pub adjacency_list: HashMap<String, HashSet<String>>,
}

impl CountryGraph {
    pub fn new() -> Self {
        CountryGraph {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, source: &str, target: &str) {
        self.adjacency_list.entry(source.to_string()).or_insert_with(HashSet::new).insert(target.to_string());
        self.adjacency_list.entry(target.to_string()).or_insert_with(HashSet::new).insert(source.to_string());
    }

    pub fn get_neighbors(&self, country: &str) -> Option<&HashSet<String>> {
        self.adjacency_list.get(country)
    }

    pub fn has_country(&self, country: &str) -> bool {
        self.adjacency_list.contains_key(country)
    }
}