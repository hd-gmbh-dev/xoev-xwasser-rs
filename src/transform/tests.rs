use super::*;

fn sample_xml() -> String {
    include_str!("fixtures/sample.xml").to_string()
}

fn sample_xml_with_zi() -> String {
    let base = load_quality_report();
    transform_vorgang_transportieren_2010(
        base,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    )
}

fn sample_xml_no_leser_no_autor() -> String {
    let base = load_quality_report();
    let mut lines: Vec<&str> = base.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        if lines[i].contains("<leser>") || lines[i].contains("<autor>") {
            let tag = if lines[i].contains("<leser>") {
                "</leser>"
            } else {
                "</autor>"
            };
            lines.remove(i);
            while i < lines.len() && !lines[i].contains(tag) {
                lines.remove(i);
            }
            if i < lines.len() {
                lines.remove(i);
            }
        } else {
            i += 1;
        }
    }
    lines.join("\n")
}

fn sample_xml_custom_prefix() -> String {
    let base = sample_xml_with_zi();
    let result = base
        .replace("xmlns:xwas=", "xmlns:xw=")
        .replace("xwas:", "xw:")
        .replace("xmlns:xw:Signature", "xmlns:ds:Signature")
        .replace("xw:Signature", "ds:Signature");
    let parsed = raxb::de::from_str::<crate::model::transport::VorgangTransportieren2010>(&result);
    assert!(
        parsed.is_ok(),
        "custom prefix fixture must be raxb-parseable"
    );
    result
}

fn load_quality_report() -> &'static str {
    include_str!("../../tests/quality_report_minimal.xml")
}

fn extract_zusatzinfo_block(xml: &str) -> String {
    // Find the start of the line containing <xwas:zusatzinformationen>
    let tag = "<xwas:zusatzinformationen>";
    let tag_pos = xml.find(tag);
    let end = xml.find("</xwas:zusatzinformationen>");
    if let (Some(tp), Some(e)) = (tag_pos, end) {
        // Find the start of the line (leading whitespace)
        let line_start = xml[..tp].rfind('\n').map(|p| p + 1).unwrap_or(0);
        xml[line_start..e + "</xwas:zusatzinformationen>".len()].to_string()
    } else {
        "<not found>".to_string()
    }
}

fn assert_raxb_roundtrip(xml: &str) -> crate::model::transport::VorgangTransportieren2010 {
    match raxb::de::from_str(xml) {
        Ok(p) => p,
        Err(e) => panic!("raxb round-trip failed: {e:?}"),
    }
}

// ---- tests ----

#[test]
fn test_noop_is_byte_identical() {
    let xml = sample_xml();
    let result = transform_vorgang_transportieren_2010(&xml, &TransformOptions::default());
    assert_eq!(result, xml);
}

#[test]
fn test_leser_mutation() {
    let xml = load_quality_report();
    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:99999999".into()),
                    name: Some("NewReader".into()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    );
    let parsed = assert_raxb_roundtrip(&result);
    assert_eq!(parsed.nachrichtenkopf_g2g.leser.kennung, "psw:99999999");
    assert_eq!(parsed.nachrichtenkopf_g2g.leser.name, "NewReader");
    assert_eq!(parsed.nachrichtenkopf_g2g.autor.kennung, "psw:01003110");
    assert_eq!(parsed.nachrichtenkopf_g2g.autor.name, "Author");
}

#[test]
fn test_autor_mutation() {
    let xml = load_quality_report();
    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                autor: Some(ElementUpdate {
                    kennung: Some("psw:autor123".into()),
                    name: Some("Updated Autor".into()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    );
    let parsed = assert_raxb_roundtrip(&result);
    assert_eq!(parsed.nachrichtenkopf_g2g.leser.kennung, "psw:11113110");
    assert_eq!(parsed.nachrichtenkopf_g2g.leser.name, "Reader");
    assert_eq!(parsed.nachrichtenkopf_g2g.autor.kennung, "psw:autor123");
    assert_eq!(parsed.nachrichtenkopf_g2g.autor.name, "Updated Autor");
}

#[test]
fn test_leser_and_autor_mutation() {
    let xml = load_quality_report();
    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:leser1".into()),
                    name: Some("Leser1".into()),
                }),
                autor: Some(ElementUpdate {
                    kennung: Some("psw:autor1".into()),
                    name: Some("Autor1".into()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    );
    let parsed = assert_raxb_roundtrip(&result);
    assert_eq!(parsed.nachrichtenkopf_g2g.leser.kennung, "psw:leser1");
    assert_eq!(parsed.nachrichtenkopf_g2g.leser.name, "Leser1");
    assert_eq!(parsed.nachrichtenkopf_g2g.autor.kennung, "psw:autor1");
    assert_eq!(parsed.nachrichtenkopf_g2g.autor.name, "Autor1");
}

#[test]
fn test_zusatzinfo_full_replacement() {
    let base = load_quality_report();
    let with_zi = transform_vorgang_transportieren_2010(
        base,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    );
    assert!(with_zi.contains("xwas:zusatzinformationen"));

    let result = transform_vorgang_transportieren_2010(
        &with_zi,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    );
    let parsed = assert_raxb_roundtrip(&result);
    let zi = parsed
        .zusatzinformationen
        .as_ref()
        .expect("zusatzinfo must be present");
    assert_eq!(zi.zustaendige_behoerde_id, vec!["auth-001"]);
}

#[test]
fn test_zusatzinfo_replace_with_multiple_entries() {
    let base = load_quality_report();
    let with_zi = transform_vorgang_transportieren_2010(
        base,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["original".into()]),
            }),
            ..Default::default()
        },
    );
    // Replace with multiple entries
    let result = transform_vorgang_transportieren_2010(
        &with_zi,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["new-1".into(), "new-2".into()]),
            }),
            ..Default::default()
        },
    );
    let parsed = assert_raxb_roundtrip(&result);
    let zi = parsed
        .zusatzinformationen
        .as_ref()
        .expect("zusatzinfo must be present");
    assert_eq!(zi.zustaendige_behoerde_id, vec!["new-1", "new-2"]);
}

#[test]
fn test_zusatzinfo_replace_with_empty() {
    let base = load_quality_report();
    let with_zi = transform_vorgang_transportieren_2010(
        base,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth".into()]),
            }),
            ..Default::default()
        },
    );
    // Replace with empty content
    let result = transform_vorgang_transportieren_2010(
        &with_zi,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&[]),
            }),
            ..Default::default()
        },
    );
    let parsed = assert_raxb_roundtrip(&result);
    let zi = parsed
        .zusatzinformationen
        .as_ref()
        .expect("zusatzinfo element should remain");
    assert!(zi.zustaendige_behoerde_id.is_empty());
}

#[test]
fn test_zusatzinfo_preserves_kommentar_and_wasserversorgungsgebiet() {
    // Start from the quality report (valid XWasser document), insert
    // zusatzinformationen with kommentar, wasserversorgungsgebietID, and
    // a comment, then replace the IDs — should preserve everything else.
    let base = load_quality_report();
    let with_zi = transform_vorgang_transportieren_2010(
        base,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    );

    // Inject kommentar, wasserversorgungsgebietID, and a comment into the
    // zusatzinformationen block
    let with_extra = with_zi.replace(
        r#"<xwas:zustaendigeBehoerdeID>auth-001</xwas:zustaendigeBehoerdeID>
  </xwas:zusatzinformationen>"#,
        r#"<xwas:zustaendigeBehoerdeID>auth-001</xwas:zustaendigeBehoerdeID>
    <xwas:wasserversorgungsgebietID>wv-123</xwas:wasserversorgungsgebietID>
    <xwas:kommentar>some comment</xwas:kommentar>
    <!-- important comment -->
  </xwas:zusatzinformationen>"#,
    );

    // Verify the injection worked
    assert!(
        with_extra
            .contains("<xwas:wasserversorgungsgebietID>wv-123</xwas:wasserversorgungsgebietID>")
    );
    assert!(with_extra.contains("<xwas:kommentar>some comment</xwas:kommentar>"));
    assert!(with_extra.contains("<!-- important comment -->"));

    // Now replace the ID — should preserve kommentar, wasserversorgungsgebietID, and comment
    let result = transform_vorgang_transportieren_2010(
        &with_extra,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["new-id".into()]),
            }),
            ..Default::default()
        },
    );

    // Comments preserved
    assert!(result.contains("<!-- important comment -->"));
    // kommentar preserved
    assert!(result.contains("<xwas:kommentar>some comment</xwas:kommentar>"));
    // wasserversorgungsgebietID preserved
    assert!(
        result.contains("<xwas:wasserversorgungsgebietID>wv-123</xwas:wasserversorgungsgebietID>")
    );
    // Old ID replaced
    assert!(!result.contains("auth-001"));
    // New ID present with correct prefix
    assert!(result.contains("<xwas:zustaendigeBehoerdeID>new-id</xwas:zustaendigeBehoerdeID>"));

    // raxb round-trip proves the output is valid XWasser
    let parsed = assert_raxb_roundtrip(&result);
    let zi = parsed
        .zusatzinformationen
        .as_ref()
        .expect("zusatzinfo must be present");
    assert_eq!(zi.zustaendige_behoerde_id, vec!["new-id"]);
    assert_eq!(zi.wasserversorgungsgebiet_id.as_deref(), Some("wv-123"));
    assert_eq!(zi.kommentar.as_deref(), Some("some comment"));
}

