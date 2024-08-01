#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

use super::codes::{CodeAmtsspracheEuType, CodeGesamtbewertungType, CodeUntersuchungsstelleType};
use super::shared::anschrift::AnschriftType;
use super::shared::auftraggeber::AuftraggeberType;
use super::shared::behoerde::ZustaendigeBehoerdeType;
use super::shared::misc::ErweiterungType;
use super::shared::person::NatuerlichePersonType;
use super::shared::probe::{ProbennahmestelleType, ProbennehmerType};
use super::shared::unterssuchungsstelle::BeauftragteUntersuchungsstelleType;

/// Klasse zur Erfassung bzw. zum Transport der Daten eines Prüfberichts. Prüfberichte
/// werden erstellt, nachdem eine Wasserprobe im Labor analysiert wurde.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
))]
pub struct PruefberichtType {
    #[xml(ns = b"xwas", name = b"pruefberichtUUID", ty = "child")]
    pub pruefbericht_uuid: String,
    #[xml(ns = b"xwas", name = b"probennahmestelle", ty = "child")]
    #[serde(default)]
    pub probennahmestelle: Vec<ProbennahmestelleType>,
    #[xml(
        default,
        ns = b"xwas",
        name = b"nameBeauftragteUntersuchungsstelle",
        ty = "child"
    )]
    pub name_beauftragte_untersuchungsstelle: CodeUntersuchungsstelleType,
    #[xml(ns = b"xwas", name = b"probennehmer", ty = "child")]
    pub probennehmer: Vec<ProbennehmerType>,
    #[xml(
        default,
        ns = b"xwas",
        name = b"pruefberichtEnthaeltTeilergebnisse",
        ty = "child"
    )]
    #[serde(default)]
    pub pruefbericht_enthaelt_teilergebnisse: bool,
    #[xml(ns = b"xwas", name = b"korrekturvermerk", ty = "child")]
    pub korrekturvermerk: Option<String>,
    #[xml(
        default,
        ns = b"xwas",
        name = b"pruefberichtGemVorgabenAkkredition",
        ty = "child"
    )]
    pub pruefbericht_gem_vorgaben_akkredition: bool,
    #[xml(ns = b"xwas", name = b"titel", ty = "child")]
    pub titel: String,
    #[xml(ns = b"xwas", name = b"gesamtbewertung", ty = "child")]
    pub gesamtbewertung: CodeGesamtbewertungType,
    #[xml(ns = b"xwas", name = b"auffaelligkeiten", ty = "child")]
    #[serde(default)]
    pub auffaelligkeiten: Vec<String>,
    #[xml(
        default,
        ns = b"xwas",
        name = b"zeitpunktValidierungPruefbericht",
        ty = "child"
    )]
    pub zeitpunkt_validierung_pruefbericht: String, // TODO: Invent xs:dateTime
    #[xml(
        ns = b"xwas",
        name = b"fuerValidierungVerantwortlichePerson",
        ty = "child"
    )]
    #[serde(default)]
    pub fuer_validierung_verantwortliche_person: Vec<NatuerlichePersonType>,
    #[xml(
        default,
        ns = b"xwas",
        name = b"freigabeUebermittlungBetreiber",
        ty = "child"
    )]
    #[serde(default)]
    pub freigabe_uebermittlung_betreiber: bool,
    #[xml(ns = b"xwas", name = b"pruefberichtIDLabor", ty = "child")]
    pub pruefbericht_id_labor: String,
    #[xml(default, ns = b"xwas", name = b"swVersion", ty = "child")]
    pub sw_version: String,
    #[xml(default, ns = b"xwas", name = b"sprachePruefbericht", ty = "child")]
    pub sprache_pruefbericht: CodeAmtsspracheEuType,
    #[xml(ns = b"xwas", name = b"rechtlicherDisclaimer", ty = "child")]
    pub rechtlicher_disclaimer: String,
    #[xml(ns = b"xwas", name = b"zeitpunktUebermittlungAnSHAPTH", ty = "child")]
    pub zeitpunkt_uebermittlung_an_shapth: String, // TODO: Invent xs:dateTime
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
    #[xml(ns = b"xwas", name = b"auftraggeber", ty = "child")]
    pub auftraggeber: AuftraggeberType,
    // TODO: fix typo once fixed in XWasser
    #[xml(ns = b"xwas", name = b"zustaemdigeBehoerde", ty = "child")]
    #[serde(default)]
    pub zustaendige_behoerde: Vec<ZustaendigeBehoerdeType>,
    #[xml(
        default,
        ns = b"xwas",
        name = b"beauftragteUntersuchungsstelle",
        ty = "child"
    )]
    pub beauftragte_untersuchungsstelle: BeauftragteUntersuchungsstelleType,
    #[xml(ns = b"xwas", name = b"ortDerLabortaetigkeiten", ty = "child")]
    #[serde(default)]
    pub ort_der_labortaetigkeiten: Vec<AnschriftType>,
    #[xml(ns = b"xwas", name = b"anhang", ty = "child")]
    #[serde(default)]
    pub anhang: Vec<String>,
    #[xml(ns = b"xwas", name = b"erweiterung", ty = "child")]
    pub erweiterung: Option<ErweiterungType>,
    #[xml(name = b"id", ty = "attr")]
    pub id: String,
}
