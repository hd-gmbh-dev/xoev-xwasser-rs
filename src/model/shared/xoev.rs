#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

use xoev_xwasser_derive::XWasserValidate;

#[cfg(feature = "wasm")]
use tsify::Tsify;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

use crate::{TNS, model::codes::GenericXwasserCode};

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct XWasserXoevCode {
    #[xml(name = b"code", ty = "child")]
    pub code: String,
    #[xml(name = b"name", ty = "child")]
    #[cfg_attr(feature = "builder", builder(default))]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub name: Option<String>,
    #[xml(name = b"listURI", ty = "attr")]
    #[cfg_attr(feature = "builder", builder(default))]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub list_uri: Option<String>,
    #[xml(name = b"listVersionID", ty = "attr")]
    #[cfg_attr(feature = "builder", builder(default))]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub list_version_id: Option<String>,
}

impl GenericXwasserCode for XWasserXoevCode {
    fn code(&self) -> &str {
        &self.code
    }
}