#[test]
fn test_insert_leser() {
    let xml = sample_xml_no_leser_no_autor();
    let result = transform_vorgang_transportieren_2010(
        &xml,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:inserted".into()),
                    name: Some("Inserted Reader".into()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    );
    let leser_pos = match result.find("psw:inserted") {
        Some(p) => p,
        None => panic!("psw:inserted not found"),
    };
    let dvdv_pos = match result.find("dvdvDienstkennung") {
        Some(p) => p,
        None => panic!("dvdv not found"),
    };
    assert!(leser_pos < dvdv_pos);
}

#[test]
fn test_insert_autor() {
    let xml = sample_xml_no_leser_no_autor();
    let result = transform_vorgang_transportieren_2010(
        &xml,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:l".into()),
                    name: Some("Leser".into()),
                }),
                autor: Some(ElementUpdate {
                    kennung: Some("psw:a".into()),
                    name: Some("Autor".into()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    );
    let leser_pos = match result.find("psw:l") {
        Some(p) => p,
        None => panic!("psw:l not found"),
    };
    let autor_pos = match result.find("psw:a") {
        Some(p) => p,
        None => panic!("psw:a not found"),
    };
    assert!(leser_pos < autor_pos);
}

#[test]
fn test_insert_zusatzinformationen() {
    let xml = load_quality_report();
    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["new-auth".into()]),
            }),
            ..Default::default()
        },
    );
    let parsed = assert_raxb_roundtrip(&result);
    let zi = parsed
        .zusatzinformationen
        .as_ref()
        .expect("zusatzinfo must be present");
    assert_eq!(zi.zustaendige_behoerde_id, vec!["new-auth"]);
}

#[test]
fn test_insert_zusatzinformationen_before_signature() {
    // When <zusatzinformationen> is missing but a <ds:Signature> is present,
    // the inserted block must land BEFORE the signature (XSD sequence is
    // vorgang, zusatzinformationen, ds:Signature), not after it at </root>.
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1" xmlns:ds="http://www.w3.org/2000/09/xmldsig#">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht><nachrichtenUUID>id</nachrichtenUUID></identifikation.nachricht>
    <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
  </nachrichtenkopf.g2g>
  <xwas:vorgang><xwas:identifikationVorgang><xwas:vorgangsID>id</xwas:vorgangsID></xwas:identifikationVorgang></xwas:vorgang>
  <ds:Signature>
    <ds:SignedInfo/>
  </ds:Signature>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["new-auth".into()]),
            }),
            ..Default::default()
        },
    );

    // zusatzinformationen must appear before ds:Signature
    let zi_pos = result
        .find("<xwas:zusatzinformationen>")
        .expect("zi inserted");
    let sig_pos = result.find("<ds:Signature>").expect("signature present");
    assert!(
        zi_pos < sig_pos,
        "zusatzinformationen must precede ds:Signature. got:\n{result}"
    );
    // And the inserted ID is present
    assert!(result.contains("<xwas:zustaendigeBehoerdeID>new-auth</xwas:zustaendigeBehoerdeID>"));
    // The signature block is preserved verbatim after the insert
    assert!(result.contains("<ds:SignedInfo/>"));
    assert!(result.contains("</ds:Signature>"));
}

#[test]
fn test_custom_prefix_raxb_roundtrip() {
    // sample_xml_custom_prefix uses "xw:" prefix; verify transform preserves
    // raxb parseability and field values
    let xml = sample_xml_custom_prefix();
    assert!(xml.contains("xw:"), "fixture must use custom xw: prefix");

    let result = transform_vorgang_transportieren_2010(
        &xml,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:custom".into()),
                    name: Some("Custom".into()),
                }),
                ..Default::default()
            }),
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
        },
    );

    // raxb must parse output directly (no prefix normalization)
    let parsed = assert_raxb_roundtrip(&result);
    assert_eq!(parsed.nachrichtenkopf_g2g.leser.kennung, "psw:custom");
    assert_eq!(parsed.nachrichtenkopf_g2g.leser.name, "Custom");
    let zi = parsed
        .zusatzinformationen
        .as_ref()
        .expect("zusatzinfo must be present");
    assert_eq!(zi.zustaendige_behoerde_id, vec!["auth-001"]);
    // Verify output uses xw: prefix throughout (not xwas:)
    assert!(
        result.contains("xw:zusatzinformationen"),
        "should use xw: prefix for zusatzinfo"
    );
    assert!(
        result.contains("xw:zustaendigeBehoerdeID"),
        "should use xw: prefix for zustaendigeBehoerdeID"
    );
    assert!(
        !result.contains("xwas:zusatzinformationen"),
        "output should NOT use xwas: prefix"
    );
    // Verify authority content values via XML text
    assert!(
        result.contains("xw:zustaendigeBehoerdeID>auth-001"),
        "authority ID value"
    );
}

#[test]
fn test_raxb_roundtrip_noop() {
    assert_raxb_roundtrip(load_quality_report());
}

#[test]
fn test_comment_preservation() {
    let result = transform_vorgang_transportieren_2010(&sample_xml(), &TransformOptions::default());
    assert!(result.contains("<!-- root comment -->"));
}

#[test]
fn test_whitespace_preservation() {
    let result = transform_vorgang_transportieren_2010(&sample_xml(), &TransformOptions::default());
    assert!(result.contains("  <nachrichtenkopf.g2g>"));
    assert!(result.contains("    <identifikation.nachricht>"));
}

#[test]
fn test_signature_roundtrip() {
    let result = transform_vorgang_transportieren_2010(&sample_xml(), &TransformOptions::default());
    assert!(result.contains("ds:Signature"));
    assert!(result.contains("ds:SignedInfo"));
    assert!(result.contains("ds:DigestValue"));
    assert!(result.contains("ds:SignatureValue"));
    assert!(result.contains("ds:X509Data"));
}

#[test]
fn test_raxb_roundtrip_quality_report() {
    let xml = load_quality_report();
    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:mutated".into()),
                    name: Some("Mutated Reader".into()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    );
    let parsed: crate::model::transport::VorgangTransportieren2010 =
        match raxb::de::from_str(&result) {
            Ok(p) => p,
            Err(e) => panic!("raxb round-trip failed: {e:?}"),
        };
    assert_eq!(parsed.nachrichtenkopf_g2g.leser.kennung, "psw:mutated");
    assert_eq!(parsed.nachrichtenkopf_g2g.leser.name, "Mutated Reader");
    assert_eq!(parsed.nachrichtenkopf_g2g.autor.kennung, "psw:01003110");
}

