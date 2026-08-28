#[cfg(feature = "builder")]
#[test]
fn test_olb_report_signature_builder() -> anyhow::Result<()> {
    use xoev_xwasser::{
        LOCAL_SCHEMA, SCHEMA,
        builder::{
            transport::{NachrichtenTypEnum, nachrichtenkopf_g2g},
            vorgang::identifikation_vorgang,
        },
        model::{
            codes::{CodeDokumenttypType, CodeUebermittlungsartType}, shared::dokument::DokumentType,
            signature::Signature, transport::VorgangTransportieren2010, vorgang::JahresberichtType,
            vorgang::Vorgang,
        },
    };

    let identifikation_vorgang = identifikation_vorgang(None);
    let dokument = DokumentType::builder()
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
    let jahresbericht = JahresberichtType::builder()
        .jahresbericht_id("id12345".to_string())
        .titel("Jahresbericht 2025".to_string())
        .uebermittlungsart(CodeUebermittlungsartType::from("1010"))
        .dokumentreferenz(vec!["id12345".to_string()])
        .kommentar(None)
        .id("id12345".to_string())
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
                .anlage(Vec::from([dokument]))
                .bemerkung(None)
                .identifikation_vorgang(identifikation_vorgang)
                .vorgang_type(xoev_xwasser::model::vorgang::VorgangType::Jahresbericht(
                    jahresbericht,
                ))
                .build(),
        )
        .zusatzinformationen(Default::default())
        .signature(Some(Signature { exists: true }))
        .build();

    let json = serde_json::to_string_pretty(&e).unwrap();
    std::fs::write("tests/olb_report_signature_builder.json", json)?;
    let xml = raxb::ser::to_string_pretty_with_decl(&e)?;
    std::fs::write(
        "tests/olb_report_signature_builder_test_result.xml",
        xml.replace(SCHEMA, LOCAL_SCHEMA),
    )?;
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
