#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

use crate::model::codes::{
    CodeAnlassUntersuchungType, CodeArtProbennahmestelleType,
    CodeAufbereitungsstoffDesinfektionsverfahrenType, CodeBewertungUntersuchungswertType,
    CodeMediumType, CodeMesswertergaenzungType, CodeParameterauspraegungType,
    CodeParameterunterauswahlType, CodeProbenbewertungType, CodeProbenentnahmegeraetType,
    CodeProbengefaessType, CodeProbennahmeverfahrenType, CodeShapthParameterEinheitType,
    CodeShapthParameterType, CodeUntersuchungsverfahrenType,
};

#[cfg(feature = "wasm")]
use tsify::Tsify;

use super::{
    behoerde::ZustaendigeBehoerdeType, organisation::OrganisationType,
    person::NatuerlichePersonType,
};

// TODO: implement Box<T>, Arc<T>, Rc<T> for raxb
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[xml(tns(b"xwas", b"xwasser"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[serde(tag = "t", content = "c")]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_5_2/"
))]
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
#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_5_2/"
))]
pub struct ProbennehmerType {
    #[xml(ns = b"xwas", name = b"probennehmerID", ty = "child")]
    pub probennehmer_id: String, // TODO: Invent UUID
    #[xml(ns = b"xwas", name = b"probennehmer", ty = "child")]
    pub probennehmer: Probennehmer,
    #[xml(ns = b"xwas", name = b"fremdsystemID_Probennehmer", ty = "child")]
    pub fremdsystem_id_probennehmer: Option<String>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
    #[xml(name = b"id", ty = "attr")]
    pub id: String,
}

/// Klasse für den Transport von Informationen zu einer Probennahmestelle.
#[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_5_2/"
))]
pub struct ProbennahmestelleType {
    #[xml(ns = b"xwas", name = b"probennahmestelleID", ty = "child")]
    pub probennahmestelle_id: String,
    #[xml(ns = b"xwas", name = b"objektID", ty = "child")]
    pub objekt_id: String,
    #[xml(ns = b"xwas", name = b"probe", ty = "child")]
    #[serde(default)]
    pub probe: Vec<ProbeType>,
    #[xml(ns = b"xwas", name = b"terminplanID", ty = "child")]
    #[serde(default)]
    pub terminplan_id: Vec<String>,
    #[xml(ns = b"xwas", name = b"nameProbennahmestelle", ty = "child")]
    pub name_probennahmestelle: String,
    #[xml(ns = b"xwas", name = b"artProbennahmestelle", ty = "child")]
    pub art_probennahmestelle: CodeArtProbennahmestelleType,
    #[xml(ns = b"xwas", name = b"stockwerkProbennahmestelle", ty = "child")]
    pub stockwerk_probennahmestelle: Option<u64>,
    #[xml(ns = b"xwas", name = b"mediumAnDerProbennahmestelle", ty = "child")]
    #[serde(default)]
    pub medium_an_der_probennahmestelle: Vec<CodeMediumType>,
    #[xml(
        ns = b"xwas",
        name = b"desinfektionUndAufbereitungDesWassers",
        ty = "child"
    )]
    pub desinfektion_und_aufbereitung_des_wassers:
        Vec<CodeAufbereitungsstoffDesinfektionsverfahrenType>,
    #[xml(ns = b"xwas", name = b"altID", ty = "child")]
    pub alt_id: Option<String>,
    #[xml(ns = b"xwas", name = b"berichtspflichtig", ty = "child")]
    pub berichtspflichtig: Option<bool>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
    #[xml(name = b"id", ty = "attr")]
    pub id: String,
}

