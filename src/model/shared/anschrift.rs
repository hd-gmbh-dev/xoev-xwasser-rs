#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify_next::Tsify;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

use crate::model::codes::CodeAnschrifttypType;

use super::{
    behoerde::VerwaltungspolitischeKodierungType, staat::StaatType, xoev::XWasserXoevCode,
};

/// Eine Anschrift beschreibt einen Ort mit den klassischen Ordnungsbegriffen wie Orts-
/// und Straßennamen sowie ergänzenden Informationen wie Ortsteil und Postfach. Eine
/// Anschrift kann genutzt werden, um Orte zu benennen, an denen sich Personen aufhalten,
/// an denen Objekte zu finden sind, oder an denen Ereignisse stattfinden. Darüber hinaus
/// kann sie genutzt werden, um Post oder Waren zuzustellen. Daher enthält sie auch die
/// notwendigen Attribute um Postfächer zu adressieren. Die Anschrift kann außerdem über
/// eine Subkomponente verfügen, die eine Beschreibung des Ortes mittels Geokoordinaten
/// erlaubt. Die Anschrift kann auch über eine Subkomponente verfügen, die eine
/// verwaltungspolitische Zuordnung des Ortes erlaubt (Zuordnung zu einer Gemeinde über
/// den AGS, eines Bundesland, etc.).
#[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_7_0/"
))]
pub struct AnschriftType {
    #[xml(ns = b"xwas", name = b"strassenschluessel", ty = "child")]
    pub strassenschluessel: Option<XWasserXoevCode>,
    #[xml(ns = b"xwas", name = b"strasse", ty = "child")]
    pub strasse: Option<String>,
    #[xml(ns = b"xwas", name = b"hausnummer", ty = "child")]
    pub hausnummer: Option<String>,
    #[xml(ns = b"xwas", name = b"postfach", ty = "child")]
    pub postfach: Option<String>,
    #[xml(ns = b"xwas", name = b"postleitzahl", ty = "child")]
    pub postleitzahl: Option<String>,
    #[xml(ns = b"xwas", name = b"ort", ty = "child")]
    pub ort: Option<String>,
    #[xml(ns = b"xwas", name = b"ortsteil", ty = "child")]
    pub ortsteil: Option<String>,
    #[xml(ns = b"xwas", name = b"ortFruehererGemeindename", ty = "child")]
    pub ort_frueherer_gemeindename: Option<String>,
    #[xml(ns = b"xwas", name = b"wohnungsgeber", ty = "child")]
    pub wohnungsgeber: Option<String>,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    pub zusatz: Option<String>,
    #[xml(ns = b"xwas", name = b"typ", ty = "child")]
    #[serde(default)]
    pub typ: Vec<CodeAnschrifttypType>,
    #[xml(ns = b"xwas", name = b"staat", ty = "child")]
    pub staat: Option<StaatType>,
    #[xml(ns = b"xwas", name = b"verwaltungspolitischeKodierung", ty = "child")]
    pub verwaltungspolitische_kodierung: Option<VerwaltungspolitischeKodierungType>,
    #[xml(name = b"id", ty = "attr")]
    pub id: String,
}