#[test]
fn test_insert_leser_preserves_indentation_unit() {
    // Test with 4-space indentation
    let xml_4space = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
    <nachrichtenkopf.g2g>
        <identifikation.nachricht>
            <nachrichtenUUID>id</nachrichtenUUID>
        </identifikation.nachricht>
        <dvdvDienstkennung>s</dvdvDienstkennung>
    </nachrichtenkopf.g2g>
    <xwas:vorgang>
        <xwas:identifikationVorgang>
            <xwas:vorgangsID>id</xwas:vorgangsID>
        </xwas:identifikationVorgang>
    </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml_4space,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:inserted".into()),
                    name: Some("Inserted Reader".into()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    // Verify the inserted leser uses 4-space indentation unit
    assert!(
        result.contains("        <leser>"),
        r#"inserted leser should use 8-space indent (2 levels of 4), got:
{}"#,
        result
    );
    assert!(
        result.contains("            <verzeichnisdienst"),
        "inserted verzeichnisdienst should use 12-space indent (3 levels)"
    );
    assert!(
        result.contains("                <code>"),
        "inserted code should use 16-space indent (4 levels)"
    );
    assert!(
        result.contains("            <kennung>psw:inserted</kennung>"),
        "inserted kennung should use 12-space indent"
    );
    assert!(
        result.contains("            <name>Inserted Reader</name>"),
        "inserted name should use 12-space indent"
    );

    // Also verify with 2-space indentation (the default)
    let xml_2space = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>id</nachrichtenUUID>
    </identifikation.nachricht>
    <dvdvDienstkennung>s</dvdvDienstkennung>
  </nachrichtenkopf.g2g>
  <xwas:vorgang>
    <xwas:identifikationVorgang>
      <xwas:vorgangsID>id</xwas:vorgangsID>
    </xwas:identifikationVorgang>
  </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#;

    let result_2 = transform_vorgang_transportieren_2010(
        xml_2space,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:inserted".into()),
                    name: Some("Inserted Reader".into()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    assert!(
        result_2.contains("  <leser>"),
        r#"inserted leser should use 2-space indent, got:
{}"#,
        result_2
    );
    assert!(
        result_2.contains("    <verzeichnisdienst"),
        "inserted verzeichnisdienst should use 4-space indent"
    );
    assert!(
        result_2.contains("      <code>"),
        "inserted code should use 6-space indent"
    );

    // Also verify with 8-space indentation
    let xml_8space = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
        <nachrichtenkopf.g2g>
            <identifikation.nachricht>
                <nachrichtenUUID>id</nachrichtenUUID>
            </identifikation.nachricht>
            <dvdvDienstkennung>s</dvdvDienstkennung>
        </nachrichtenkopf.g2g>
        <xwas:vorgang>
            <xwas:identifikationVorgang>
                <xwas:vorgangsID>id</xwas:vorgangsID>
            </xwas:identifikationVorgang>
        </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#;

    let result_8 = transform_vorgang_transportieren_2010(
        xml_8space,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:inserted".into()),
                    name: Some("Inserted Reader".into()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    assert!(
        result_8.contains("        <leser>"),
        r#"inserted leser should use 8-space indent, got:
{}"#,
        result_8
    );
    assert!(
        result_8.contains("            <verzeichnisdienst"),
        "inserted verzeichnisdienst should use 12-space indent"
    );
    assert!(
        result_8.contains("                <code>"),
        "inserted code should use 16-space indent"
    );
}

#[test]
fn test_insert_zusatzinformationen_preserves_indentation_unit() {
    // Test with 4-space indentation
    let xml_4space = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
    <nachrichtenkopf.g2g>
        <identifikation.nachricht>
            <nachrichtenUUID>id</nachrichtenUUID>
        </identifikation.nachricht>
        <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
        <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
    </nachrichtenkopf.g2g>
    <xwas:vorgang>
        <xwas:identifikationVorgang>
            <xwas:vorgangsID>id</xwas:vorgangsID>
        </xwas:identifikationVorgang>
    </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml_4space,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    );

    // Verify the inserted zusatzinformationen uses 4-space indentation unit
    assert!(
        result.contains("    <xwas:zusatzinformationen>"),
        r#"inserted zusatzinfo should use 4-space indent, got:
{}"#,
        result
    );
    assert!(
        result
            .contains("        <xwas:zustaendigeBehoerdeID>auth-001</xwas:zustaendigeBehoerdeID>"),
        "inserted authority ID should use 8-space indent"
    );
}

#[test]
fn test_insert_leser_single_line_xml() {
    // Single-line (compact) XML has no whitespace text nodes, so the
    // indent unit falls back to 2 spaces.
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?><xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1"><nachrichtenkopf.g2g><identifikation.nachricht><nachrichtenUUID>id</nachrichtenUUID></identifikation.nachricht><leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser><autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor><dvdvDienstkennung>s</dvdvDienstkennung></nachrichtenkopf.g2g><xwas:vorgang><xwas:identifikationVorgang><xwas:vorgangsID>id</xwas:vorgangsID></xwas:identifikationVorgang></xwas:vorgang></xwas:vorgang.transportieren.2010>"#;
    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:inserted".into()),
                    name: Some("Inserted Reader".into()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    );
    // Should still insert the leser element
    assert!(result.contains("<leser>"));
    assert!(result.contains("psw:inserted"));
    assert!(result.contains("Inserted Reader"));
    // Verify the output is valid XML (can be parsed by quick-xml)
    let mut rdr = raxb::quick_xml::NsReader::from_str(&result);
    rdr.config_mut().trim_text(false);
    let mut depth = 0;
    let mut buf = Vec::new();
    loop {
        match rdr.read_event_into(&mut buf) {
            Ok(raxb::quick_xml::events::Event::Start(_)) => depth += 1,
            Ok(raxb::quick_xml::events::Event::End(_)) => depth -= 1,
            Ok(raxb::quick_xml::events::Event::Eof) => break,
            Err(_) => panic!("output is not valid XML"),
            _ => {}
        }
        buf.clear();
    }
    assert_eq!(depth, 0, "XML tags should be balanced");
}

#[test]
fn test_no_duplicate_tags_in_zusatzinfo_replacement() {
    // Verify that replacing zusatzinformationen doesn't produce
    // duplicate tags (e.g. double <zusatzinformationen> or
    // duplicate <zustaendigeBehoerdeID> entries)
    let base = load_quality_report();
    let with_zi = transform_vorgang_transportieren_2010(
        base,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    );

    // Count opening tags
    let count_tag = |tag: &str| {
        let needle = format!("<{tag}");
        with_zi.matches(&needle).count()
    };

    // Should have exactly 1 <zusatzinformationen> open tag
    assert_eq!(
        count_tag("xwas:zusatzinformationen"),
        1,
        "should have exactly 1 zusatzinformationen open tag"
    );
    // Should have exactly 1 </zusatzinformationen> close tag
    assert_eq!(
        with_zi.matches("</xwas:zusatzinformationen>").count(),
        1,
        "should have exactly 1 zusatzinformationen close tag"
    );
    // Should have exactly 1 <zustaendigeBehoerdeID> (the new one)
    assert_eq!(
        count_tag("xwas:zustaendigeBehoerdeID"),
        1,
        "should have exactly 1 zustaendigeBehoerdeID open tag"
    );
    assert_eq!(
        with_zi.matches("</xwas:zustaendigeBehoerdeID>").count(),
        1,
        "should have exactly 1 zustaendigeBehoerdeID close tag"
    );
}

#[test]
fn test_no_duplicate_tags_in_leser_mutation() {
    // Verify that mutating leser doesn't produce duplicate tags
    let base = load_quality_report();
    let result = transform_vorgang_transportieren_2010(
        base,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:mutated".into()),
                    name: Some("Mutated Reader".into()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    // Count opening tags
    let count_tag = |tag: &str| {
        let needle = format!("<{tag}");
        result.matches(&needle).count()
    };

    // Should have exactly 1 <leser> open tag
    assert_eq!(
        count_tag("leser"),
        1,
        "should have exactly 1 leser open tag"
    );
    // Should have exactly 1 </leser> close tag
    assert_eq!(
        result.matches("</leser>").count(),
        1,
        "should have exactly 1 leser close tag"
    );
    // Should have exactly 1 <kennung> inside leser (quality report has 2: leser + autor)
    assert!(
        count_tag("kennung") >= 1,
        "should have at least 1 kennung open tag"
    );
    assert!(
        result.matches("</kennung>").count() >= 1,
        "should have at least 1 kennung close tag"
    );
    // Should have exactly 1 <name> inside leser (quality report has many <name> tags)
    assert!(
        count_tag("name") >= 1,
        "should have at least 1 name open tag"
    );
    assert!(
        result.matches("</name>").count() >= 1,
        "should have at least 1 name close tag"
    );
}

#[test]
fn test_no_duplicate_tags_in_leser_insertion() {
    // Verify that inserting leser doesn't produce duplicate tags
    let xml = sample_xml_no_leser_no_autor();
    let result = transform_vorgang_transportieren_2010(
        &xml,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:inserted".into()),
                    name: Some("Inserted Reader".into()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    // Count opening tags
    let count_tag = |tag: &str| {
        let needle = format!("<{tag}");
        result.matches(&needle).count()
    };

    // Should have exactly 1 <leser> open tag
    assert_eq!(
        count_tag("leser"),
        1,
        "should have exactly 1 leser open tag"
    );
    assert_eq!(
        result.matches("</leser>").count(),
        1,
        "should have exactly 1 leser close tag"
    );
    // Should have exactly 1 <verzeichnisdienst> inside inserted leser
    assert_eq!(
        count_tag("verzeichnisdienst"),
        1,
        "should have exactly 1 verzeichnisdienst open tag"
    );
    assert_eq!(
        result.matches("</verzeichnisdienst>").count(),
        1,
        "should have exactly 1 verzeichnisdienst close tag"
    );
}

#[test]
fn test_no_duplicate_tags_in_autor_mutation() {
    // Verify that mutating autor doesn't produce duplicate tags
    let base = load_quality_report();
    let result = transform_vorgang_transportieren_2010(
        base,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                autor: Some(ElementUpdate {
                    kennung: Some("psw:mutated_autor".into()),
                    name: Some("Mutated Autor".into()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    // Count opening tags
    let count_tag = |tag: &str| {
        let needle = format!("<{tag}");
        result.matches(&needle).count()
    };

    // Should have exactly 1 <autor> open tag
    assert_eq!(
        count_tag("autor"),
        1,
        "should have exactly 1 autor open tag"
    );
    assert_eq!(
        result.matches("</autor>").count(),
        1,
        "should have exactly 1 autor close tag"
    );
    // Should have at least 1 <kennung> (quality report has 2: leser + autor)
    assert!(
        count_tag("kennung") >= 1,
        "should have at least 1 kennung open tag"
    );
    assert!(
        result.matches("</kennung>").count() >= 1,
        "should have at least 1 kennung close tag"
    );
    // Should have at least 1 <name> (quality report has many <name> tags)
    assert!(
        count_tag("name") >= 1,
        "should have at least 1 name open tag"
    );
    assert!(
        result.matches("</name>").count() >= 1,
        "should have at least 1 name close tag"
    );
}

#[test]
fn test_no_duplicate_tags_in_autor_insertion() {
    // Verify that inserting autor doesn't produce duplicate tags
    let xml = sample_xml_no_leser_no_autor();
    let result = transform_vorgang_transportieren_2010(
        &xml,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:l".into()),
                    name: Some("Leser".into()),
                }),
                autor: Some(ElementUpdate {
                    kennung: Some("psw:a".into()),
                    name: Some("Autor".into()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    // Count opening tags
    let count_tag = |tag: &str| {
        let needle = format!("<{tag}");
        result.matches(&needle).count()
    };

    // Should have exactly 1 <autor> open tag
    assert_eq!(
        count_tag("autor"),
        1,
        "should have exactly 1 autor open tag"
    );
    assert_eq!(
        result.matches("</autor>").count(),
        1,
        "should have exactly 1 autor close tag"
    );
    // Should have exactly 2 <verzeichnisdienst> (one in leser, one in autor)
    assert_eq!(
        count_tag("verzeichnisdienst"),
        2,
        "should have exactly 2 verzeichnisdienst open tags (leser + autor)"
    );
    assert_eq!(
        result.matches("</verzeichnisdienst>").count(),
        2,
        "should have exactly 2 verzeichnisdienst close tags"
    );
}

#[test]
fn test_zusatzinfo_replacement_preserves_xml_structure() {
    // Verify that replacing zusatzinformationen produces well-formed XML
    // with correct nesting and no orphaned tags
    let base = load_quality_report();
    let with_zi = transform_vorgang_transportieren_2010(
        base,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    );

    // Verify XML is well-formed by parsing with quick-xml
    let mut rdr = raxb::quick_xml::NsReader::from_str(&with_zi);
    rdr.config_mut().trim_text(false);
    let mut depth = 0;
    let mut buf = Vec::new();
    loop {
        match rdr.read_event_into(&mut buf) {
            Ok(raxb::quick_xml::events::Event::Start(_)) => {
                depth += 1;
            }
            Ok(raxb::quick_xml::events::Event::End(e)) => {
                let lok = e.local_name().as_ref().to_vec();
                depth -= 1;
                // zusatzinformationen should close before the root element
                if lok == b"zusatzinformationen" {
                    assert!(depth > 0, "zusatzinformationen should not close the root");
                }
            }
            Ok(raxb::quick_xml::events::Event::Eof) => break,
            Err(_) => panic!("output is not well-formed XML"),
            _ => {}
        }
        buf.clear();
    }
    assert_eq!(depth, 0, "XML tags should be balanced");
}

#[test]
fn test_zusatzinfo_replacement_preserves_all_fields() {
    // Comprehensive test: verify that ALL fields in ZusatzinformationenType
    // are preserved when replacing zustaendigeBehoerdeID entries
    let base = load_quality_report();
    let with_zi = transform_vorgang_transportieren_2010(
        base,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    );

    // Inject all possible fields
    let with_all_fields = with_zi.replace(
        r#"<xwas:zustaendigeBehoerdeID>auth-001</xwas:zustaendigeBehoerdeID>
  </xwas:zusatzinformationen>"#,
        r#"<xwas:zustaendigeBehoerdeID>auth-001</xwas:zustaendigeBehoerdeID>
    <xwas:wasserversorgungsgebietID>wv-123</xwas:wasserversorgungsgebietID>
    <xwas:kommentar>test comment</xwas:kommentar>
    <!-- test comment -->
  </xwas:zusatzinformationen>"#,
    );

    // Replace the IDs
    let result = transform_vorgang_transportieren_2010(
        &with_all_fields,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["new-id".into()]),
            }),
            ..Default::default()
        },
    );

    // Verify all fields are preserved
    assert!(
        result.contains("<xwas:wasserversorgungsgebietID>wv-123</xwas:wasserversorgungsgebietID>")
    );
    assert!(result.contains("<xwas:kommentar>test comment</xwas:kommentar>"));
    assert!(result.contains("<!-- test comment -->"));
    assert!(result.contains("<xwas:zustaendigeBehoerdeID>new-id</xwas:zustaendigeBehoerdeID>"));
    assert!(!result.contains("auth-001"));

    // Verify raxb round-trip
    let parsed = assert_raxb_roundtrip(&result);
    let zi = parsed
        .zusatzinformationen
        .as_ref()
        .expect("zusatzinfo must be present");
    assert_eq!(zi.zustaendige_behoerde_id, vec!["new-id"]);
    assert_eq!(zi.wasserversorgungsgebiet_id.as_deref(), Some("wv-123"));
    assert_eq!(zi.kommentar.as_deref(), Some("test comment"));
}

#[test]
fn test_zusatzinfo_replacement_with_multiple_fields_and_ids() {
    // Test replacing with multiple IDs while preserving multiple fields
    let base = load_quality_report();
    let with_zi = transform_vorgang_transportieren_2010(
        base,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    );

    let with_all = with_zi.replace(
        r#"<xwas:zustaendigeBehoerdeID>auth-001</xwas:zustaendigeBehoerdeID>
  </xwas:zusatzinformationen>"#,
        r#"<xwas:zustaendigeBehoerdeID>auth-001</xwas:zustaendigeBehoerdeID>
    <xwas:wasserversorgungsgebietID>wv-123</xwas:wasserversorgungsgebietID>
    <xwas:kommentar>comment1</xwas:kommentar>
    <!-- c1 -->
  </xwas:zusatzinformationen>"#,
    );

    let result = transform_vorgang_transportieren_2010(
        &with_all,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["id1".into(), "id2".into(), "id3".into()]),
            }),
            ..Default::default()
        },
    );

    // Verify all IDs are present
    assert!(result.contains("<xwas:zustaendigeBehoerdeID>id1</xwas:zustaendigeBehoerdeID>"));
    assert!(result.contains("<xwas:zustaendigeBehoerdeID>id2</xwas:zustaendigeBehoerdeID>"));
    assert!(result.contains("<xwas:zustaendigeBehoerdeID>id3</xwas:zustaendigeBehoerdeID>"));
    // Verify fields are preserved
    assert!(
        result.contains("<xwas:wasserversorgungsgebietID>wv-123</xwas:wasserversorgungsgebietID>")
    );
    assert!(result.contains("<xwas:kommentar>comment1</xwas:kommentar>"));
    assert!(result.contains("<!-- c1 -->"));
    // Verify old ID is gone
    assert!(!result.contains("auth-001"));

    // Verify raxb round-trip
    let parsed = assert_raxb_roundtrip(&result);
    let zi = parsed
        .zusatzinformationen
        .as_ref()
        .expect("zusatzinfo must be present");
    assert_eq!(zi.zustaendige_behoerde_id, vec!["id1", "id2", "id3"]);
    assert_eq!(zi.wasserversorgungsgebiet_id.as_deref(), Some("wv-123"));
    assert_eq!(zi.kommentar.as_deref(), Some("comment1"));
}

#[test]
fn test_zusatzinfo_replacement_preserves_comments_at_all_positions() {
    // Test that comments before, between, and after zustaendigeBehoerdeID
    // elements are all preserved
    let base = load_quality_report();
    let with_zi = transform_vorgang_transportieren_2010(
        base,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    );

    let with_comments = with_zi.replace(
        r#"<xwas:zustaendigeBehoerdeID>auth-001</xwas:zustaendigeBehoerdeID>
  </xwas:zusatzinformationen>"#,
        r#"<!-- before -->
    <xwas:zustaendigeBehoerdeID>auth-001</xwas:zustaendigeBehoerdeID>
    <!-- between -->
    <xwas:wasserversorgungsgebietID>wv</xwas:wasserversorgungsgebietID>
    <!-- after -->
  </xwas:zusatzinformationen>"#,
    );

    let result = transform_vorgang_transportieren_2010(
        &with_comments,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["new-id".into()]),
            }),
            ..Default::default()
        },
    );

    assert!(result.contains("<!-- before -->"));
    assert!(result.contains("<!-- between -->"));
    assert!(result.contains("<!-- after -->"));
    assert!(result.contains("<xwas:zustaendigeBehoerdeID>new-id</xwas:zustaendigeBehoerdeID>"));
    assert!(result.contains("<xwas:wasserversorgungsgebietID>wv</xwas:wasserversorgungsgebietID>"));
}

#[test]
fn test_zusatzinfo_insertion_indentation_2space() {
    // Verify exact indentation when inserting zusatzinformationen
    // into a 2-space-indented XML document
    // Indent unit is 2, so: root child = 2, zusatzinfo children = 4
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>id</nachrichtenUUID>
    </identifikation.nachricht>
    <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
  </nachrichtenkopf.g2g>
  <xwas:vorgang>
    <xwas:identifikationVorgang>
      <xwas:vorgangsID>id</xwas:vorgangsID>
    </xwas:identifikationVorgang>
  </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    );

    // Assert the entire produced zusatzinformationen block
    let expected_block = r#"  <xwas:zusatzinformationen>
    <xwas:zustaendigeBehoerdeID>auth-001</xwas:zustaendigeBehoerdeID>
  </xwas:zusatzinformationen>"#;
    let actual_block = extract_zusatzinfo_block(&result);
    assert_eq!(
        actual_block, expected_block,
        "zusatzinfo block should match expected indentation"
    );
}

#[test]
fn test_zusatzinfo_insertion_indentation_4space() {
    // Verify exact indentation when inserting zusatzinformationen
    // into a 4-space-indented XML document
    // Indent unit is 4, so: root child = 4, zusatzinfo children = 8
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
    <nachrichtenkopf.g2g>
        <identifikation.nachricht>
            <nachrichtenUUID>id</nachrichtenUUID>
        </identifikation.nachricht>
        <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
        <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
    </nachrichtenkopf.g2g>
    <xwas:vorgang>
        <xwas:identifikationVorgang>
            <xwas:vorgangsID>id</xwas:vorgangsID>
        </xwas:identifikationVorgang>
    </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    );

    // Assert the entire produced zusatzinformationen block
    let expected_block = r#"    <xwas:zusatzinformationen>
        <xwas:zustaendigeBehoerdeID>auth-001</xwas:zustaendigeBehoerdeID>
    </xwas:zusatzinformationen>"#;
    let actual_block = extract_zusatzinfo_block(&result);
    assert_eq!(
        actual_block, expected_block,
        "zusatzinfo block should match expected indentation"
    );
}

