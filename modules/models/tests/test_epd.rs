use lcax_models::epd;

#[test]
fn test_epd() -> Result<(), String> {
    let epd = epd::EPD::default();
    assert!(!epd.id.is_empty());

    Ok(())
}
