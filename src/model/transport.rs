#![allow(non_snake_case, dead_code)]

use raxb::{value::ConstStr, XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

use xoev_xwasser_derive::XWasserValidate;

#[cfg(feature = "wasm")]
use tsify::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

use crate::TNS;

use super::{signature::Signature, vorgang::Vorgang};

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
pub struct Code {
    #[xml(ty = "text", default)]
    #[cfg_attr(feature = "builder", builder(default))]
    pub code: String,
}

impl From<&str> for Code {
    fn from(value: &str) -> Self {
        Self {
            code: value.to_string(),
        }
    }
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
pub struct NachrichtenkopfG2g {
    #[xml(name = b"identifikation.nachricht", ty = "child")]
    pub identifikation_nachricht: IdentifikationNachricht,
    #[xml(name = b"leser", ty = "child")]
    pub leser: BehoerdeG2GType,
    #[xml(name = b"autor", ty = "child")]
    pub autor: BehoerdeG2GType,
    #[xml(name = b"referenzUUID", ty = "child")]
    pub referenz_uuid: Option<String>,
    #[xml(default, name = b"dvdvDienstkennung", ty = "child")]
    pub dvdv_dienstkennung: String,
    #[serde(default)]
    #[xml(name = b"zustaendigeBehoerdeID", ty = "child")]
    pub zustaendige_behoerde_id: Vec<String>,
    #[xml(name = b"wasserversorgungsgebietID", ty = "child")]
    pub wasserversorgungsgebiet_id: Option<String>,
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
pub struct IdentifikationNachricht {
    #[xml(name = b"nachrichtenUUID", ty = "child")]
    pub nachrichten_uuid: Option<String>, //ConstStr,
    #[xml(name = b"nachrichtentyp", ty = "child")]
    pub nachrichten_typ: Option<NachrichtenTyp>,
    #[xml(name = b"erstellungszeitpunkt", ty = "child")]
    pub erstellungszeitpunkt: Option<String>,
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
pub struct NachrichtenTyp {
    #[xml(
        default,
        name = b"listURI",
        ty = "attr",
        value = "urn:xoev-de:xwasser:codeliste:nachrichtentyp"
    )]
    #[serde(skip)]
    #[cfg_attr(feature = "builder", builder(default))]
    pub _list_uri: ConstStr,
    #[xml(default, name = b"listVersionID", ty = "attr", value = "1")]
    #[serde(skip)]
    #[cfg_attr(feature = "builder", builder(default))]
    pub _list_version_id: ConstStr,
    #[xml(name = b"code", ty = "child")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub code: Code,
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
pub struct BehoerdeG2GType {
    #[xml(name = b"verzeichnisdienst", ty = "child")]
    pub verzeichnisdienst: Verzeichnisdienst,
    #[xml(name = b"kennung", ty = "child")]
    pub kennung: String,
    #[xml(name = b"name", ty = "child")]
    pub name: String,
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
pub struct Verzeichnisdienst {
    #[xml(name = b"listVersionID", ty = "attr", value = "")]
    #[serde(skip)]
    #[cfg_attr(feature = "builder", builder(default))]
    pub _list_version_id: ConstStr,
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
}

#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(root = b"vorgang.transportieren.2010")]
#[xml(tns(b"xwas", TNS))]
pub struct VorgangTransportieren2010 {
    #[xml(
        ns = b"xmlns",
        name = b"xsi",
        ty = "attr",
        value = "http://www.w3.org/2001/XMLSchema-instance",
        default
    )]
    #[serde(skip)]
    #[cfg_attr(feature = "builder", builder(default))]
    _xmlns_xsi: ConstStr,
    #[xml(
        ns = b"xsi",
        name = b"schemaLocation",
        ty = "attr",
        value = "https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_9_5/ xwasser.xsd",
        default
    )]
    #[serde(skip)]
    #[cfg_attr(feature = "builder", builder(default))]
    schema_location: ConstStr,
    #[xml(
        ns = b"xmlns",
        name = b"xwas",
        ty = "attr",
        value = "https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_9_5/",
        default
    )]
    #[serde(skip)]
    #[cfg_attr(feature = "builder", builder(default))]
    _xmlns: ConstStr,
    #[xml(name = b"produkt", ty = "attr")]
    pub produkt: String,
    #[xml(name = b"produkthersteller", ty = "attr")]
    pub produkthersteller: String,
    #[xml(name = b"produktversion", ty = "attr")]
    pub produktversion: String,
    #[xml(name = b"standard", ty = "attr", value = "XWasser")]
    #[serde(skip)]
    #[cfg_attr(feature = "builder", builder(default))]
    _standard: ConstStr,
    #[xml(name = b"test", ty = "attr")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub test: Option<bool>,
    #[xml(name = b"version", ty = "attr", value = "0.9.5")]
    #[serde(skip)]
    #[cfg_attr(feature = "builder", builder(default))]
    _version: ConstStr,
    #[xml(name = b"nachrichtenkopf.g2g", ty = "child")]
    pub nachrichtenkopf_g2g: NachrichtenkopfG2g,
    #[xml(ns = b"xwas", name = b"vorgang", ty = "child")]
    pub vorgang: Vorgang,
    #[xml(ns = b"ds", name = b"Signature", ty = "child")]
    pub signature: Option<Signature>,
}
