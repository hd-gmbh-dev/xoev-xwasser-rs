#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify::Tsify;

#[derive(Clone, Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_5_2/"
))]
pub struct XWasserXoevCode {
    #[xml(name = b"code", ty = "child")]
    pub code: String,
    #[xml(name = b"name", ty = "child")]
    pub name: Option<String>,
    #[xml(name = b"listURI", ty = "attr")]
    pub list_uri: Option<String>,
    #[xml(name = b"listVersionID", ty = "attr")]
    pub list_version_id: Option<String>,
}
