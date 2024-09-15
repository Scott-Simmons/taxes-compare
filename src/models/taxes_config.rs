use crate::models::tax_schedule::IncomeTaxSchedule;
use crate::Deserialize;
use std::collections::HashMap;
use std::fs;
/// A taxes config represents all information available.
#[derive(Deserialize, Debug, Clone)]
pub struct TaxesConfig {
    /// Mapping from country to its tax schedule.
    pub country_map: HashMap<String, IncomeTaxSchedule>,
}
impl TaxesConfig {
    pub fn new(config_path: &str) -> TaxesConfig {
        let file = fs::File::open(config_path).expect("File should open read only");
        let json: TaxesConfig = serde_json::from_reader(file).expect("JSON was not well formatted");
        json
    }
    pub fn get_country(&self, country: &str) -> Option<&IncomeTaxSchedule> {
        self.country_map.get(country)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::taxes_config::TaxesConfig;
    #[test]
    fn test_taxes_config() {
        let file_path = "test_data/valid_config.json";
        let taxes_config = TaxesConfig::new(&file_path);

        assert_eq!(taxes_config.country_map.len(), 2);
        assert!(taxes_config.country_map.contains_key("New Zealand"));
        assert!(taxes_config.country_map.contains_key("Australia"));

        assert_eq!(
            taxes_config
                .country_map
                .get("New Zealand")
                .unwrap()
                .schedule
                .len(),
            5
        );
        assert_eq!(
            taxes_config
                .country_map
                .get("Australia")
                .unwrap()
                .schedule
                .len(),
            5
        );
    }
}
