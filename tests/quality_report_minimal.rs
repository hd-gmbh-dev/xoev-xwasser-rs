#[cfg(feature = "schema")]
#[test]
fn test_minimal_quality_report() -> anyhow::Result<()> {
    use xoev_xwasser::model::VorgangTransportieren2010;
    let source = include_str!("./quality_report_minimal.xml");
    let validation = xoev_xwasser::schemas::XmlValidation::new()?;
    validation.validate(source.as_bytes())?;
    let _: VorgangTransportieren2010 = raxb::de::from_str(source)?;
    Ok(())
}