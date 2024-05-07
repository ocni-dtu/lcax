#[cfg(test)]
mod tests {
    #[test]
    fn test_tech_flow() -> Result<(), String> {
        let tech_flow = lcax::techflow::TechFlow::new();
        assert!(!tech_flow.id.is_empty());

        Ok(())
    }
}
