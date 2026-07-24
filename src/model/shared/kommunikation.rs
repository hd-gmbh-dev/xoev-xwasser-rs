#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

use crate::model::codes::deserialize_optional_code;
use xoev_xwasser_derive::XWasserValidate;

#[cfg(feature = "wasm")]
use tsify::Tsify;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

use crate::{TNS, model::codes::CodeKommunikationType};

/// "Kommunikation" fasst Angaben zur Erreichbarkeit über elektronische
/// Kommunikationskanäle (z.B. Telefon, Fax, E-Mail) zusammen.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct KommunikationType {
    #[xml(ns = b"xwas", name = b"kanal", ty = "child")]
    #[serde(deserialize_with = "deserialize_optional_code")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub kanal: Option<CodeKommunikationType>,
    #[xml(ns = b"xwas", name = b"kennung", ty = "child")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub kennung: Option<String>,
    #[xml(ns = b"xwas", name = b"istDienstlich", ty = "child")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub ist_dienstlich: Option<bool>,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub zusatz: Option<String>,
}
