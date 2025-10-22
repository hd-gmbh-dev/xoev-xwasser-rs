#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

use xoev_xwasser_derive::XWasserValidate;

#[cfg(feature = "wasm")]
use tsify::Tsify;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

/// Der Zeitraum kennzeichnet einen Abschnitt auf einem Zeitstrahl durch Angabe von
/// Beginn und/oder Ende.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_9_5/"
))]
pub struct ZeitraumType {
    #[xml(ns = b"xwas", name = b"beginn", ty = "child")]
    pub beginn: Option<String>, // eigentlich of type xs:date
    #[xml(ns = b"xwas", name = b"ende", ty = "child")]
    pub ende: Option<String>,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    pub zusatz: Option<String>,
}
