#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

use crate::model::codes::deserialize_list_of_codes;
use crate::model::codes::deserialize_optional_code;
use crate::{
    TNS,
    model::codes::{
        CodeAnlassUntersuchungType, CodeArtEntnahmearmaturType,
        CodeAufbereitungsstoffDesinfektionsverfahrenType, CodeBewertungUntersuchungswertType,
        CodeKategorieProbennahmestelleType, CodeMediumType, CodeMesswertergaenzungType,
        CodeParameterauspraegungType, CodeProbenbewertungType, CodeProbenentnahmegeraetType,
        CodeProbengefaessType, CodeProbennahmeverfahrenType, CodeShapthParameterEinheitType,
        CodeShapthParameterType, CodeUnterkategorieProbennahmestelleType,
        CodeUntersuchungsverfahrenType,
    },
};
use xoev_xwasser_derive::XWasserValidate;

#[cfg(feature = "wasm")]
use tsify::Tsify;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

use super::{
    behoerde::ZustaendigeBehoerdeType,
    misc::{AngabenAlternativeIdGesundheitType, AngabenAlternativeIdUmweltType},
    organisation::OrganisationType,
    person::NatuerlichePersonType,
};

// TODO: implement Box<T>, Arc<T>, Rc<T> for raxb
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize)]
#[xml(tns(b"xwas", b"xwasser"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "t", content = "c")]
#[xml(tns(b"xwas", TNS))]
pub enum Probennehmer {
    #[xml(ns = b"xwas", name = b"organisation", ty = "child")]
    Organisation(OrganisationType),
    #[xml(ns = b"xwas", name = b"natuerlichePerson", ty = "child")]
    NatuerlichePerson(NatuerlichePersonType),
    // TODO: fix typo once it is fixed in XSD
    #[xml(ns = b"xwas", name = b"zustaendigeBehoerde", ty = "child")]
    ZustaendigeBehoerde(ZustaendigeBehoerdeType),
}

/// Klasse für den Transport von Informationen zu einem Probennehmer [Durch das Labor mit
/// dem Prüfbericht mit zu übermittelnde Informationen].
#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct ProbennehmerType {
    #[xml(ns = b"xwas", name = b"probennehmerID", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub probennehmer_id: Option<String>, // TODO: Invent UUID
    #[xml(ns = b"xwas", name = b"probennehmer", ty = "child")]
    pub probennehmer: Probennehmer,
    #[xml(ns = b"xwas", name = b"fremdsystemID_Probennehmer", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub fremdsystem_id_probennehmer: Option<String>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub kommentar: Option<String>,
    #[xml(name = b"id", ty = "attr")]
    pub id: String,
}

/// Klasse für den Transport von Informationen zu einer Probennahmestelle.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct ProbennahmestelleType {
    #[xml(ns = b"xwas", name = b"probennahmestelleID", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub probennahmestelle_id: Option<String>,
    #[xml(ns = b"xwas", name = b"objektID", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub objekt_id: Option<String>,
    #[xml(ns = b"xwas", name = b"nameProbennahmestelle", ty = "child")]
    pub name_probennahmestelle: String,
    #[xml(ns = b"xwas", name = b"kategorieProbennahmestelle", ty = "child")]
    pub kategorie_probennahmestelle: CodeKategorieProbennahmestelleType,
    #[xml(ns = b"xwas", name = b"unterkategorieProbennahmestelle", ty = "child")]
    #[serde(deserialize_with = "deserialize_optional_code")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub unterkategorie_probennahmestelle: Option<CodeUnterkategorieProbennahmestelleType>,
    #[xml(ns = b"xwas", name = b"artDerEntnahmearmatur", ty = "child")]
    #[serde(deserialize_with = "deserialize_optional_code")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub art_der_entnahmearmatur: Option<CodeArtEntnahmearmaturType>,
    #[xml(ns = b"xwas", name = b"stockwerkProbennahmestelle", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub stockwerk_probennahmestelle: Option<i16>,
    #[xml(ns = b"xwas", name = b"mediumAnDerProbennahmestelle", ty = "child")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_list_of_codes")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub medium_an_der_probennahmestelle: Vec<CodeMediumType>,
    #[xml(
        ns = b"xwas",
        name = b"desinfektionUndAufbereitungDesWassers",
        ty = "child"
    )]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_list_of_codes")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub desinfektion_und_aufbereitung_des_wassers:
        Vec<CodeAufbereitungsstoffDesinfektionsverfahrenType>,
    #[xml(ns = b"xwas", name = b"angabenAlternativeIDGesundheit", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub angaben_alternative_id_gesundheit: Option<AngabenAlternativeIdGesundheitType>,
    #[xml(ns = b"xwas", name = b"angabenAlternativeIDUmwelt", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub angaben_alternative_id_umwelt: Option<AngabenAlternativeIdUmweltType>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub kommentar: Option<String>,
    #[xml(name = b"id", ty = "attr")]
    pub id: String,
}

