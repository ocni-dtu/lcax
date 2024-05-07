#[cfg(test)]
mod tests {

    #[test]
    fn test_epd() -> Result<(), String> {
        let epd = lcax::epd::EPD::new();
        assert!(!epd.id.is_empty());

        Ok(())
    }
}
