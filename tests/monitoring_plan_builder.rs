#[cfg(feature = "builder")]
#[test]
fn test_monitoring_plan_builder() -> anyhow::Result<()> {
    use xoev_xwasser::{
        builder::{
            shared::untersuchungsplan::untersuchungsplan_type,
            transport::{nachrichtenkopf_g2g, NachrichtenTypEnum},
            vorgang::identifikation_vorgang,
        },
        model::{transport::VorgangTransportieren2010, vorgang::Vorgang},
    };

    let identifikation_vorgang = identifikation_vorgang(None);
    let untersuchungsplan = untersuchungsplan_type();

    let e = VorgangTransportieren2010::builder()
        .produkt("XWasser Test".into())
        .produkthersteller("H&D GmbH".into())
        .produktversion("0.700.1".into())
        .test(Some(true))
        .nachrichtenkopf_g2g(nachrichtenkopf_g2g(
            NachrichtenTypEnum::VorgangTransportieren2010,
        ))
        .vorgang(
            Vorgang::builder()
                .anlage(Default::default())
                .bemerkung(None)
                .identifikation_vorgang(identifikation_vorgang)
                .vorgang_type(
                    xoev_xwasser::model::vorgang::VorgangType::Untersuchungsplan(untersuchungsplan),
                )
                .build(),
        )
        .build();

    let json = serde_json::to_string_pretty(&e).unwrap();
    std::fs::write("tests/monitoring_plan_builder.json", json)?;
    let xml = raxb::ser::to_string_pretty_with_decl(&e)?;
    std::fs::write("tests/monitoring_plan_builder_test_result.xml", xml.replace(r#"xsi:schemaLocation="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_7_0/ xwasser.xsd""#,
    r#"xsi:schemaLocation="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_7_0/ ../schemas/V0_7_0/xwasser.xsd""#))?;
    Ok(())
}

#[cfg(feature = "schema")]
fn monitoring_plan_builder_test_result() -> String {
    std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join("tests/monitoring_plan_builder_test_result.xml"),
    )
    .unwrap()
}

#[cfg(feature = "schema")]
#[test]
fn test_monitoring_plan_builder_test_result_against_schema() -> anyhow::Result<()> {
    let s = monitoring_plan_builder_test_result();
    let validation = xoev_xwasser::schemas::XmlValidation::new()?;
    validation.validate(s.as_bytes())?;
    Ok(())
}
