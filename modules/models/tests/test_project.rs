use lcax_models::project;

#[test]
fn test_lcax() -> Result<(), String> {
    let project = project::Project::new();
    assert!(!project.id.is_empty());

    Ok(())
}
