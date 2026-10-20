//! Current version of every codelist referenced by a **Code-Typ 3** field.
//!
//! Code type 3 leaves the codelist version to the sender
//! (`listVersionID`), so a document created by this library has to pick one: the
//! version that is currently published. That information is not part of the
//! schema, so it is generated from the codelist data
//! (`hd-gmbh-dev/xoev-codelists-rs`) and kept here as a plain table — 27 entries,
//! no codelist rows, so the wasm build stays data free.
//!
//! Regenerate with the sync tool of the codelists repository:
//! `cargo run -p xtask -- export --npm npm` (the `index.json` `current` values).
//!
//! Types whose version is fixed by the standard (code type 1/2) do not use this
//! table: their version is a compile-time constant taken from the schema.

/// Current published version per sender-chosen codelist.
///
/// Sorted by URN; the order is irrelevant for lookup but keeps diffs readable.
pub const CURRENT_VERSIONS: &[(&str, &str)] = &[
    (
        "urn:de:bund:destatis:bevoelkerungsstatistik:schluessel:ags",
        "2026-09-30",
    ),
    (
        "urn:de:bund:destatis:bevoelkerungsstatistik:schluessel:bezirk",
        "2019-07-31",
    ),
    (
        "urn:de:bund:destatis:bevoelkerungsstatistik:schluessel:kreis",
        "2026-01-31",
    ),
    (
        "urn:de:bund:destatis:bevoelkerungsstatistik:schluessel:rs",
        "2026-09-30",
    ),
    (
        "urn:de:bund:destatis:bevoelkerungsstatistik:schluessel:staat",
        "2026-08-05",
    ),
    (
        "urn:de:bund:destatis:bevoelkerungsstatistik:schluessel:staatsangehoerigkeit",
        "2026-08-05",
    ),
    (
        "urn:de:dsmeld:schluesseltabelle:pass.und.ausweisdokumente",
        "5",
    ),
    ("urn:xoev-de:bund:bmi:bit:codeliste:dvdv.praefix", "51"),
    ("urn:xoev-de:xauslaender:codeliste:sprachenkatalog", "11"),
    (
        "urn:xoev-de:xunternehmen:codeliste:artgesetzlichervertreter",
        "1",
    ),
    ("urn:xoev-de:xunternehmen:codeliste:rechtsformen", "2.4"),
    ("urn:xoev-de:xwasser:codeliste:art-entnahmearmatur", "1"),
    ("urn:xoev-de:xwasser:codeliste:art-objekt", "3"),
    ("urn:xoev-de:xwasser:codeliste:art-probennahmestelle", "2"),
    (
        "urn:xoev-de:xwasser:codeliste:aufbereitungsstoffe-desinfektionsverfahren",
        "3",
    ),
    ("urn:xoev-de:xwasser:codeliste:desinfektionsart", "2"),
    ("urn:xoev-de:xwasser:codeliste:massnahmen", "2"),
    ("urn:xoev-de:xwasser:codeliste:medium", "3"),
    ("urn:xoev-de:xwasser:codeliste:parameterauspraegung", "5"),
    ("urn:xoev-de:xwasser:codeliste:probenentnahmegeraet", "1"),
    ("urn:xoev-de:xwasser:codeliste:probengefaess", "1"),
    ("urn:xoev-de:xwasser:codeliste:probennahmeverfahren", "4"),
    ("urn:xoev-de:xwasser:codeliste:shapth-parameter", "5"),
    (
        "urn:xoev-de:xwasser:codeliste:shapth-parameter-einheit",
        "2",
    ),
    (
        "urn:xoev-de:xwasser:codeliste:unterkategorie-probennahmestelle",
        "1",
    ),
    ("urn:xoev-de:xwasser:codeliste:untersuchung-anlass", "3"),
    ("urn:xoev-de:xwasser:codeliste:untersuchungsverfahren", "3"),
];

/// Current published version of a sender-chosen codelist.
///
/// Returns an empty string when the codelist is unknown, which keeps a document
/// constructible; the schema validation of the produced document will reject it.
pub fn current_version(urn: &str) -> &'static str {
    CURRENT_VERSIONS
        .iter()
        .find_map(|(codelist, version)| (*codelist == urn).then_some(*version))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::{CURRENT_VERSIONS, current_version};

    #[test]
    fn every_entry_is_reachable() {
        for (urn, version) in CURRENT_VERSIONS {
            assert_eq!(current_version(urn), *version);
        }
        assert_eq!(current_version("urn:unknown"), "");
    }

    #[test]
    fn table_has_the_sender_chosen_codelists() {
        // 27 = the code type 3 codelists of XWasser 1.0.1
        assert_eq!(CURRENT_VERSIONS.len(), 27);
    }
}