/// Klasse zum Transport von Informationen, welche über eine Probe vorliegen sollen, die
/// im Rahmen eines Prüfberichts via SHAPTH übermittelt wird.
#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct ProbeType {
    #[xml(ns = b"xwas", name = b"probeID", ty = "child")]
    pub probe_id: String,
    #[xml(ns = b"xwas", name = b"probennahmestelle", ty = "child")]
    pub probennahmestelle: String,
    #[xml(ns = b"xwas", name = b"terminplanID", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub terminplan_id: Option<String>,
    #[xml(ns = b"xwas", name = b"probennehmer", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub probennehmer: Option<String>,
    #[xml(ns = b"xwas", name = b"titelProbe", ty = "child")]
    pub titel_probe: String,
    #[xml(ns = b"xwas", name = b"analyseergebnisParameter", ty = "child")]
    pub analyseergebnis_parameter: Vec<AnalyseergebnisParameterType>,
    #[xml(ns = b"xwas", name = b"anlassDerUntersuchung", ty = "child")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_list_of_codes")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub anlass_der_untersuchung: Vec<CodeAnlassUntersuchungType>,
    #[xml(ns = b"xwas", name = b"medium", ty = "child")]
    #[serde(deserialize_with = "deserialize_optional_code")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub medium: Option<CodeMediumType>,
    #[xml(
        ns = b"xwas",
        name = b"akkreditierteDurchfuehrungDerProbennahme",
        ty = "child"
    )]
    pub akkreditierte_durchfuehrung_der_probennahme: bool,
    #[xml(ns = b"xwas", name = b"ergaenzungZumMedium", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub ergaenzung_zum_medium: Option<String>,
    #[xml(default, ns = b"xwas", name = b"zeitpunktProbennahme", ty = "child")]
    pub zeitpunkt_probennahme: String, // TODO: Invent xs:dateTime
    #[xml(ns = b"xwas", name = b"probennahmeverfahren", ty = "child")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_list_of_codes")]
    pub probennahmeverfahren: Vec<CodeProbennahmeverfahrenType>,
    #[xml(ns = b"xwas", name = b"probenentnahmegeraet", ty = "child")]
    #[serde(deserialize_with = "deserialize_optional_code")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub probenentnahmegeraet: Option<CodeProbenentnahmegeraetType>,
    #[xml(ns = b"xwas", name = b"probengefaess", ty = "child")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_list_of_codes")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub probengefaess: Vec<CodeProbengefaessType>,
    #[xml(
        ns = b"xwas",
        name = b"ergaenzendeInformationenZuProbenentnahmegeraet",
        ty = "child"
    )]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub ergaenzende_informationen_zu_probenentnahmegeraet: Option<String>,
    #[xml(
        ns = b"xwas",
        name = b"desinfektionProbenentnahmegeraetDurchgefuehrt",
        ty = "child"
    )]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub desinfektion_probenentnahmegeraet_durchgefuehrt: Option<bool>,
    #[xml(ns = b"xwas", name = b"konservierungDerProbe", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub konservierung_der_probe: Vec<String>,
    #[xml(ns = b"xwas", name = b"kommentarZurProbennahme", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub kommentar_zur_probennahme: Option<String>,
    #[xml(ns = b"xwas", name = b"informationenZumProbentransport", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub informationen_zum_probentransport: Option<String>,
    #[xml(
        ns = b"xwas",
        name = b"eingangProbeBeiUntersuchungsstelle",
        ty = "child"
    )]
    pub eingang_probe_bei_untersuchungsstelle: String, // TODO: Invent xs:dateTime
    #[xml(ns = b"xwas", name = b"beginnLabortaetigkeitAnalytik", ty = "child")]
    pub beginn_labortaetigkeit_analytik: String, // TODO: Invent xs:dateTime
    #[xml(ns = b"xwas", name = b"abschlussLabortaetigkeitAnalytik", ty = "child")]
    pub abschluss_labortaetigkeit_analytik: String,
    #[xml(ns = b"xwas", name = b"konformitaetsbewertungDerProbe", ty = "child")]
    pub konformitaetsbewertung_der_probe: CodeProbenbewertungType,
    #[xml(ns = b"xwas", name = b"berichtspflichtig", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub berichtspflichtig: Option<bool>,
    #[xml(ns = b"xwas", name = b"vonProbennehmerVergebeneProbeID", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub von_probennehmer_vergebene_probe_id: Option<String>,
    #[xml(ns = b"xwas", name = b"angelieferteProbe", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub angelieferte_probe: Option<bool>,
    #[xml(
        ns = b"xwas",
        name = b"informationenZurAngeliefertenProbe",
        ty = "child"
    )]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub informationen_zur_angelieferten_probe: Option<String>,
    #[xml(ns = b"xwas", name = b"probeID_ausLabor", ty = "child")]
    pub probe_id_aus_labor: String,
    #[xml(ns = b"xwas", name = b"anhang", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub anhang: Vec<String>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub kommentar: Option<String>,
    #[xml(name = b"id", ty = "attr")]
    pub id: String,
}

