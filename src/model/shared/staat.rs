#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::model::codes::{CodeStaatType, CodeStaatsangehoerigkeitType};

/// Hier werden Angaben zur Staatsangehörigkeit zusammengefasst.
#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_6_0/"
))]
pub struct StaatsangehoerigkeitType {
    #[xml(ns = b"xwas", name = b"staatsangehoerigkeit", ty = "child")]
    pub staatsangehoerigkeit: CodeStaatsangehoerigkeitType,
}

/// Als Staat bezeichnet man eine politische Ordnung, die ein gemeinsames als
/// Staatsgebiet abgegrenztes Territorium, ein dazugehöriges Staatsvolk und eine
/// Machtausübung über dieses umfasst.
#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_6_0/"
))]
pub struct StaatType {
    #[xml(ns = b"xwas", name = b"staat", ty = "child")]
    pub staat: CodeStaatType,
}
