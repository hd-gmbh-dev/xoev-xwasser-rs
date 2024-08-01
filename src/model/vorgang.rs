#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::pruefbericht::PruefberichtType;

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
))]
pub struct IdentifikationVorgang {
    #[xml(ns = b"xwas", name = b"vorgangsID", ty = "child")]
    pub vorgangs_id: String,
    // #[xml(ns = b"xwas", name = b"aktenzeichen", ty = "child")]
    // pub aktenzeichen: String,
}

/// Dieser Datentyp enthält die Angaben zu einem Vorgang.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
))]
pub struct Vorgang {
    #[xml(ns = b"xwas", name = b"identifikationVorgang", ty = "child")]
    pub identifikation_vorgang: IdentifikationVorgang,
    #[xml(ns = b"xwas", name = b"vorgangType", ty = "child")]
    pub vorgang_type: VorgangType,
    #[xml(ns = b"xwas", name = b"bemerkung", ty = "child")]
    pub bemerkung: Option<String>,
    // #[xml(ns=b"xwas", name = b"anlage", ty = "child")]
    // #[serde(default)]
    // pub anlage: Vec<Anlage>,
}

// TODO: implement Box<T>, Arc<T>, Rc<T> for raxb
#[allow(clippy::large_enum_variant)]
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[serde(tag = "t", content = "c")]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
))]
pub enum VorgangType {
    #[xml(ns = b"xwas", name = b"pruefbericht")]
    Pruefbericht(PruefberichtType),
    // #[xml(ns = b"xwas", name = b"untersuchungsplan")]
    // Untersuchungsplan(UntersuchungsplanType),
    // #[xml(ns = b"xwas", name = b"olgBericht")]
    // OlgBericht(DokumentType),
    #[default]
    #[xml(ns = b"xwas", name = b"unknown")]
    None,
}
