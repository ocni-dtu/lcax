use lcax_models::project;

#[test]
fn test_lcax() -> Result<(), String> {
    let project = project::Project::default();
    assert!(project.id.is_empty());

    Ok(())
}
