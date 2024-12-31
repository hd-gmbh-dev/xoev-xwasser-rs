use xoev_xwasser::model::transport::VorgangTransportieren2010;

fn minimal_olb_report() -> String {
    std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join("tests/olb_report_minimal.xml"),
    )
    .unwrap()
}

#[cfg(feature = "schema")]
fn minimal_olb_report_json() -> String {
    std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join("tests/olb_report_minimal.json"),
    )
    .unwrap()
}

#[cfg(feature = "schema")]
#[test]
fn test_minimal_olb_report_against_schema() -> anyhow::Result<()> {
    let s = minimal_olb_report();
    let validation = xoev_xwasser::schemas::XmlValidation::new()?;
    validation.validate(s.as_bytes())?;
    Ok(())
}

#[test_log::test]
fn test_minimal_olb_report_against_deserialize() -> anyhow::Result<()> {
    let s = minimal_olb_report();
    let e: VorgangTransportieren2010 = raxb::de::from_str(&s)?;
    let json = serde_json::to_string_pretty(&e).unwrap();
    std::fs::write("tests/olb_report_minimal.json", json)?;
    std::fs::write("tests/olb_report_minimal.debug.log", format!("{e:#?}"))?;
    Ok(())
}

#[cfg(feature = "schema")]
#[test]
fn test_minimal_olb_report_against_serialize() -> anyhow::Result<()> {
    let s: VorgangTransportieren2010 = serde_json::from_str(&minimal_olb_report_json())?;
    let xml = raxb::ser::to_string_pretty_with_decl(&s)?;
    let validation = xoev_xwasser::schemas::XmlValidation::new()?;
    let result = validation.validate(xml.as_bytes());
    if let Err(e) = result {
        eprintln!("{e}");
    }
    dbg!(&xml);
    std::fs::write("tests/olb_report_minimal_test_result.xml", xml.replace("https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_8_0/ xwasser.xsd", "https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_8_0/ ../schemas/V0_8_0/xwasser.xsd"))?;
    Ok(())
}
