use xoev_xwasser::model::administration::AdministrationQuittung0020;

fn administration_receipt() -> String {
    std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join("tests/administration_receipt.xml"),
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
