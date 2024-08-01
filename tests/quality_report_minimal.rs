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
    eprintln!("{}", serde_json::to_string_pretty(&e.unwrap()).unwrap());
    Ok(())
}

#[cfg(feature = "schema")]
#[test]
fn test_minimal_quality_report_against_serialize() -> anyhow::Result<()> {
    let s: VorgangTransportieren2010 = serde_json::from_value(build_quality_report_minimal())?;
    let xml = raxb::ser::to_string_pretty_with_decl(&s)?;
    let validation = xoev_xwasser::schemas::XmlValidation::new()?;
    let result = validation.validate(xml.as_bytes());
    if let Err(e) = result {
        eprintln!("{e}");
        eprintln!("{xml}");
    }
    Ok(())
}

fn build_quality_report_minimal() -> serde_json::Value {
    serde_json::json!({
      "produkt": "SHAPTH CLI",
      "test": true,
      "nachrichtenkopf_g2g": {
        "identifikation_nachricht": {
          "nachrichten_uuid": "693c64d6-456f-4d14-abe7-fe9681c74aae",
          "nachrichten_typ": {
            "code": {
              "list_uri": null,
              "list_version_id": null,
              "code": "0010"
            }
          },
          "erstellungszeitpunkt": "2024-05-28T09:00:00"
        },
        "leser": {
          "verzeichnisdienst": {
            "code": {
              "list_uri": null,
              "list_version_id": null,
              "code": ""
            }
          },
          "kennung": "psw:11113110",
          "name": "Reader"
        },
        "autor": {
          "verzeichnisdienst": {
            "code": {
              "list_uri": null,
              "list_version_id": null,
              "code": ""
            }
          },
          "kennung": "psw:01003110",
          "name": "Author"
        },
        "referenz_uuid": "238b7cc7-6d64-4db8-9c69-779bb65d60b1",
        "dvdv_dienstkennung": "s"
      }
    })
}
