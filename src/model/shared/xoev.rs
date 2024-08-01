#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify::Tsify;

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct XoevCode {
    #[xml(name = b"code", ty = "child")]
    pub code: String,
    #[xml(name = b"name", ty = "child")]
    pub name: Option<String>,
    #[xml(name = b"listURI", ty = "attr")]
    pub list_uri: Option<String>,
    #[xml(name = b"listVersionID", ty = "attr")]
    pub list_version_id: Option<String>,
}