/// Klasse zum Transport von Informationen, welche über eine Probe vorliegen sollen, die
/// im Rahmen eines Prüfberichts via SHAPTH übermittelt wird.
#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_5_2/"
))]
pub struct ProbeType {
    #[xml(ns = b"xwas", name = b"probeID", ty = "child")]
    pub probe_id: String,
    #[xml(ns = b"xwas", name = b"untersuchungsplanID", ty = "child")]
    pub untersuchungsplan_id: Option<String>,
    #[xml(ns = b"xwas", name = b"probennehmer", ty = "child")]
    pub probennehmer: String,
    #[xml(ns = b"xwas", name = b"analyseergebnisParameter", ty = "child")]
    pub analyseergebnis_parameter: Vec<AnalyseergebnisParameterType>,
    #[xml(ns = b"xwas", name = b"anlassDerUntersuchung", ty = "child")]
    #[serde(default)]
    pub anlass_der_untersuchung: Vec<CodeAnlassUntersuchungType>,
    #[xml(ns = b"xwas", name = b"medium", ty = "child")]
    pub medium: Option<CodeMediumType>,
    #[xml(
        ns = b"xwas",
        name = b"akkreditierteDurchfuehrungDerProbennahme",
        ty = "child"
    )]
    pub akkreditierte_durchfuehrung_der_probennahme: bool,
    #[xml(ns = b"xwas", name = b"ergaenzungZumMedium", ty = "child")]
    pub ergaenzung_zum_medium: Option<String>,
    #[xml(default, ns = b"xwas", name = b"zeitpunktProbennahme", ty = "child")]
    pub zeitpunkt_probennahme: String, // TODO: Invent xs:dateTime
    #[xml(ns = b"xwas", name = b"probennahmeverfahren", ty = "child")]
    #[serde(default)]
    pub probennahmeverfahren: Vec<CodeProbennahmeverfahrenType>,
    #[xml(ns = b"xwas", name = b"probenentnahmegeraet", ty = "child")]
    pub probenentnahmegeraet: Option<CodeProbenentnahmegeraetType>,
    #[xml(ns = b"xwas", name = b"probengefaess", ty = "child")]
    pub probengefaess: Option<CodeProbengefaessType>,
    #[xml(
        ns = b"xwas",
        name = b"ergaenzendeInformationenZuProbenentnahmegeraet",
        ty = "child"
    )]
    pub ergaenzende_informationen_zu_probenentnahmegeraet: Option<String>,
    #[xml(
        ns = b"xwas",
        name = b"desinfektionProbenentnahmegeraetDurchgefuehrt",
        ty = "child"
    )]
    pub desinfektion_probenentnahmegeraet_durchgefuehrt: Option<bool>,
    #[xml(
        ns = b"xwas",
        name = b"konservierungAufbereitungDesinfektionProbe",
        ty = "child"
    )]
    #[serde(default)]
    pub konservierung_aufbereitung_desinfektion_probe:
        Vec<CodeAufbereitungsstoffDesinfektionsverfahrenType>,
    #[xml(ns = b"xwas", name = b"kommentarZurProbennahme", ty = "child")]
    pub kommentar_zur_probennahme: String,
    #[xml(ns = b"xwas", name = b"informationenZumProbentransport", ty = "child")]
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
    pub berichtspflichtig: Option<bool>,
    #[xml(ns = b"xwas", name = b"vonProbennehmerVergebeneProbeID", ty = "child")]
    pub von_probennehmer_vergebene_probe_id: String,
    #[xml(ns = b"xwas", name = b"probeID_ausLabor", ty = "child")]
    pub probe_id_aus_labor: String,
    #[xml(ns = b"xwas", name = b"anhang", ty = "child")]
    #[serde(default)]
    pub anhang: Vec<String>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
    #[xml(name = b"id", ty = "attr")]
    pub id: String,
}

