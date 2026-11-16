//! The XSD pins the version of these codelists (`listVersionID` is `fixed=`) but
//! keeps `code` as `xs:token`, so nothing in the schema rejects an unknown
//! value. Their declarations must therefore carry `validate`, otherwise the
//! model accepts anything — the server then has no chance to reject it.
use std::fs;

/// Codelists the baukasten does not enumerate (measured on 0.9.2 … 1.0.1: the
/// same eleven in every line).
const SCHEMA_UNVALIDATED: &[&str] = &[
    "urn:de:bund:destatis:bevoelkerungsstatistik:schluessel:bundesland",
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
}
