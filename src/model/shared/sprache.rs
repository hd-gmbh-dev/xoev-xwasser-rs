#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

use crate::model::codes::CodeSpracheType;

#[cfg(feature = "wasm")]
use tsify_next::Tsify;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

/// Unter "Sprache" werden Informationen über Sprachen zusammengefasst.
#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_6_0/"
))]
pub struct SpracheType {
    #[xml(ns = b"xwas", name = b"sprache", ty = "child")]
    pub sprache: CodeSpracheType,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    pub zusatz: Option<String>,
}