#[test]
fn test_zusatzinfo_insertion_indentation_8space() {
    // Verify exact indentation when inserting zusatzinformationen
    // into an 8-space-indented XML document
    // Indent unit is 8, so: root child = 8, zusatzinfo children = 16
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
        <nachrichtenkopf.g2g>
            <identifikation.nachricht>
                <nachrichtenUUID>id</nachrichtenUUID>
            </identifikation.nachricht>
            <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
            <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
        </nachrichtenkopf.g2g>
        <xwas:vorgang>
            <xwas:identifikationVorgang>
                <xwas:vorgangsID>id</xwas:vorgangsID>
            </xwas:identifikationVorgang>
        </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    );

    // Assert the entire produced zusatzinformationen block
    let expected_block = r#"        <xwas:zusatzinformationen>
                <xwas:zustaendigeBehoerdeID>auth-001</xwas:zustaendigeBehoerdeID>
        </xwas:zusatzinformationen>"#;
    let actual_block = extract_zusatzinfo_block(&result);
    assert_eq!(
        actual_block, expected_block,
        "zusatzinfo block should match expected indentation"
    );
}

#[test]
fn test_zusatzinfo_replacement_indentation_2space() {
    // Verify exact indentation when REPLACING zusatzinformationen
    // in a 2-space-indented XML document
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>id</nachrichtenUUID>
    </identifikation.nachricht>
    <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
  </nachrichtenkopf.g2g>
  <xwas:vorgang>
    <xwas:identifikationVorgang>
      <xwas:vorgangsID>id</xwas:vorgangsID>
    </xwas:identifikationVorgang>
  </xwas:vorgang>
  <xwas:zusatzinformationen>
    <xwas:zustaendigeBehoerdeID>old-id</xwas:zustaendigeBehoerdeID>
    <xwas:wasserversorgungsgebietID>wv-123</xwas:wasserversorgungsgebietID>
    <xwas:kommentar>old comment</xwas:kommentar>
  </xwas:zusatzinformationen>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["new-id".into()]),
            }),
            ..Default::default()
        },
    );

    // Assert the entire produced zusatzinformationen block
    let expected_block = r#"  <xwas:zusatzinformationen>
    <xwas:zustaendigeBehoerdeID>new-id</xwas:zustaendigeBehoerdeID>
    <xwas:wasserversorgungsgebietID>wv-123</xwas:wasserversorgungsgebietID>
    <xwas:kommentar>old comment</xwas:kommentar>
  </xwas:zusatzinformationen>"#;
    let actual_block = extract_zusatzinfo_block(&result);
    assert_eq!(
        actual_block, expected_block,
        "zusatzinfo block should match expected indentation"
    );
    assert!(!result.contains("old-id"));
}

