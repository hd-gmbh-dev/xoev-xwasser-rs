//! Guards the convention behind `listVersionID`:
//!
//! * XWasser code type 1/2 fixes the version in the XML schema → the model keeps
//!   a compile-time constant (`#[xoev_xwasser_code(urn, "1")]`);
//! * code type 3 leaves the version to the sender → the model has a runtime
//!   field whose default is the currently published version
//!   (`#[xoev_xwasser_code(urn, current = "5")]`).
//!
//! The check reads the schema and `codes.rs` instead of trusting a
//! hand-maintained list: a wrong entry silently produces documents that fail
//! schema validation (that is how `untersuchungsverfahren` ended up declaring
//! version 2 while the data was at 3).

use raxb::quick_xml::{Reader, events::Event};

const SCHEMA: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/schemas/V1_0_1/xwasser-baukasten.xsd"
);

const CODES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/model/codes.rs");

/// One `#[xoev_xwasser_code(…)]` declaration of the model.
struct ModelDeclaration {
    urn: String,
    fixed: Option<String>,
    current: Option<String>,
}

fn model_declarations() -> Vec<ModelDeclaration> {
    let source = std::fs::read_to_string(CODES).expect("codes.rs");
    let marker = "#[xoev_xwasser_code(";
    let mut declarations = Vec::new();
    let mut rest = source.as_str();
    while let Some(start) = rest.find(marker) {
        let after = &rest[start + marker.len()..];
        let Some(end) = after.find(")]") else {
            break;
        };
        let body = &after[..end];
        rest = &after[end + 2..];
        let Some(urn) = body.split('"').nth(1) else {
            continue;
        };
        let current = body
            .split("current = \"")
            .nth(1)
            .and_then(|rest| rest.split('"').next())
            .map(str::to_string);
        let fixed = if current.is_some() {
            None
        } else {
            body.split(',').skip(1).collect::<Vec<_>>().join(",").split('"').nth(1).map(str::to_string)
        };
        declarations.push(ModelDeclaration {
            urn: urn.to_string(),
            fixed,
            current,
        });
    }
    declarations
}

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

/// The model must mirror the schema: a version fixed by the schema is a
/// compile-time constant, a sender-chosen codelist has a runtime field whose
/// default is the currently published version.
#[test]
fn model_declares_the_versions_the_schema_expects() {
    let schema = codelists_from_schema();
    assert!(schema.len() > 60, "schema not parsed: {}", schema.len());
    let declarations = model_declarations();
    assert!(
        declarations.len() > 60,
        "codes.rs not parsed: {}",
        declarations.len()
    );

    let mut problems = Vec::new();
    for (urn, fixed) in &schema {
        let Some(declaration) = declarations.iter().find(|d| &d.urn == urn) else {
            problems.push(format!("{urn}: no declaration in codes.rs"));
            continue;
        };
        match fixed {
            Some(version) => {
                if declaration.fixed.as_deref() != Some(version.as_str()) {
                    problems.push(format!(
                        "{urn}: schema fixes version {version}, model has {:?}",
                        declaration.fixed
                    ));
                }
                if declaration.current.is_some() {
                    problems.push(format!(
                        "{urn}: schema fixes the version, the model must not default it"
                    ));
                }
            }
            None => {
                if declaration.fixed.is_some() {
                    problems.push(format!(
                        "{urn}: the sender chooses the version, the model must not fix it to {:?}",
                        declaration.fixed
                    ));
                }
                if declaration.current.is_none() {
                    problems.push(format!(
                        "{urn}: sender-chosen codelist without a current version"
                    ));
                }
            }
        }
    }

    assert!(problems.is_empty(), "{}", problems.join("\n"));
}
