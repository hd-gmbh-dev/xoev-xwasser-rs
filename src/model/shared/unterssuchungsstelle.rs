#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::model::codes::{CodeRechtsformenType, CodeUntersuchungsstelleType};

use super::{
    anschrift::AnschriftType,
    kommunikation::KommunikationType,
    misc::IdentifikationType,
    organisation::{NameOrganisationType, OrganisationType, RegistrierungType},
    xoev::XWasserXoevCode,
    zeitraum::ZeitraumType,
};

/// Klasse für den Transport von Informationen zu einer zugelassenen Untersuchungsstelle.
#[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_5_3/"
))]
pub struct ZugelasseneUntersuchungsstelleType {
    // TODO: should be flattend by raxb
    #[xml(ns = b"xwas", name = b"rechtsform", ty = "child")]
    pub rechtsform: Option<CodeRechtsformenType>,
    #[xml(ns = b"xwas", name = b"branche", ty = "child")]
    #[serde(default)]
    pub branche: Vec<XWasserXoevCode>,
    #[xml(ns = b"xwas", name = b"zweck", ty = "child")]
    #[serde(default)]
    pub zweck: Vec<XWasserXoevCode>,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<NameOrganisationType>,
    #[xml(ns = b"xwas", name = b"unterorganisation", ty = "child")]
    #[serde(default)]
    pub unterorganisation: Vec<OrganisationType>,
    #[xml(ns = b"xwas", name = b"kommunikation", ty = "child")]
    #[serde(default)]
    pub kommunikation: Vec<KommunikationType>,
    #[xml(ns = b"xwas", name = b"registrierung", ty = "child")]
    #[serde(default)]
    pub registrierung: Vec<RegistrierungType>,
    #[xml(ns = b"xwas", name = b"identifikation", ty = "child")]
    #[serde(default)]
    pub identifikation: Vec<IdentifikationType>,
    #[xml(ns = b"xwas", name = b"existenzzeitraum", ty = "child")]
    pub existenzzeitraum: Option<ZeitraumType>,
    #[xml(ns = b"xwas", name = b"anschrift", ty = "child")]
    #[serde(default)]
    pub anschrift: Vec<AnschriftType>,
    #[xml(ns = b"xwas", name = b"zugelasseneUntersuchungsstelleID", ty = "child")]
    pub zugelassene_untersuchungsstelle_id: String, //ConstStr,
    #[xml(
        ns = b"xwas",
        name = b"nameZugelasseneUntersuchungsstelle",
        ty = "child"
    )]
    pub name_zugelassene_untersuchungsstelle: CodeUntersuchungsstelleType,
    #[xml(
        ns = b"xwas",
        name = b"pruefgebieteUntersuchungenPhysChem",
        ty = "child"
    )]
    pub pruefgebiete_untersuchungen_phys_chem: Option<bool>,
    #[xml(
        ns = b"xwas",
        name = b"pruefgebieteUntersuchungenMikrobio",
        ty = "child"
    )]
    pub pruefgebiete_untersuchungen_mikrobio: Option<bool>,
    #[xml(
        ns = b"xwas",
        name = b"pruefgebieteUntersuchungenRadionuklide",
        ty = "child"
    )]
    pub pruefgebiete_untersuchungen_radionuklide: Option<bool>,
    #[xml(ns = b"xwas", name = b"akkreditierungsnummer", ty = "child")]
    pub akkreditierungsnummer: Option<String>,
    #[xml(
        ns = b"xwas",
        name = b"kommentarBeauftragteUntersuchungsstelle",
        ty = "child"
    )]
    pub kommentar_beauftragte_untersuchungsstelle: Option<String>,
    #[xml(
        ns = b"xwas",
        name = b"kommentarZugelasseneUntersuchungsstelle",
        ty = "child"
    )]
    pub kommentar_zugelassene_untersuchungsstelle: Option<String>,
}

/// Klasse für den Transport von ergänzenden Informationen zu den Daten aus dem Register
/// für Zugelassene Untersuchungsstellen im Falle der Beauftragung einer Untersuchung.
#[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_5_3/"
))]
pub struct BeauftragteUntersuchungsstelleType {
    // TODO: should be flattend by raxb
    #[xml(ns = b"xwas", name = b"rechtsform", ty = "child")]
    pub rechtsform: Option<CodeRechtsformenType>,
    #[xml(ns = b"xwas", name = b"branche", ty = "child")]
    #[serde(default)]
    pub branche: Vec<XWasserXoevCode>,
    #[xml(ns = b"xwas", name = b"zweck", ty = "child")]
    #[serde(default)]
    pub zweck: Vec<XWasserXoevCode>,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<NameOrganisationType>,
    #[xml(ns = b"xwas", name = b"unterorganisation", ty = "child")]
    #[serde(default)]
    pub unterorganisation: Vec<OrganisationType>,
    #[xml(ns = b"xwas", name = b"kommunikation", ty = "child")]
    #[serde(default)]
    pub kommunikation: Vec<KommunikationType>,
    #[xml(ns = b"xwas", name = b"registrierung", ty = "child")]
    #[serde(default)]
    pub registrierung: Vec<RegistrierungType>,
    #[xml(ns = b"xwas", name = b"identifikation", ty = "child")]
    #[serde(default)]
    pub identifikation: Vec<IdentifikationType>,
    #[xml(ns = b"xwas", name = b"existenzzeitraum", ty = "child")]
    pub existenzzeitraum: Option<ZeitraumType>,
    #[xml(ns = b"xwas", name = b"anschrift", ty = "child")]
    #[serde(default)]
    pub anschrift: Vec<AnschriftType>,
    #[xml(ns = b"xwas", name = b"zugelasseneUntersuchungsstelleID", ty = "child")]
    pub zugelassene_untersuchungsstelle_id: String, //ConstStr,
    #[xml(
        ns = b"xwas",
        name = b"nameZugelasseneUntersuchungsstelle",
        ty = "child"
    )]
    pub name_zugelassene_untersuchungsstelle: CodeUntersuchungsstelleType,
    #[xml(
        ns = b"xwas",
        name = b"pruefgebieteUntersuchungenPhysChem",
        ty = "child"
    )]
    pub pruefgebiete_untersuchungen_phys_chem: Option<bool>,
    #[xml(
        ns = b"xwas",
        name = b"pruefgebieteUntersuchungenMikrobio",
        ty = "child"
    )]
    pub pruefgebiete_untersuchungen_mikrobio: Option<bool>,
    #[xml(
        ns = b"xwas",
        name = b"pruefgebieteUntersuchungenRadionuklide",
        ty = "child"
    )]
    pub pruefgebiete_untersuchungen_radionuklide: Option<bool>,
    #[xml(ns = b"xwas", name = b"akkreditierungsnummer", ty = "child")]
    pub akkreditierungsnummer: Option<String>,
    #[xml(
        ns = b"xwas",
        name = b"kommentarZugelasseneUntersuchungsstelle",
        ty = "child"
    )]
    pub kommentar_zugelassene_untersuchungsstelle: Option<String>,
    #[xml(
        ns = b"xwas",
        name = b"kommentarBeauftragteUntersuchungsstelle",
        ty = "child"
    )]
    pub kommentar_beauftragte_untersuchungsstelle: Option<String>,
}