#[test]
fn test_header_preserved_with_zusatzinfo_insertion() {
    // Verify that the XML declaration (header) is preserved when
    // inserting zusatzinformationen
    let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>id</nachrichtenUUID>
    </identifikation.nachricht>
    <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
  </nachrichtenkopf.g2g>
  <xwas:vorgang>
    <xwas:identifikationVorgang>
      <xwas:vorgangsID>id</xwas:vorgangsID>
    </xwas:identifikationVorgang>
  </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    );

    // Header must be preserved exactly
    assert!(
        result.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>"),
        "header should be preserved"
    );
}

#[test]
fn test_header_preserved_with_zusatzinfo_replacement() {
    // Verify that the XML declaration (header) is preserved when
    // replacing zusatzinformationen
    let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>id</nachrichtenUUID>
    </identifikation.nachricht>
    <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
  </nachrichtenkopf.g2g>
  <xwas:vorgang>
    <xwas:identifikationVorgang>
      <xwas:vorgangsID>id</xwas:vorgangsID>
    </xwas:identifikationVorgang>
  </xwas:vorgang>
  <xwas:zusatzinformationen>
    <xwas:zustaendigeBehoerdeID>old-id</xwas:zustaendigeBehoerdeID>
    <xwas:wasserversorgungsgebietID>wv-123</xwas:wasserversorgungsgebietID>
    <xwas:kommentar>old comment</xwas:kommentar>
  </xwas:zusatzinformationen>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["new-id".into()]),
            }),
            ..Default::default()
        },
    );

    // Header must be preserved exactly
    assert!(
        result.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>"),
        "header should be preserved"
    );
}

