#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

use xoev_xwasser_derive::XWasserValidate;

use crate::{model::codes::CodeSpracheType, TNS};

#[cfg(feature = "wasm")]
use tsify::Tsify;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

/// Unter "Sprache" werden Informationen über Sprachen zusammengefasst.
#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct SpracheType {
    #[xml(ns = b"xwas", name = b"sprache", ty = "child")]
    pub sprache: CodeSpracheType,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    pub zusatz: Option<String>,
}
