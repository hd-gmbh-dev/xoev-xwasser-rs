//! Guards the convention behind `listVersionID`:
//!
//! * XWasser code type 1/2 fixes the version in the XML schema → the model keeps
//!   a constant and the version never comes from `CURRENT_VERSIONS`;
//! * code type 3 leaves the version to the sender → the model has a runtime
//!   field and `CURRENT_VERSIONS` must know the codelist.
//!
//! The check compares the model's table with the schema instead of trusting a
//! hand-maintained list (a wrong entry here silently produces documents that
//! fail schema validation).

use std::collections::BTreeSet;

use raxb::quick_xml::{Reader, events::Event};
use xoev_xwasser::model::codelist_versions::CURRENT_VERSIONS;

const SCHEMA: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/schemas/V1_0_1/xwasser-baukasten.xsd"
);

/// `Code.*Type` with a `listVersionID` attribute → (codelist urn, fixed version)
fn codelists_from_schema() -> Vec<(String, Option<String>)> {
    let source = std::fs::read_to_string(SCHEMA).expect("schema");
    let mut reader = Reader::from_str(&source);
    let mut found = Vec::new();
    let mut current: Option<String> = None;
    let mut urn: Option<String> = None;
    let mut fixed: Option<String> = None;
    let mut in_kennung = false;
    let mut has_version_attribute = false;

    loop {
        match reader.read_event().expect("xml") {
            Event::Start(start) => {
                let name = String::from_utf8_lossy(start.name().as_ref()).to_string();
                match name.as_str() {
                    "xs:complexType" => {
                        let type_name = start
                            .attributes()
                            .flatten()
                            .find(|attr| attr.key.as_ref() == b"name")
                            .map(|attr| String::from_utf8_lossy(&attr.value).to_string());
                        current = type_name.filter(|name| name.starts_with("Code."));
                        urn = None;
                        fixed = None;
                        has_version_attribute = false;
                    }
                    "xs:attribute" => {
                        read_version_attribute(
                            start.attributes(),
                            &mut has_version_attribute,
                            &mut fixed,
                        );
                    }
                    "kennung" => in_kennung = true,
                    _ => {}
                }
            }
            Event::Empty(empty) => {
                let name = String::from_utf8_lossy(empty.name().as_ref()).to_string();
                if name == "xs:attribute" {
                    read_version_attribute(
                        empty.attributes(),
                        &mut has_version_attribute,
                        &mut fixed,
                    );
                }
            }
            Event::Text(text) if in_kennung => {
                urn = Some(String::from_utf8_lossy(text.as_ref()).to_string());
            }
            Event::End(end) => {
                let name = String::from_utf8_lossy(end.name().as_ref()).to_string();
                match name.as_str() {
                    "kennung" => in_kennung = false,
                    "xs:complexType" => {
                        if let Some(urn) = urn.take()
                            && has_version_attribute
                        {
                            found.push((urn, fixed.take()));
                        }
                        current = None;
                    }
                    _ => {}
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }
    let _ = current;
    found
}

/// `xs:attribute` is self-closing, so it arrives as `Event::Empty`.
fn read_version_attribute<'a>(
    attributes: raxb::quick_xml::events::attributes::Attributes<'a>,
    has_version_attribute: &mut bool,
    fixed: &mut Option<String>,
) {
    let mut is_version = false;
    let mut fixed_value = None;
    for attr in attributes.flatten() {
        match attr.key.as_ref() {
            b"name" if attr.value.as_ref() == b"listVersionID" => is_version = true,
            b"fixed" => {
                fixed_value = Some(String::from_utf8_lossy(&attr.value).to_string());
            }
            _ => {}
        }
    }
    if is_version {
        *has_version_attribute = true;
        *fixed = fixed_value;
    }
}

#[test]
fn runtime_version_table_matches_the_schema() {
    let codelists = codelists_from_schema();
    assert!(
        codelists.len() > 60,
        "schema not parsed: {}",
        codelists.len()
    );

    let sender: BTreeSet<String> = codelists
        .iter()
        .filter(|(_, fixed)| fixed.is_none())
        .map(|(urn, _)| urn.clone())
        .collect();
    let fixed: BTreeSet<String> = codelists
        .iter()
        .filter(|(_, fixed)| fixed.is_some())
        .map(|(urn, _)| urn.clone())
        .collect();

    let table: BTreeSet<String> = CURRENT_VERSIONS
        .iter()
        .map(|(urn, _)| urn.to_string())
        .collect();

    assert_eq!(
        table, sender,
        "CURRENT_VERSIONS must list exactly the sender-chosen codelists"
    );
    for urn in &fixed {
        assert!(
            !table.contains(urn),
            "{urn} has a version fixed by the schema and must not be in CURRENT_VERSIONS"
        );
    }
}