#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct AnalyseergebnisParameterType {
    #[xml(ns = b"xwas", name = b"analyseergebnisParameterID", ty = "child")]
    pub analyseergebnis_parameter_id: String, // TODO: Invent UUID
    #[xml(ns = b"xwas", name = b"anschriftID", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub anschrift_id: Option<String>,
    #[xml(ns = b"xwas", name = b"zugelasseneUntersuchungsstelle", ty = "child")]
    pub zugelassene_untersuchungsstelle: String,
    #[xml(
        ns = b"xwas",
        name = b"akkreditierteDurchfuehrungAnalyse",
        ty = "child"
    )]
    pub akkreditierte_durchfuehrung_analyse: bool,
    #[xml(ns = b"xwas", name = b"zugelasseneDurchfuehrungAnalyse", ty = "child")]
    #[serde(default)]
    pub zugelassene_durchfuehrung_analyse: bool,
    #[xml(ns = b"xwas", name = b"untersuchungsverfahren", ty = "child")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_list_of_codes")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub untersuchungsverfahren: Vec<CodeUntersuchungsverfahrenType>,
    #[xml(
        ns = b"xwas",
        name = b"ergaenzungZumUntersuchungsverfahren",
        ty = "child"
    )]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub ergaenzung_zum_untersuchungsverfahren: Option<String>,
    #[xml(default, ns = b"xwas", name = b"untersuchterParameter", ty = "child")]
    pub untersuchter_parameter: CodeShapthParameterType,
    #[xml(ns = b"xwas", name = b"parameterauspraegung", ty = "child")]
    #[serde(deserialize_with = "deserialize_optional_code")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub parameterauspraegung: Option<CodeParameterauspraegungType>,
    #[xml(
        ns = b"xwas",
        name = b"parameterDurchBetreiberUntersucht",
        ty = "child"
    )]
    pub parameter_durch_betreiber_untersucht: bool,
    #[xml(ns = b"xwas", name = b"wurdeDerParameterKorrigiert", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub wurde_der_parameter_korrigiert: Option<bool>,
    #[xml(ns = b"xwas", name = b"untersuchungswertParameter", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub untersuchungswert_parameter: Option<f64>,
    #[xml(ns = b"xwas", name = b"einheitDesUntersuchungswerts", ty = "child")]
    #[serde(deserialize_with = "deserialize_optional_code")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub einheit_des_untersuchungswerts: Option<CodeShapthParameterEinheitType>,
    #[xml(
        ns = b"xwas",
        name = b"ergaenzungZumUntersuchungswertParameter",
        ty = "child"
    )]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_code")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub ergaenzung_zum_untersuchungswert_parameter: Option<CodeMesswertergaenzungType>,
    #[xml(ns = b"xwas", name = b"parameterwertErgaenzung", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub parameterwert_ergaenzung: Option<String>,
    #[xml(ns = b"xwas", name = b"ausgewertetesAnsatzvolumen", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub ausgewertetes_ansatzvolumen: Option<f64>,
    #[xml(ns = b"xwas", name = b"verknuepfteParameter", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub verknuepfte_parameter: Vec<String>,
    #[xml(
        default,
        ns = b"xwas",
        name = b"bewertungUntersuchungswert",
        ty = "child"
    )]
    pub bewertung_untersuchungswert: CodeBewertungUntersuchungswertType,
    #[xml(ns = b"xwas", name = b"parameterauffaelligkeit", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub parameterauffaelligkeit: Option<String>,
    #[xml(
        ns = b"xwas",
        name = b"messunsicherheitUntersuchungswertAbsolut",
        ty = "child"
    )]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub messunsicherheit_untersuchungswert_absolut: Option<f64>,
    #[xml(
        ns = b"xwas",
        name = b"messunsicherheitUntersuchungswertRelativ",
        ty = "child"
    )]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub messunsicherheit_untersuchungswert_relativ: Option<f64>,
    #[xml(ns = b"xwas", name = b"bestimmungsgrenzeLoQ", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub bestimmungsgrenze_lo_q: Option<f64>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub kommentar: Option<String>,
    #[xml(name = b"id", ty = "attr")]
    pub id: String,
}
