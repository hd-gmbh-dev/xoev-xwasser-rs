#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify_next::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

use super::{
    codes::CodeDokumenttypType, pruefbericht::PruefberichtType,
    shared::dokument::DokumentRepraesentationType,
    shared::untersuchungsplan::UntersuchungsplanType,
};

#[derive(Clone, Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_7_0/"
))]
pub struct IdentifikationVorgang {
    #[xml(ns = b"xwas", name = b"vorgangsID", ty = "child")]
    pub vorgangs_id: String,
    // #[xml(ns = b"xwas", name = b"aktenzeichen", ty = "child")]
    // pub aktenzeichen: String,
}

/// Dieser Datentyp enthält die Angaben zu einem Vorgang.
#[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_7_0/"
))]
pub struct Vorgang {
    #[xml(ns = b"xwas", name = b"identifikationVorgang", ty = "child")]
    pub identifikation_vorgang: IdentifikationVorgang,
    #[xml(ns = b"xwas", name = b"vorgangType", ty = "child")]
    pub vorgang_type: VorgangType,
    #[xml(ns = b"xwas", name = b"bemerkung", ty = "child")]
    pub bemerkung: Option<String>,
    #[xml(ns = b"xwas", name = b"anlage", ty = "child")]
    #[serde(default)]
    pub anlage: Vec<DokumentType>,
}

/// Eine zum Antrag gehörige Unterlage in verschiedenen Dokumentendarstellungen, z. B. PDF oder eine Datendarstellung. Es muss immer das komplette Dokument mit allen Darstellungen übertragen werden.
#[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_7_0/"
))]
pub struct DokumentType {
    #[xml(ns = b"xwas", name = b"dokumentTyp", ty = "child")]
    pub dokument_typ: CodeDokumenttypType,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: String,
    #[xml(ns = b"xwas", name = b"aktuelleVersion", ty = "child")]
    pub aktuelle_version: Option<String>,
    #[xml(ns = b"xwas", name = b"letzteVersion", ty = "child")]
    pub letzte_version: Option<String>,
    #[xml(ns = b"xwas", name = b"dokumentRepraesentation", ty = "child")]
    pub dokument_repraesentation: DokumentRepraesentationType,
    #[xml(ns = b"xwas", name = b"personReferenzID", ty = "child")]
    #[serde(default)]
    pub person_referenz_id: Vec<String>,
    #[xml(name = b"dokumentID", ty = "attr")]
    pub dokument_id: String,
}

// TODO: implement Box<T>, Arc<T>, Rc<T> for raxb
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "t", content = "c")]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_7_0/"
))]
pub enum VorgangType {
    #[xml(ns = b"xwas", name = b"pruefbericht")]
    Pruefbericht(PruefberichtType),
    #[xml(ns = b"xwas", name = b"untersuchungsplan")]
    Untersuchungsplan(UntersuchungsplanType),
    #[xml(ns = b"xwas", name = b"olb_bericht")]
    OlbBericht(DokumentType),
    #[default]
    #[xml(ns = b"xwas", name = b"unknown")]
    None,
}