#[test]
fn test_header_no_standalone_preserved() {
    // Verify that header without standalone is also preserved
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>id</nachrichtenUUID>
    </identifikation.nachricht>
    <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
  </nachrichtenkopf.g2g>
  <xwas:vorgang>
    <xwas:identifikationVorgang>
      <xwas:vorgangsID>id</xwas:vorgangsID>
    </xwas:identifikationVorgang>
  </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    );

    // Header must be preserved exactly
    assert!(
        result.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"),
        "header should be preserved"
    );
}

#[test]
fn test_zusatzinfo_replacement_indentation_4space() {
    // Verify exact indentation when REPLACING zusatzinformationen
    // in a 4-space-indented XML document
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
    <nachrichtenkopf.g2g>
        <identifikation.nachricht>
            <nachrichtenUUID>id</nachrichtenUUID>
        </identifikation.nachricht>
        <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
        <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
    </nachrichtenkopf.g2g>
    <xwas:vorgang>
        <xwas:identifikationVorgang>
            <xwas:vorgangsID>id</xwas:vorgangsID>
        </xwas:identifikationVorgang>
    </xwas:vorgang>
    <xwas:zusatzinformationen>
        <xwas:zustaendigeBehoerdeID>old-id</xwas:zustaendigeBehoerdeID>
        <xwas:wasserversorgungsgebietID>wv-123</xwas:wasserversorgungsgebietID>
        <xwas:kommentar>old comment</xwas:kommentar>
    </xwas:zusatzinformationen>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["new-id".into()]),
            }),
            ..Default::default()
        },
    );

    // Assert the entire produced zusatzinformationen block
    let expected_block = r#"    <xwas:zusatzinformationen>
        <xwas:zustaendigeBehoerdeID>new-id</xwas:zustaendigeBehoerdeID>
        <xwas:wasserversorgungsgebietID>wv-123</xwas:wasserversorgungsgebietID>
        <xwas:kommentar>old comment</xwas:kommentar>
    </xwas:zusatzinformationen>"#;
    let actual_block = extract_zusatzinfo_block(&result);
    assert_eq!(
        actual_block, expected_block,
        "zusatzinfo block should match expected indentation"
    );
    assert!(!result.contains("old-id"));
}

#[test]
fn test_zusatzinfo_replacement_indentation_8space() {
    // Verify exact indentation when REPLACING zusatzinformationen
    // in an 8-space-indented XML document
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
        <nachrichtenkopf.g2g>
            <identifikation.nachricht>
                <nachrichtenUUID>id</nachrichtenUUID>
            </identifikation.nachricht>
            <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
            <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
        </nachrichtenkopf.g2g>
        <xwas:vorgang>
            <xwas:identifikationVorgang>
                <xwas:vorgangsID>id</xwas:vorgangsID>
            </xwas:identifikationVorgang>
        </xwas:vorgang>
        <xwas:zusatzinformationen>
                <xwas:zustaendigeBehoerdeID>old-id</xwas:zustaendigeBehoerdeID>
                <xwas:wasserversorgungsgebietID>wv-123</xwas:wasserversorgungsgebietID>
                <xwas:kommentar>old comment</xwas:kommentar>
        </xwas:zusatzinformationen>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["new-id".into()]),
            }),
            ..Default::default()
        },
    );

    // Assert the entire produced zusatzinformationen block
    let expected_block = r#"        <xwas:zusatzinformationen>
                <xwas:zustaendigeBehoerdeID>new-id</xwas:zustaendigeBehoerdeID>
                <xwas:wasserversorgungsgebietID>wv-123</xwas:wasserversorgungsgebietID>
                <xwas:kommentar>old comment</xwas:kommentar>
        </xwas:zusatzinformationen>"#;
    let actual_block = extract_zusatzinfo_block(&result);
    assert_eq!(
        actual_block, expected_block,
        "zusatzinfo block should match expected indentation"
    );
    assert!(!result.contains("old-id"));
}

#[test]
fn test_nachrichtenkopf_preserved_with_zusatzinfo_insertion() {
    // Verify that nachrichtenkopf.g2g content is preserved when
    // inserting zusatzinformationen
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>uuid-1234</nachrichtenUUID>
      <nachrichtentyp listURI="urn:xoev-de:xwasser:codeliste:nachrichtentyp" listVersionID="1">
        <code>2010</code>
      </nachrichtentyp>
      <erstellungszeitpunkt>2024-05-28T09:00:00</erstellungszeitpunkt>
    </identifikation.nachricht>
    <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
    <dvdvDienstkennung>s</dvdvDienstkennung>
  </nachrichtenkopf.g2g>
  <xwas:vorgang>
    <xwas:identifikationVorgang>
      <xwas:vorgangsID>id</xwas:vorgangsID>
    </xwas:identifikationVorgang>
  </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    );

    // nachrichtenkopf.g2g content must be preserved
    assert!(result.contains("<nachrichtenUUID>uuid-1234</nachrichtenUUID>"));
    assert!(result.contains("<code>2010</code>"));
    assert!(result.contains("<erstellungszeitpunkt>2024-05-28T09:00:00</erstellungszeitpunkt>"));
    assert!(result.contains("<kennung>r</kennung>"));
    assert!(result.contains("<kennung>a</kennung>"));
    assert!(result.contains("<dvdvDienstkennung>s</dvdvDienstkennung>"));
}

#[test]
fn test_nachrichtenkopf_preserved_with_zusatzinfo_replacement() {
    // Verify that nachrichtenkopf.g2g content is preserved when
    // replacing zusatzinformationen
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>uuid-1234</nachrichtenUUID>
      <nachrichtentyp listURI="urn:xoev-de:xwasser:codeliste:nachrichtentyp" listVersionID="1">
        <code>2010</code>
      </nachrichtentyp>
      <erstellungszeitpunkt>2024-05-28T09:00:00</erstellungszeitpunkt>
    </identifikation.nachricht>
    <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
    <dvdvDienstkennung>s</dvdvDienstkennung>
  </nachrichtenkopf.g2g>
  <xwas:vorgang>
    <xwas:identifikationVorgang>
      <xwas:vorgangsID>id</xwas:vorgangsID>
    </xwas:identifikationVorgang>
  </xwas:vorgang>
  <xwas:zusatzinformationen>
    <xwas:zustaendigeBehoerdeID>old-id</xwas:zustaendigeBehoerdeID>
  </xwas:zusatzinformationen>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["new-id".into()]),
            }),
            ..Default::default()
        },
    );

    // nachrichtenkopf.g2g content must be preserved
    assert!(result.contains("<nachrichtenUUID>uuid-1234</nachrichtenUUID>"));
    assert!(result.contains("<code>2010</code>"));
    assert!(result.contains("<erstellungszeitpunkt>2024-05-28T09:00:00</erstellungszeitpunkt>"));
    assert!(result.contains("<kennung>r</kennung>"));
    assert!(result.contains("<kennung>a</kennung>"));
    assert!(result.contains("<dvdvDienstkennung>s</dvdvDienstkennung>"));
    assert!(!result.contains("old-id"));
}

