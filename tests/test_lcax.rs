#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_lcax() -> Result<(), String> {
        lcax::project::LCAxProject {
            id: "2e257f19-50cc-47de-b54e-2af1add052eb".to_string(),
            name: "Test".to_string(),
            description: "Test Project".to_string(),
            comment: None,
            location: "DK".to_string(),
            format_version: "1.1.0".to_string(),
            lcia_method: None,
            classification_system: None,
            life_span: None,
            life_cycle_stages: vec![],
            impact_categories: vec![],
            emission_parts: HashMap::new(),
            results: None,
            meta_data: None,
        };
        Ok(())
    }
}
