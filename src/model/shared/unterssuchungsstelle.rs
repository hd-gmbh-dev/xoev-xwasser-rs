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
    anschrift::AnschriftType,
    kommunikation::KommunikationType,
    misc::IdentifikationType,
    organisation::{NameOrganisationType, OrganisationType, RegistrierungType},
    xoev::XWasserXoevCode,
    zeitraum::ZeitraumType,
};

/// Klasse für den Transport von Informationen zu einer zugelassenen Untersuchungsstelle.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct ZugelasseneUntersuchungsstelleType {
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
    #[xml(ns = b"xwas", name = b"zugelasseneUntersuchungsstelleID", ty = "child")]
    pub zugelassene_untersuchungsstelle_id: String, //ConstStr,
    #[xml(
        ns = b"xwas",
        name = b"pruefgebieteUntersuchungenPhysChem",
        ty = "child"
    )]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub pruefgebiete_untersuchungen_phys_chem: Option<bool>,
    #[xml(
        ns = b"xwas",
        name = b"pruefgebieteUntersuchungenMikrobio",
        ty = "child"
    )]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub pruefgebiete_untersuchungen_mikrobio: Option<bool>,
    #[xml(
        ns = b"xwas",
        name = b"pruefgebieteUntersuchungenRadionuklide",
        ty = "child"
    )]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub pruefgebiete_untersuchungen_radionuklide: Option<bool>,
    #[xml(ns = b"xwas", name = b"pruefgebieteNurVorOrtParameter", ty = "child")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub pruefgebiete_nur_vor_ort_parameter: Option<bool>,
    #[xml(ns = b"xwas", name = b"akkreditierungsnummer", ty = "child")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub akkreditierungsnummer: Option<String>,
    #[xml(
        ns = b"xwas",
        name = b"kommentarZugelasseneUntersuchungsstelle",
        ty = "child"
    )]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub kommentar_zugelassene_untersuchungsstelle: Option<String>,
}

/// Klasse für den Transport von ergänzenden Informationen zu den Daten aus dem Register
/// für Zugelassene Untersuchungsstellen im Falle der Beauftragung einer Untersuchung.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct BeauftragteUntersuchungsstelleType {
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
    #[xml(ns = b"xwas", name = b"zugelasseneUntersuchungsstelleID", ty = "child")]
    pub zugelassene_untersuchungsstelle_id: String, //ConstStr,
    #[xml(
        ns = b"xwas",
        name = b"pruefgebieteUntersuchungenPhysChem",
        ty = "child"
    )]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub pruefgebiete_untersuchungen_phys_chem: Option<bool>,
    #[xml(
        ns = b"xwas",
        name = b"pruefgebieteUntersuchungenMikrobio",
        ty = "child"
    )]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub pruefgebiete_untersuchungen_mikrobio: Option<bool>,
    #[xml(
        ns = b"xwas",
        name = b"pruefgebieteUntersuchungenRadionuklide",
        ty = "child"
    )]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub pruefgebiete_untersuchungen_radionuklide: Option<bool>,
    #[xml(ns = b"xwas", name = b"pruefgebieteNurVorOrtParameter", ty = "child")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub pruefgebiete_nur_vor_ort_parameter: Option<bool>,
    #[xml(ns = b"xwas", name = b"akkreditierungsnummer", ty = "child")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub akkreditierungsnummer: Option<String>,
    #[xml(
        ns = b"xwas",
        name = b"kommentarZugelasseneUntersuchungsstelle",
        ty = "child"
    )]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub kommentar_zugelassene_untersuchungsstelle: Option<String>,
    #[xml(
        ns = b"xwas",
        name = b"kommentarBeauftragteUntersuchungsstelle",
        ty = "child"
    )]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub kommentar_beauftragte_untersuchungsstelle: Option<String>,
}
