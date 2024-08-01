use xoev_xwasser::model::transport::VorgangTransportieren2010;

// const SOURCE: &'static str = include_str!("./quality_report_minimal.xml");

fn minimal_quality_report() -> String {
    std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join("tests/quality_report_minimal.xml"),
    )
    .unwrap()
}
fn minimal_quality_report_json() -> String {
    std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join("tests/quality_report_minimal.json"),
    )
    .unwrap()
}

#[cfg(feature = "schema")]
#[test]
fn test_minimal_quality_report_against_schema() -> anyhow::Result<()> {
    let s = minimal_quality_report();
    let validation = xoev_xwasser::schemas::XmlValidation::new()?;
    validation.validate(s.as_bytes())?;
    Ok(())
}

#[test]
fn test_minimal_quality_report_against_deserialize() -> anyhow::Result<()> {
    let s = minimal_quality_report();
    let e: Result<VorgangTransportieren2010, raxb::de::XmlDeserializeError> =
        raxb::de::from_str(&s);
    eprintln!("{e:#?}");
    let json = serde_json::to_string_pretty(&e.unwrap()).unwrap();
    eprintln!("{json}");
    std::fs::write("tests/quality_report_minimal.json", json)?;
    Ok(())
}

#[cfg(feature = "schema")]
#[test]
fn test_minimal_quality_report_against_serialize() -> anyhow::Result<()> {
    let s: VorgangTransportieren2010 = serde_json::from_str(&minimal_quality_report_json())?;
    let xml = raxb::ser::to_string_pretty_with_decl(&s)?;
    let validation = xoev_xwasser::schemas::XmlValidation::new()?;
    let result = validation.validate(xml.as_bytes());
    if let Err(e) = result {
        eprintln!("{e}");
    }
    std::fs::write("tests/quality_report_minimal_test_result.xml", xml.replace("https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0 xwasser.xsd", "https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0 ../schemas/V0_5_0/xwasser.xsd"))?;
    Ok(())
}