#[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_5_2/"
))]
pub struct AnalyseergebnisParameterType {
    #[xml(ns = b"xwas", name = b"analyseergebnisParameterID", ty = "child")]
    pub analyseergebnis_parameter_id: String, // TODO: Invent UUID
    #[xml(ns = b"xwas", name = b"anschriftID", ty = "child")]
    pub anschrift_id: String,
    #[xml(ns = b"xwas", name = b"zugelasseneUntersuchungsstelle", ty = "child")]
    pub zugelassene_untersuchungsstelle: String,
    #[xml(
        ns = b"xwas",
        name = b"akkreditierteDurchfuehrungAnalyse",
        ty = "child"
    )]
    pub akkreditierte_durchfuehrung_analyse: bool,
    #[xml(ns = b"xwas", name = b"untersuchungsverfahren", ty = "child")]
    #[serde(default)]
    pub untersuchungsverfahren: Vec<CodeUntersuchungsverfahrenType>,
    #[xml(
        ns = b"xwas",
        name = b"ergaenzungZumUntersuchungsverfahren",
        ty = "child"
    )]
    pub ergaenzung_zum_untersuchungsverfahren: Option<String>,
    #[xml(default, ns = b"xwas", name = b"untersuchterParameter", ty = "child")]
    pub untersuchter_parameter: CodeShapthParameterType,
    #[xml(ns = b"xwas", name = b"parameterauspraegung", ty = "child")]
    pub parameterauspraegung: Option<CodeParameterauspraegungType>,
    #[xml(
        ns = b"xwas",
        name = b"parameterDurchBetreiberUntersucht",
        ty = "child"
    )]
    pub parameter_durch_betreiber_untersucht: bool,
    #[xml(ns = b"xwas", name = b"wurdeDerParameterKorrigiert", ty = "child")]
    pub wurde_der_parameter_korrigiert: Option<bool>,
    #[xml(ns = b"xwas", name = b"parameterUnterauswahl", ty = "child")]
    pub parameter_unterauswahl: Option<CodeParameterunterauswahlType>,
    #[xml(ns = b"xwas", name = b"untersuchungswertParameter", ty = "child")]
    pub untersuchungswert_parameter: Option<f64>,
    #[xml(ns = b"xwas", name = b"einheitDesUntersuchungswerts", ty = "child")]
    pub einheit_des_untersuchungswerts: Option<CodeShapthParameterEinheitType>,
    #[xml(
        ns = b"xwas",
        name = b"ergaenzungZumUntersuchungswertParameter",
        ty = "child"
    )]
    pub ergaenzung_zum_untersuchungswert_parameter: Option<CodeMesswertergaenzungType>,
    #[xml(ns = b"xwas", name = b"parameterwertErgaenzung", ty = "child")]
    pub parameterwert_ergaenzung: Option<String>,
    #[xml(ns = b"xwas", name = b"ausgewertetesAnsatzvolumen", ty = "child")]
    pub ausgewertetes_ansatzvolumen: Option<f64>,
    #[xml(ns = b"xwas", name = b"shapthParameterNummer", ty = "child")]
    #[serde(default)]
    pub shapth_parameter_nummer: Vec<i64>,
    #[xml(
        default,
        ns = b"xwas",
        name = b"bewertungUntersuchungswert",
        ty = "child"
    )]
    pub bewertung_untersuchungswert: CodeBewertungUntersuchungswertType,
    #[xml(ns = b"xwas", name = b"parameterauffaelligkeit", ty = "child")]
    pub parameterauffaelligkeit: Option<String>,
    #[xml(
        ns = b"xwas",
        name = b"messunsicherheitUntersuchungswertAbsolut",
        ty = "child"
    )]
    pub messunsicherheit_untersuchungswert_absolut: Option<f64>,
    #[xml(
        ns = b"xwas",
        name = b"messunsicherheitUntersuchungswertRelativ",
        ty = "child"
    )]
    pub messunsicherheit_untersuchungswert_relativ: Option<f64>,
    #[xml(ns = b"xwas", name = b"bestimmungsgrenzeLoQ", ty = "child")]
    pub bestimmungsgrenze_lo_q: Option<String>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
    #[xml(name = b"id", ty = "attr")]
    pub id: String,
}
