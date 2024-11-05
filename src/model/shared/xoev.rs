#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify_next::Tsify;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_7_0/"
))]
pub struct XWasserXoevCode {
    #[xml(name = b"code", ty = "child")]
    pub code: String,
    #[xml(name = b"name", ty = "child")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub name: Option<String>,
    #[xml(name = b"listURI", ty = "attr")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub list_uri: Option<String>,
    #[xml(name = b"listVersionID", ty = "attr")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub list_version_id: Option<String>,
}
