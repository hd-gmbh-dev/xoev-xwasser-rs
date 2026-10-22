use xoev_xwasser::model::transport::VorgangTransportieren2010;

fn fixture() -> String {
    std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join("tests/quality_report_minimal.xml"),
    )
    .unwrap()
}

/// What happens to `listVersionID` of a Code-Typ 3 field when a document is
/// deserialized and serialized again?
#[test]
fn list_version_id_of_type3_code_survives_a_round_trip() -> anyhow::Result<()> {
    let mut xml = fixture();
    // pretend the sender used older codelist versions
    let before = xml.matches("listVersionID").count();
    xml = xml.replace(
        r#"listURI="urn:xoev-de:xwasser:codeliste:untersuchungsverfahren" listVersionID="2""#,
        r#"listURI="urn:xoev-de:xwasser:codeliste:untersuchungsverfahren" listVersionID="1""#,
    );

    let parsed: VorgangTransportieren2010 = raxb::de::from_str(&xml)?;
    let json = serde_json::to_string(&parsed).unwrap();
    let reserialized = raxb::ser::to_string(&parsed)?;

    let incoming_versions: Vec<&str> = xml
        .split("listVersionID=\"")
        .skip(1)
        .map(|rest| rest.split('"').next().unwrap())
        .collect();
    let outgoing_versions: Vec<&str> = reserialized
        .split("listVersionID=\"")
        .skip(1)
        .map(|rest| rest.split('"').next().unwrap())
        .collect();

    println!("attribute count: {before}");
    println!("incoming  : {incoming_versions:?}");
    println!("outgoing  : {outgoing_versions:?}");
    println!("json has listVersionID: {}", json.contains("listVersionID"));
    assert_eq!(
        incoming_versions, outgoing_versions,
        "incoming codelist versions must be preserved"
    );
    Ok(())
}
