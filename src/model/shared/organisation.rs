#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

use crate::model::codes::deserialize_list_of_codes;
use crate::model::codes::deserialize_optional_code;
use xoev_xwasser_derive::XWasserValidate;

#[cfg(feature = "wasm")]
use tsify::Tsify;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

use crate::{TNS, model::codes::CodeRechtsformenType};

use super::{
    anschrift::AnschriftType, behoerde::BehoerdeType, kommunikation::KommunikationType,
    misc::IdentifikationType, xoev::XWasserXoevCode, zeitraum::ZeitraumType,
};

/// Eine Organisation ist eine Vereinigung mehrerer natürlicher oder juristischer
/// Personen bzw. eine rechtsfähige Personengesellschaft zu einem gemeinsamen Zweck, z.B.
/// im wirtschaftlichen, gemeinnützigen, religiösen, öffentlichen oder politischen
/// Bereich. Behörden werden über eine eigene Kernkomponente "Behoerde" abgebildet.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct OrganisationType {
    #[xml(name = b"id", ty = "attr")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub id: Option<String>,
    #[xml(ns = b"xwas", name = b"rechtsform", ty = "child")]
    #[serde(deserialize_with = "deserialize_optional_code")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub rechtsform: Option<CodeRechtsformenType>,
    #[xml(ns = b"xwas", name = b"branche", ty = "child")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_list_of_codes")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub branche: Vec<XWasserXoevCode>,
    #[xml(ns = b"xwas", name = b"zweck", ty = "child")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_list_of_codes")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub zweck: Vec<XWasserXoevCode>,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub name: Option<NameOrganisationType>,
    #[xml(ns = b"xwas", name = b"unterorganisation", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub unterorganisation: Vec<OrganisationType>,
    #[xml(ns = b"xwas", name = b"kommunikation", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub kommunikation: Vec<KommunikationType>,
    #[xml(ns = b"xwas", name = b"registrierung", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub registrierung: Vec<RegistrierungType>,
    #[xml(ns = b"xwas", name = b"identifikation", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub identifikation: Vec<IdentifikationType>,
    #[xml(ns = b"xwas", name = b"existenzzeitraum", ty = "child")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub existenzzeitraum: Option<ZeitraumType>,
    #[xml(ns = b"xwas", name = b"anschrift", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub anschrift: Vec<AnschriftType>,
}

/// Angaben zum Registereintrag.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct RegistrierungType {
    #[xml(ns = b"xwas", name = b"id", ty = "child")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub id: Option<String>,
    #[xml(ns = b"xwas", name = b"registertyp", ty = "child")]
    #[serde(deserialize_with = "deserialize_optional_code")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub registertyp: Option<XWasserXoevCode>,
    #[xml(ns = b"xwas", name = b"registrierendeBehoerde", ty = "child")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub registrierende_behoerde: Vec<BehoerdeType>,
    #[xml(ns = b"xwas", name = b"gueltigkeit", ty = "child")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub gueltigkeit: Option<ZeitraumType>,
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
pub struct Name {
    #[xml(ty = "text")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub text: Option<String>,
}

/// "NameOrganisation" fasst die Angaben zum Namen einer Organisation zusammen.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct NameOrganisationType {
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub name: Option<Name>,
    #[xml(ns = b"xwas", name = b"kurzbezeichnung", ty = "child")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub kurzbezeichnung: Option<String>,
    #[xml(ns = b"xwas", name = b"gueltigkeit", ty = "child")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub gueltigkeit: Option<ZeitraumType>,
}

/// Die Organisationseinheit fasst Angaben zur Darstellung der internen hierarchischen
/// Organisationsstruktur einer Institution zusammen, z.B. zur Darstellung von
/// Abteilungen oder Referaten.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct OrganisationseinheitType {
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: String,
    #[xml(ns = b"xwas", name = b"hierarchieebene", ty = "child")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub hierarchieebene: Option<i32>, //Option<u8>,
    #[xml(ns = b"xwas", name = b"hierarchiename", ty = "child")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub hierarchiename: Option<String>, //Option<String>,
}
