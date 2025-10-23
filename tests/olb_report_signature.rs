#[cfg(feature = "builder")]
#[test]
fn test_olb_report_signature_builder() -> anyhow::Result<()> {
    use xoev_xwasser::{
        builder::{
            transport::{nachrichtenkopf_g2g, NachrichtenTypEnum},
            vorgang::identifikation_vorgang,
        },
        model::{
            codes::CodeDokumenttypType, shared::dokument::DokumentType, signature::Signature,
            transport::VorgangTransportieren2010, vorgang::Vorgang,
        },
    };

    let identifikation_vorgang = identifikation_vorgang(None);
    let olb_bericht = DokumentType::builder()
        .dokument_typ(
            CodeDokumenttypType::builder()
                .code("1010".to_string())
                .name(None)
                .build(),
        )
        .aktuelle_version(None)
        .dokument_id("id12345".to_string())
        .dokument_repraesentation(vec![])
        .letzte_version(None)
        .name("name".to_string())
        .person_referenz_id(vec![])
        .build();

    let e = VorgangTransportieren2010::builder()
        .produkt("XWasser Test".into())
        .produkthersteller("H&D GmbH".into())
        .produktversion("0.800.0".into())
        .test(Some(true))
        .nachrichtenkopf_g2g(nachrichtenkopf_g2g(
            NachrichtenTypEnum::VorgangTransportieren2010,
        ))
        .vorgang(
            Vorgang::builder()
                .anlage(Default::default())
                .bemerkung(None)
                .identifikation_vorgang(identifikation_vorgang)
                .vorgang_type(xoev_xwasser::model::vorgang::VorgangType::OlbBericht(
                    olb_bericht,
                ))
                .build(),
        )
        .signature(Some(Signature { exists: true }))
        .build();

    let json = serde_json::to_string_pretty(&e).unwrap();
    std::fs::write("tests/olb_report_signature_builder.json", json)?;
    let xml = raxb::ser::to_string_pretty_with_decl(&e)?;
    std::fs::write("tests/olb_report_signature_builder_test_result.xml", xml.replace(r#"xsi:schemaLocation="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_9_5/ xwasser.xsd""#,
    r#"xsi:schemaLocation="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_9_5/ ../schemas/V0_9_5/xwasser.xsd""#))?;
    Ok(())
}

#[cfg(feature = "schema")]
fn olb_report_signature_builder_test_result() -> String {
    std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join("tests/olb_report_signature_builder_test_result.xml"),
    )
    .unwrap()
}

#[cfg(feature = "schema")]
#[test]
fn test_olb_report_signature_builder_test_result_against_schema() -> anyhow::Result<()> {
    let s = olb_report_signature_builder_test_result();
    let validation = xoev_xwasser::schemas::XmlValidation::new()?;
    validation.validate(s.as_bytes())?;
    Ok(())
}
