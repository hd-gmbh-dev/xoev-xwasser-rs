#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::model::codes::CodeUntersuchungsstelleType;

use super::organisation::OrganisationType;

/// Klasse für den Transport von Informationen zu einer zugelassenen Untersuchungsstelle.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
))]
pub struct ZugelasseneUntersuchungsstelleType {
    #[xml(name = b"organisation", ty = "child")]
    pub organisation: OrganisationType,
    #[xml(name = b"zugelasseneUntersuchungsstelleID", ty = "child")]
    pub zugelassene_untersuchungsstelle_id: String, //ConstStr,
    #[xml(name = b"nameZugelasseneUntersuchungsstelle", ty = "child")]
    pub name_zugelassene_untersuchungsstelle: CodeUntersuchungsstelleType,
    #[xml(name = b"pruefgebieteUntersuchungenPhysChem", ty = "child")]
    pub pruefgebiete_untersuchungen_phys_chem: Option<bool>,
    #[xml(name = b"pruefgebieteUntersuchungenMikrobio", ty = "child")]
    pub pruefgebiete_untersuchungen_mikrobio: Option<bool>,
    #[xml(name = b"pruefgebieteUntersuchungenRadionuklide", ty = "child")]
    pub pruefgebiete_untersuchungen_radionuklide: Option<bool>,
    #[xml(name = b"akkreditierungsnummer", ty = "child")]
    pub akkreditierungsnummer: Option<String>,
    #[xml(name = b"kommentarBeauftragteUntersuchungsstelle", ty = "child")]
    pub kommentar_beauftragte_untersuchungsstelle: Option<String>,
    #[xml(name = b"kommentarZugelasseneUntersuchungsstelle", ty = "child")]
    pub kommentar_zugelassene_untersuchungsstelle: Option<String>,
}

/// Klasse für den Transport von ergänzenden Informationen zu den Daten aus dem Register
/// für Zugelassene Untersuchungsstellen im Falle der Beauftragung einer Untersuchung.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
))]
pub struct BeauftragteUntersuchungsstelleType {
    #[xml(ns = b"xwas", name = b"zugelasseneUntersuchungsstelle", ty = "child")]
    pub zugelassene_untersuchungsstelle: ZugelasseneUntersuchungsstelleType,
    #[xml(
        ns = b"xwas",
        name = b"kommentarBeauftragteUntersuchungsstelle",
        ty = "child"
    )]
    pub kommentar_beauftragte_untersuchungsstelle: Option<String>,
}