#[test]
fn test_nachrichtenkopf_formatting_preserved_2space() {
    // Verify that nachrichtenkopf.g2g formatting (indentation) is
    // preserved exactly when inserting zusatzinformationen
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>id</nachrichtenUUID>
    </identifikation.nachricht>
    <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
    <dvdvDienstkennung>s</dvdvDienstkennung>
  </nachrichtenkopf.g2g>
  <xwas:vorgang>
    <xwas:identifikationVorgang>
      <xwas:vorgangsID>id</xwas:vorgangsID>
    </xwas:identifikationVorgang>
  </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    );

    // Assert the entire nachrichtenkopf.g2g block is preserved exactly
    let expected_nk = r#"  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>id</nachrichtenUUID>
    </identifikation.nachricht>
    <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
    <dvdvDienstkennung>s</dvdvDienstkennung>
  </nachrichtenkopf.g2g>"#;
    assert!(
        result.contains(expected_nk),
        "nachrichtenkopf.g2g should be preserved exactly"
    );
}

#[test]
fn test_nachrichtenkopf_formatting_preserved_4space() {
    // Verify that nachrichtenkopf.g2g formatting (indentation) is
    // preserved exactly when inserting zusatzinformationen
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
    <nachrichtenkopf.g2g>
        <identifikation.nachricht>
            <nachrichtenUUID>id</nachrichtenUUID>
        </identifikation.nachricht>
        <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
        <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
        <dvdvDienstkennung>s</dvdvDienstkennung>
    </nachrichtenkopf.g2g>
    <xwas:vorgang>
        <xwas:identifikationVorgang>
            <xwas:vorgangsID>id</xwas:vorgangsID>
        </xwas:identifikationVorgang>
    </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    );

    // Assert the entire nachrichtenkopf.g2g block is preserved exactly
    let expected_nk = r#"    <nachrichtenkopf.g2g>
        <identifikation.nachricht>
            <nachrichtenUUID>id</nachrichtenUUID>
        </identifikation.nachricht>
        <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
        <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
        <dvdvDienstkennung>s</dvdvDienstkennung>
    </nachrichtenkopf.g2g>"#;
    assert!(
        result.contains(expected_nk),
        "nachrichtenkopf.g2g should be preserved exactly"
    );
}

#[test]
fn test_nachrichtenkopf_formatting_preserved_8space() {
    // Verify that nachrichtenkopf.g2g formatting (indentation) is
    // preserved exactly when inserting zusatzinformationen
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
        <nachrichtenkopf.g2g>
            <identifikation.nachricht>
                <nachrichtenUUID>id</nachrichtenUUID>
            </identifikation.nachricht>
            <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
            <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
            <dvdvDienstkennung>s</dvdvDienstkennung>
        </nachrichtenkopf.g2g>
        <xwas:vorgang>
            <xwas:identifikationVorgang>
                <xwas:vorgangsID>id</xwas:vorgangsID>
            </xwas:identifikationVorgang>
        </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["auth-001".into()]),
            }),
            ..Default::default()
        },
    );

    // Assert the entire nachrichtenkopf.g2g block is preserved exactly
    let expected_nk = r#"        <nachrichtenkopf.g2g>
            <identifikation.nachricht>
                <nachrichtenUUID>id</nachrichtenUUID>
            </identifikation.nachricht>
            <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
            <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
            <dvdvDienstkennung>s</dvdvDienstkennung>
        </nachrichtenkopf.g2g>"#;
    assert!(
        result.contains(expected_nk),
        "nachrichtenkopf.g2g should be preserved exactly"
    );
}

#[test]
fn test_nachrichtenkopf_formatting_preserved_with_replacement_2space() {
    // Verify that nachrichtenkopf.g2g formatting is preserved exactly
    // when REPLACING zusatzinformationen
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>id</nachrichtenUUID>
    </identifikation.nachricht>
    <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
    <dvdvDienstkennung>s</dvdvDienstkennung>
  </nachrichtenkopf.g2g>
  <xwas:vorgang>
    <xwas:identifikationVorgang>
      <xwas:vorgangsID>id</xwas:vorgangsID>
    </xwas:identifikationVorgang>
  </xwas:vorgang>
  <xwas:zusatzinformationen>
    <xwas:zustaendigeBehoerdeID>old-id</xwas:zustaendigeBehoerdeID>
  </xwas:zusatzinformationen>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["new-id".into()]),
            }),
            ..Default::default()
        },
    );

    // Assert the entire nachrichtenkopf.g2g block is preserved exactly
    let expected_nk = r#"  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>id</nachrichtenUUID>
    </identifikation.nachricht>
    <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
    <dvdvDienstkennung>s</dvdvDienstkennung>
  </nachrichtenkopf.g2g>"#;
    assert!(
        result.contains(expected_nk),
        "nachrichtenkopf.g2g should be preserved exactly"
    );
    assert!(!result.contains("old-id"));
}

#[test]
fn test_nachrichtenkopf_formatting_preserved_with_replacement_4space() {
    // Verify that nachrichtenkopf.g2g formatting is preserved exactly
    // when REPLACING zusatzinformationen in 4-space XML
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
    <nachrichtenkopf.g2g>
        <identifikation.nachricht>
            <nachrichtenUUID>id</nachrichtenUUID>
        </identifikation.nachricht>
        <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
        <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
        <dvdvDienstkennung>s</dvdvDienstkennung>
    </nachrichtenkopf.g2g>
    <xwas:vorgang>
        <xwas:identifikationVorgang>
            <xwas:vorgangsID>id</xwas:vorgangsID>
        </xwas:identifikationVorgang>
    </xwas:vorgang>
    <xwas:zusatzinformationen>
        <xwas:zustaendigeBehoerdeID>old-id</xwas:zustaendigeBehoerdeID>
    </xwas:zusatzinformationen>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&["new-id".into()]),
            }),
            ..Default::default()
        },
    );

    // Assert the entire nachrichtenkopf.g2g block is preserved exactly
    let expected_nk = r#"    <nachrichtenkopf.g2g>
        <identifikation.nachricht>
            <nachrichtenUUID>id</nachrichtenUUID>
        </identifikation.nachricht>
        <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
        <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
        <dvdvDienstkennung>s</dvdvDienstkennung>
    </nachrichtenkopf.g2g>"#;
    assert!(
        result.contains(expected_nk),
        "nachrichtenkopf.g2g should be preserved exactly"
    );
    assert!(!result.contains("old-id"));
}

#[test]
fn test_nachrichten_uuid_replacement() {
    // Verify that nachrichtenUUID is replaced when option is provided
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>old-uuid</nachrichtenUUID>
    </identifikation.nachricht>
    <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
  </nachrichtenkopf.g2g>
  <xwas:vorgang>
    <xwas:identifikationVorgang>
      <xwas:vorgangsID>id</xwas:vorgangsID>
    </xwas:identifikationVorgang>
  </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                nachrichten_uuid: Some("new-uuid"),
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    assert!(result.contains("<nachrichtenUUID>new-uuid</nachrichtenUUID>"));
    assert!(!result.contains("old-uuid"));
}

#[test]
fn test_format_uuid_replacement_preserves_surrounding_whitespace() {
    // Replacing <nachrichtenUUID> text must preserve the surrounding
    // whitespace: the UUID stays on its own indented line (not glued to
    // <identifikation.nachricht>), no stray whitespace-only line appears
    // after </nachrichtenUUID>, and the following sibling stays put.
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>old-uuid</nachrichtenUUID>
      <nachrichtentyp listURI="urn:xoev-de:xwasser:codeliste:nachrichtentyp" listVersionID="1">
        <code>2010</code>
      </nachrichtentyp>
      <erstellungszeitpunkt>2026-07-23T19:59:58</erstellungszeitpunkt>
    </identifikation.nachricht>
    <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
  </nachrichtenkopf.g2g>
  <xwas:vorgang><xwas:identifikationVorgang><xwas:vorgangsID>id</xwas:vorgangsID></xwas:identifikationVorgang></xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                nachrichten_uuid: Some("new-uuid"),
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    // The replacement must keep the exact surrounding formatting: only the
    // UUID text changes, the indent and line structure are byte-identical.
    let expected = "    <identifikation.nachricht>\n      <nachrichtenUUID>new-uuid</nachrichtenUUID>\n      <nachrichtentyp";
    assert!(
        result.contains(expected),
        "UUID replacement broke surrounding formatting. got:\n{result}"
    );
    // No glued start tag ...
    assert!(!result.contains("<identifikation.nachricht><nachrichtenUUID>"));
    // ... and no stray whitespace-only line after </nachrichtenUUID>.
    assert!(!result.contains("</nachrichtenUUID>\n      \n      <nachrichtentyp"));
}

