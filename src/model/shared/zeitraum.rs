#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify::Tsify;

/// Der Zeitraum kennzeichnet einen Abschnitt auf einem Zeitstrahl durch Angabe von
/// Beginn und/oder Ende.
#[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
))]
pub struct ZeitraumType {
    #[xml(ns = b"xwas", name = b"beginn", ty = "child")]
    pub beginn: Option<String>, // eigentlich of type xs:date
    #[xml(ns = b"xwas", name = b"ende", ty = "child")]
    pub ende: Option<String>,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    pub zusatz: Option<String>,
}
