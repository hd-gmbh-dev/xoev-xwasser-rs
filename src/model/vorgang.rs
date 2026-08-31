#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

use xoev_xwasser_derive::XWasserValidate;

#[cfg(feature = "wasm")]
use tsify::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

use crate::TNS;

use super::pruefbericht::PruefberichtType;
use super::shared::{dokument::DokumentType, untersuchungsplan::UntersuchungsplanType};
use super::codes::CodeUebermittlungsartType;

/// Type name: JahresberichtType
/// Klasse für den Transport von Informationen zu einem Jahresbericht.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct JahresberichtType {
    #[xml(ns = b"xwas", name = b"jahresberichtID", ty = "child")]
    pub jahresbericht_id: String,
    #[xml(ns = b"xwas", name = b"titel", ty = "child")]
    pub titel: String,
    #[xml(ns = b"xwas", name = b"uebermittlungsart", ty = "child")]
    pub uebermittlungsart: CodeUebermittlungsartType,
    #[xml(ns = b"xwas", name = b"dokumentreferenz", ty = "child")]
    pub dokumentreferenz: Vec<String>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub kommentar: Option<String>,
    #[xml(name = b"id", ty = "attr")]
    pub id: String,
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct IdentifikationVorgang {
    #[xml(ns = b"xwas", name = b"vorgangsID", ty = "child")]
    pub vorgangs_id: String,
    // #[xml(ns = b"xwas", name = b"aktenzeichen", ty = "child")]
    // pub aktenzeichen: String,
}

/// Dieser Datentyp enthält die Angaben zu einem Vorgang.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct Vorgang {
    #[xml(ns = b"xwas", name = b"identifikationVorgang", ty = "child")]
    pub identifikation_vorgang: IdentifikationVorgang,
    #[xml(ns = b"xwas", name = b"vorgangType", ty = "child")]
    pub vorgang_type: VorgangType,
    #[xml(ns = b"xwas", name = b"bemerkung", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub bemerkung: Option<String>,
    #[xml(ns = b"xwas", name = b"anlage", ty = "child")]
    #[serde(default)]
    pub anlage: Vec<DokumentType>,
}

// TODO: implement Box<T>, Arc<T>, Rc<T> for raxb
#[allow(clippy::large_enum_variant)]
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "t", content = "c")]
#[xml(tns(b"xwas", TNS))]
pub enum VorgangType {
    #[xml(ns = b"xwas", name = b"pruefbericht")]
    Pruefbericht(PruefberichtType),
    #[xml(ns = b"xwas", name = b"untersuchungsplan")]
    Untersuchungsplan(UntersuchungsplanType),
    #[xml(ns = b"xwas", name = b"jahresbericht")]
    Jahresbericht(JahresberichtType),
    #[default]
    #[xml(ns = b"xwas", name = b"unknown")]
    None,
}
