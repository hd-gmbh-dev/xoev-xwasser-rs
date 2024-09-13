use xoev_xwasser::model::transport::VorgangTransportieren2010;



fn minimal_monitoring_plan_report() -> String {
    std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join("tests/monitoring_plan_minimal.xml"),
    )
    .unwrap()
}


#[test_log::test]
fn test_minimal_monitoring_plan_against_deserialize() -> anyhow::Result<()> {
    let s = minimal_monitoring_plan_report();
    let e: VorgangTransportieren2010 = raxb::de::from_str(&s)?;
    let json = serde_json::to_string_pretty(&e).unwrap();
    std::fs::write("tests/monitoring_plan_minimal.json", json)?;
    std::fs::write("tests/monitoring_plan_minimal.debug.log", format!("{e:#?}"))?;
    Ok(())
}

