#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

use xoev_xwasser_derive::XWasserValidate;

#[cfg(feature = "wasm")]
use tsify_next::Tsify;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

use crate::model::codes::{CodeStaatType, CodeStaatsangehoerigkeitType};

/// Hier werden Angaben zur Staatsangehörigkeit zusammengefasst.
#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_9_2/"
))]
pub struct StaatsangehoerigkeitType {
    #[xml(ns = b"xwas", name = b"staatsangehoerigkeit", ty = "child")]
    pub staatsangehoerigkeit: CodeStaatsangehoerigkeitType,
}

/// Als Staat bezeichnet man eine politische Ordnung, die ein gemeinsames als
/// Staatsgebiet abgegrenztes Territorium, ein dazugehöriges Staatsvolk und eine
/// Machtausübung über dieses umfasst.
#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_9_2/"
))]
pub struct StaatType {
    #[xml(ns = b"xwas", name = b"staat", ty = "child")]
    pub staat: CodeStaatType,
}
