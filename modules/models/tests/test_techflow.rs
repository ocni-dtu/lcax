use lcax_models::generic_impact_data;

#[test]
fn test_generic_data() -> Result<(), String> {
    let generic_data = generic_impact_data::GenericData::default();
    assert!(!generic_data.id.is_empty());

    Ok(())
}
