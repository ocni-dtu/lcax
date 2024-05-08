use lcax_models::techflow;

#[test]
fn test_tech_flow() -> Result<(), String> {
    let tech_flow = techflow::TechFlow::new();
    assert!(!tech_flow.id.is_empty());

    Ok(())
}
