#[cfg(test)]
mod tests {
    #[test]
    fn test_lcax() -> Result<(), String> {
        let project = lcax::project::Project::new();
        assert!(!project.id.is_empty());

        Ok(())
    }
}
