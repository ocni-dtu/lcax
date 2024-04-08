#[cfg(test)]
mod tests {
    use lcax::project::Location;
    use lcax::country::Country;

    #[test]
    fn test_lcax() -> Result<(), String> {
        lcax::project::LCAxProject {
            id: "2e257f19-50cc-47de-b54e-2af1add052eb".to_string(),
            name: "Test".to_string(),
            description: Some("Test Project".to_string()),
            comment: None,
            location: Location {
                country: Country::DNK,
                city: None,
                address: None,
            },
            owner: None,
            format_version: "1.1.0".to_string(),
            lcia_method: None,
            classification_system: None,
            reference_study_period: None,
            life_cycle_stages: vec![],
            impact_categories: vec![],
            results: None,
            project_info: None,
            project_phase: Default::default(),
            software_info: Default::default(),
            meta_data: None,
            assemblies: Default::default(),
        };
        Ok(())
    }
}
