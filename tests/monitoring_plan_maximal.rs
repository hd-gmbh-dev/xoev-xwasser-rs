use xoev_xwasser::model::transport::VorgangTransportieren2010;

fn maximal_monitoring_plan() -> String {
    std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join("tests/monitoring_plan_maximal.xml"),
    )
    .unwrap()
}

#[cfg(feature = "schema")]
fn maximal_monitoring_plan_json() -> String {
    std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join("tests/monitoring_plan_maximal.json"),
    )
    .unwrap()
}

#[test_log::test]
fn test_maximal_monitoring_plan_against_deserialize() -> anyhow::Result<()> {
    let s = maximal_monitoring_plan();
    let e: VorgangTransportieren2010 = raxb::de::from_str(&s)?;
    let json = serde_json::to_string_pretty(&e).unwrap();
    std::fs::write("tests/monitoring_plan_maximal.json", json)?;
    std::fs::write("tests/monitoring_plan_maximal.debug.log", format!("{e:#?}"))?;
    Ok(())
}

#[cfg(feature = "schema")]
#[test]
fn test_maximal_monitoring_plan_against_serialize() -> anyhow::Result<()> {
    let s: VorgangTransportieren2010 = serde_json::from_str(&maximal_monitoring_plan_json())?;
    let xml = raxb::ser::to_string_pretty_with_decl(&s)?;
    let validation = xoev_xwasser::schemas::XmlValidation::new()?;
    let result = validation.validate(xml.as_bytes());
    if let Err(e) = result {
        eprintln!("{e}");
    }
    dbg!(&xml);
    std::fs::write("tests/monitoring_plan_maximal_test_result.xml", xml.replace("https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_8_0/ xwasser.xsd", "https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_8_0/ ../schemas/V0_8_0/xwasser.xsd"))?;
    Ok(())
}
