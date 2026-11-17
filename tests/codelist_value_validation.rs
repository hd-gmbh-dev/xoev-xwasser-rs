//! The XSD pins the version of these codelists (`listVersionID` is `fixed=`) but
//! keeps `code` as `xs:token`, so nothing in the schema rejects an unknown
//! value. Their declarations must therefore carry `validate`, otherwise the
//! model accepts anything — the server then has no chance to reject it.
use std::fs;

/// Codelists the baukasten does not enumerate (measured on 0.9.2 … 1.0.1: the
/// same eleven in every line).
/// `bundesland` is deliberately absent: the published list keys the states as
/// `01`..`16`, real documents send `DEBY`/`DEBW`, and enforcing it would reject
/// data that is accepted today. Revisit with the data owners.
const NOT_VALIDATED_ON_PURPOSE: &[&str] =
    &["urn:de:bund:destatis:bevoelkerungsstatistik:schluessel:bundesland"];

const SCHEMA_UNVALIDATED: &[&str] = &[
    "urn:de:dsmeld:schluesseltabelle:auskunftssperre",
    "urn:de:dsmeld:schluesseltabelle:familienstand",
    "urn:de:dsmeld:schluesseltabelle:familienstand.beendigungsgrund",
    "urn:xpersonenstand:schluesseltabelle:namensart",
    "urn:de:xoev:codeliste:erreichbarkeit",
    "urn:xoev-de:fim:codeliste:xzufi.anschrifttyp",
    "urn:xoev-de:kosit:codeliste:currency-codes",
    "urn:xoev-de:xewaffe:codeliste:statusfachlich",
    "urn:xoev-de:xewaffe:codeliste:statustechnisch",
    "urn:xoev-de:xfahrtenschreiber:codeliste:sprache-eu",
];

/// Attribute bodies of all `#[xoev_xwasser_code(..)]` declarations.
fn declarations() -> Vec<String> {
    let source = fs::read_to_string("src/model/codes.rs").expect("read codes.rs");
    let mut out = Vec::new();
    let mut rest = source.as_str();
    while let Some(start) = rest.find("#[xoev_xwasser_code(") {
        rest = &rest[start..];
        let end = rest.find(")]").expect("attribute closes");
        out.push(rest[..end].to_string());
        rest = &rest[end..];
    }
    out
}

#[test]
fn the_lists_the_schema_cannot_check_are_validated_by_the_model() {
    let declarations = declarations();
    let missing: Vec<&str> = SCHEMA_UNVALIDATED
        .iter()
        .copied()
        .filter(|urn| {
            let declaration = declarations
                .iter()
                .find(|declaration| declaration.contains(&format!("\"{urn}\"")))
                .unwrap_or_else(|| panic!("no declaration for {urn}"));
            !declaration.contains("validate")
        })
        .collect();

    assert!(
        missing.is_empty(),
        "these codelists are not enumerated by the XSD and must declare `validate`: {missing:?}"
    );

    // the exception stays an exception: the day the data matches the documents,
    // the list above should lose this flag and the URN must move back
    for urn in NOT_VALIDATED_ON_PURPOSE {
        let declaration = declarations
            .iter()
            .find(|declaration| declaration.contains(&format!("\"{urn}\"")))
            .unwrap_or_else(|| panic!("no declaration for {urn}"));
        assert!(
            !declaration.contains("validate"),
            "{urn} is validated on purpose, remove the exception"
        );
    }
}

/// The opaque error must name the version the message claimed: a value can be
/// valid in one version and unknown in another, and the receiver's message has
/// to tell the sender which one is wrong.
#[test]
fn a_rejected_value_carries_the_transmitted_version() {
    use xoev_xwasser::model::transport::VorgangTransportieren2010;
    use xoev_xwasser::{CodeListsProvider, XWasserValidate, XWasserValidateError};

    struct Nothing;

    impl CodeListsProvider for Nothing {
        fn contains(&self, _: &str, _: &str, _: &str) -> bool {
            false
        }
    }

    let xml = std::fs::read_to_string(
        std::env::current_dir()
            .expect("cwd")
            .join("tests/quality_report_minimal.xml"),
    )
    .expect("fixture");
    let parsed: VorgangTransportieren2010 = raxb::de::from_str(&xml).expect("parses");

    let err = parsed
        .xwasser_validate(&Nothing)
        .expect_err("nothing is valid against an empty provider");
    let XWasserValidateError::CodeListValueNotFound { version, .. } = err;
    assert!(
        !version.is_empty(),
        "the error must carry the version, got {version:?}"
    );
}
