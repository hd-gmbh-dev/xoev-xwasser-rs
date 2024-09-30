use xoev_xwasser::model::administration::AdministrationQuittung0020;

fn administration_receipt() -> String {
    std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join("tests/administration_receipt.xml"),
    )
    .unwrap()
}

fn administration_receipt_json() -> String {
    std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join("tests/administration_receipt.json"),
    )
    .unwrap()
}

#[test_log::test]
fn test_administration_receipt_against_deserialize() -> anyhow::Result<()> {
    let s = administration_receipt();
    let e: AdministrationQuittung0020 = raxb::de::from_str(&s)?;
    let json = serde_json::to_string_pretty(&e).unwrap();
    std::fs::write("tests/administration_receipt.json", json)?;
    std::fs::write("tests/administration_receipt.debug.log", format!("{e:#?}"))?;
    Ok(())
}

#[cfg(feature = "schema")]
#[test]
fn test_minimal_olb_report_against_serialize() -> anyhow::Result<()> {
    let s: AdministrationQuittung0020 = serde_json::from_str(&administration_receipt_json())?;
    let xml = raxb::ser::to_string_pretty_with_decl(&s)?;
    let validation = xoev_xwasser::schemas::XmlValidation::new()?;
    let result = validation.validate(xml.as_bytes());
    if let Err(e) = result {
        eprintln!("{e}");
    }
    dbg!(&xml);
    std::fs::write("tests/administration_receipt_test_result.xml", xml.replace("https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_6_0/ xwasser.xsd", "https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_6_0/ ../schemas/V0_6_0/xwasser.xsd"))?;
    Ok(())
}
