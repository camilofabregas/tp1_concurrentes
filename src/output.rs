use std::collections::HashMap;
use crate::{site::Site, stat::Stat};
use serde::Serialize;

#[derive(Serialize)]
pub struct Output {
    padron: i32,
    sites: HashMap<String, Site>,
    tags: HashMap<String, Stat>,
    totals: HashMap<String, Vec<String>>,
}

impl Output {
    pub fn new(_padron: i32, _sites: HashMap<String, Site>, _tags: HashMap<String, Stat>, _totals: HashMap<String, Vec<String>>) -> Self {
        Self { padron: _padron, sites: _sites, tags: _tags, totals: _totals }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_new() -> Result<(), String> {
        let sites_test: HashMap<String, Site> = HashMap::new();
        let tags_test: HashMap<String, Stat> = HashMap::new();
        let totals_test: HashMap<String, Vec<String>> = HashMap::new();
        let output = Output::new(0, sites_test, tags_test, totals_test);

        assert!(output.padron == 0 && output.sites.len() == 0 && output.tags.len() == 0 && output.totals.len() == 0);
        Ok(())
    }
}