#[test]
fn test_nachrichten_uuid_insertion() {
    // Verify that nachrichtenUUID is inserted when missing
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
    </identifikation.nachricht>
    <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
  </nachrichtenkopf.g2g>
  <xwas:vorgang>
    <xwas:identifikationVorgang>
      <xwas:vorgangsID>id</xwas:vorgangsID>
    </xwas:identifikationVorgang>
  </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                nachrichten_uuid: Some("inserted-uuid"),
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    assert!(result.contains("<nachrichtenUUID>inserted-uuid</nachrichtenUUID>"));
}

#[test]
fn test_nachrichten_uuid_preserved_when_not_set() {
    // Verify that nachrichtenUUID is preserved when option is not set
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_1">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>preserved-uuid</nachrichtenUUID>
    </identifikation.nachricht>
    <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
  </nachrichtenkopf.g2g>
  <xwas:vorgang>
    <xwas:identifikationVorgang>
      <xwas:vorgangsID>id</xwas:vorgangsID>
    </xwas:identifikationVorgang>
  </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#;

    let result = transform_vorgang_transportieren_2010(
        xml,
        &TransformOptions {
            ..Default::default()
        },
    );

    assert!(result.contains("<nachrichtenUUID>preserved-uuid</nachrichtenUUID>"));
}

// ---------------------------------------------------------------------
// Byte-level formatting regression tests
//
// These pin down the exact whitespace/line-break behavior around the
// boundaries of inserted/replaced elements. Earlier the insertion paths
// produced blank lines and glued-together closing tags because the
// measured indents already carry a leading newline while the helpers
// also emitted `\n`, and because the preceding whitespace text node was
// written before the insertion ran. These tests guard against regressions
// of that class.
// ---------------------------------------------------------------------

fn base_doc() -> &'static str {
    include_str!("fixtures/base_doc.xml")
}

#[test]
fn test_format_uuid_insert_no_blank_line_or_glued_close() {
    // nachrichtenUUID inserted into an empty <identifikation.nachricht>
    // must land on its own correctly-indented line, with the closing tag
    // on its own line — no dangling blank line, no glued tag.
    let xml = base_doc().replace("      <nachrichtenUUID>old-uuid</nachrichtenUUID>\n", "");
    let result = transform_vorgang_transportieren_2010(
        &xml,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                nachrichten_uuid: Some("inserted-uuid"),
                ..Default::default()
            }),
            ..Default::default()
        },
    );
    let expected = r#"    <identifikation.nachricht>
      <nachrichtenUUID>inserted-uuid</nachrichtenUUID>
    </identifikation.nachricht>"#;
    assert!(
        result.contains(expected),
        "expected UUID insertion block not found. got:\n{result}"
    );
}

#[test]
fn test_format_leser_insert_not_glued_to_autor() {
    // When <leser> is missing but <autor> exists, the inserted </leser>
    // must not be glued to <autor>, and no blank lines may appear.
    let xml = base_doc().replace(
            "    <leser>\n      <verzeichnisdienst listVersionID=\"\">\n        <code></code>\n      </verzeichnisdienst>\n      <kennung>r</kennung>\n      <name>Reader</name>\n    </leser>\n",
            "",
        );
    let result = transform_vorgang_transportieren_2010(
        &xml,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:new".into()),
                    name: Some("Inserted".into()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    );
    let expected = r#"    </identifikation.nachricht>
    <leser>
      <verzeichnisdienst listVersionID="">
        <code></code>
      </verzeichnisdienst>
      <kennung>psw:new</kennung>
      <name>Inserted</name>
    </leser>
    <autor>"#;
    assert!(
        result.contains(expected),
        "expected leser insertion block not found. got:\n{result}"
    );
}

#[test]
fn test_format_zusatzinfo_insert_not_glued_to_root_close() {
    // Inserted <zusatzinformationen> (when missing) must not produce a
    // blank line before it nor a glued </zusatzinformationen></root>.
    let ids = vec!["auth-001".to_string()];
    let result = transform_vorgang_transportieren_2010(
        base_doc(),
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&ids),
            }),
            ..Default::default()
        },
    );
    let expected = r#"  </xwas:vorgang>
  <xwas:zusatzinformationen>
    <xwas:zustaendigeBehoerdeID>auth-001</xwas:zustaendigeBehoerdeID>
  </xwas:zusatzinformationen>
</xwas:vorgang.transportieren.2010>"#;
    assert!(
        result.contains(expected),
        "expected zusatzinfo insertion block not found. got:\n{result}"
    );
}

#[test]
fn test_format_zusatzinfo_replace_no_blank_line_before() {
    // Replacing zusatzinfo content must not add a blank line before
    // <zusatzinformationen> (the preceding whitespace is preserved once).
    let xml = base_doc().replace(
            "</xwas:vorgang>\n</xwas:vorgang.transportieren.2010>",
            "</xwas:vorgang>\n  <xwas:zusatzinformationen>\n    <xwas:zustaendigeBehoerdeID>old1</xwas:zustaendigeBehoerdeID>\n    <xwas:wasserversorgungsgebietID>wv1</xwas:wasserversorgungsgebietID>\n    <xwas:kommentar>keep me</xwas:kommentar>\n  </xwas:zusatzinformationen>\n</xwas:vorgang.transportieren.2010>",
        );
    let ids = vec!["new1".to_string(), "new2".to_string()];
    let result = transform_vorgang_transportieren_2010(
        &xml,
        &TransformOptions {
            zusatzinformationen: Some(ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(&ids),
            }),
            ..Default::default()
        },
    );
    let expected = r#"  </xwas:vorgang>
  <xwas:zusatzinformationen>
    <xwas:zustaendigeBehoerdeID>new1</xwas:zustaendigeBehoerdeID>
    <xwas:zustaendigeBehoerdeID>new2</xwas:zustaendigeBehoerdeID>
    <xwas:wasserversorgungsgebietID>wv1</xwas:wasserversorgungsgebietID>
    <xwas:kommentar>keep me</xwas:kommentar>
  </xwas:zusatzinformationen>
</xwas:vorgang.transportieren.2010>"#;
    assert!(
        result.contains(expected),
        "expected zusatzinfo replacement block not found. got:\n{result}"
    );
}

#[test]
fn test_format_crlf_preserved_on_leser_insert() {
    // Inserted elements must use the source document's line ending
    // (CRLF here), not hard-coded \n.
    let lf = base_doc().replace(
            "    <leser>\n      <verzeichnisdienst listVersionID=\"\">\n        <code></code>\n      </verzeichnisdienst>\n      <kennung>r</kennung>\n      <name>Reader</name>\n    </leser>\n",
            "",
        );
    let crlf = lf.replace('\n', "\r\n");
    let result = transform_vorgang_transportieren_2010(
        &crlf,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:new".into()),
                    name: Some("Inserted".into()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    );
    // The inserted <leser> block must use CRLF throughout.
    let expected = "    <leser>\r\n      <verzeichnisdienst listVersionID=\"\">\r\n        <code></code>\r\n      </verzeichnisdienst>\r\n      <kennung>psw:new</kennung>\r\n      <name>Inserted</name>\r\n    </leser>";
    assert!(
        result.contains(expected),
        "expected CRLF leser insertion block not found. got:\n{result}"
    );
    // And no bare LF (\n not preceded by \r) may appear inside the
    // inserted block — the document uses CRLF, so every newline must be \r\n.
    let block_start = result.find("    <leser>").unwrap();
    let block_end = result[block_start..].find("</leser>").unwrap() + "</leser>".len();
    let block = &result[block_start..block_start + block_end];
    let has_bare_lf = block
        .as_bytes()
        .iter()
        .enumerate()
        .any(|(i, &b)| b == b'\n' && (i == 0 || block.as_bytes()[i - 1] != b'\r'));
    assert!(
        !has_bare_lf,
        "inserted leser block must not contain bare LF in a CRLF document. block:\n{block}"
    );
}

#[test]
fn test_format_malformed_xml_returned_unchanged() {
    // A parse error must not silently produce a truncated document;
    // the original input is returned unchanged.
    let malformed = "<xwas:vorgang.transportieren.2010><leser><unclosed>";
    let result = transform_vorgang_transportieren_2010(malformed, &TransformOptions::default());
    assert_eq!(malformed, result);
}
