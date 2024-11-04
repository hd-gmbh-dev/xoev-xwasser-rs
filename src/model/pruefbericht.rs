#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify_next::Tsify;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

use super::{
    codes::{CodeAmtsspracheEuType, CodeGesamtbewertungType, CodeUntersuchungsstelleType},
    shared::{
        anschrift::AnschriftType,
        auftraggeber::AuftraggeberType,
        behoerde::ZustaendigeBehoerdeType,
        misc::ErweiterungType,
        person::NatuerlichePersonType,
        probe::{ProbeType, ProbennahmestelleType, ProbennehmerType},
        unterssuchungsstelle::{
            BeauftragteUntersuchungsstelleType, ZugelasseneUntersuchungsstelleType,
        },
    },
};

/// Klasse zur Erfassung bzw. zum Transport der Daten eines Prüfberichts. Prüfberichte
/// werden erstellt, nachdem eine Wasserprobe im Labor analysiert wurde.
#[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_7_0/"
))]
pub struct PruefberichtType {
    #[xml(ns = b"xwas", name = b"pruefberichtUUID", ty = "child")]
    pub pruefbericht_uuid: String,
    #[xml(ns = b"xwas", name = b"vorgaengerPruefberichtID", ty = "child")]
    pub vorgaenger_pruefbericht_id: Option<String>,
    #[xml(ns = b"xwas", name = b"auftragsnummer", ty = "child")]
    pub auftragsnummer: String,
    #[xml(ns = b"xwas", name = b"probennahmestelle", ty = "child")]
    #[serde(default)]
    pub probennahmestelle: Vec<ProbennahmestelleType>,
    #[xml(ns = b"xwas", name = b"probe", ty = "child")]
    #[serde(default)]
    pub probe: Vec<ProbeType>,
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
    pub zeitpunkt_uebermittlung_an_shapth: Option<String>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
    #[xml(ns = b"xwas", name = b"auftraggeber", ty = "child")]
    pub auftraggeber: AuftraggeberType,
    #[xml(ns = b"xwas", name = b"zustaendigeBehoerde", ty = "child")]
    #[serde(default)]
    pub zustaendige_behoerde: Vec<ZustaendigeBehoerdeType>,
    #[xml(
        default,
        ns = b"xwas",
        name = b"beauftragteUntersuchungsstelle",
        ty = "child"
    )]
    pub beauftragte_untersuchungsstelle: BeauftragteUntersuchungsstelleType,
    #[xml(ns = b"xwas", name = b"zugelasseneUntersuchungsstelle", ty = "child")]
    #[serde(default)]
    pub zugelassene_untersuchungsstelle: Vec<ZugelasseneUntersuchungsstelleType>,
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
