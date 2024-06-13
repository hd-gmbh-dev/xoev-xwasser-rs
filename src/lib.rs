#![allow(non_snake_case, dead_code)]

use raxb::{value::ConstStr, XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct Code {
    #[serde(default)]
    #[xml(name = b"listURI", ty = "attr")]
    pub list_uri: Option<String>,
    #[serde(default)]
    #[xml(name = b"listVersionID", ty = "attr")]
    pub list_version_id: Option<String>,
    #[xml(ty = "text", default)]
    pub code: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"identifikation.nachricht")]
pub struct NachrichtenTyp {
    #[xml(
        name = b"listURI",
        ty = "attr",
        value = "urn:xoev-de:xwasser:codeliste:nachrichtentyp"
    )]
    #[serde(skip)]
    pub _list_uri: ConstStr,
    #[xml(name = b"listVersionID", ty = "attr", value = "1")]
    #[serde(skip)]
    pub _list_version_id: ConstStr,
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"identifikation.nachricht")]
pub struct IdentifikationNachricht {
    #[xml(name = b"nachrichtenUUID", ty = "child")]
    pub nachrichten_uuid: NachrichtenUUID,
    #[xml(name = b"nachrichtentyp", ty = "child")]
    pub nachrichten_typ: NachrichtenTyp,
    #[xml(name = b"erstellungszeitpunkt", ty = "child")]
    pub erstellungszeitpunkt: Erstellungszeitpunkt,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"erstellungszeitpunkt")]
pub struct Erstellungszeitpunkt {
    #[xml(name = b"erstellungszeitpunkt", ty = "child")]
    pub erstellungszeitpunkt: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"nachrichtenUUID")]
pub struct NachrichtenUUID {
    #[xml(name = b"nachrichtenUUID", ty = "child")]
    pub nachrichten_uuid: String, 
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"verzeichnisdienst")]
pub struct Verzeichnisdienst {
    #[xml(name = b"listVersionID", ty = "attr", value = "")]
    #[serde(skip)]
    pub _list_version_id: ConstStr,
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"leser")]
pub struct Leser {
    #[xml(name = b"verzeichnisdienst", ty = "child")]
    pub verzeichnisdienst: Verzeichnisdienst,
    #[xml(name = b"kennung", ty = "child")]
    pub kennung: Kennung,
    #[xml(name = b"name", ty = "child")]
    pub name: String,
    #[xml(name = b"erreichbarkeit" , ty = "child")]
    pub erreichbarkeit: Erreichbarkeit,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"erreichbarkeit")]
pub struct Erreichbarkeit { 
    #[xml(name = b"kanal", ty = "child")]
    pub kanal: Kanal,
    #[xml(name = b"kennung", ty = "child")]
    pub kennung: Kennung,
    #[xml(name = b"zusatz", ty = "child")]
    pub zusatz: Zusatz,
}


#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"zusatz")]
pub struct Zusatz {
    #[xml(name = b"zusatz", ty = "child")] 
    pub zusatz: String,
}


#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"kanal")]
pub struct Kanal { 
    #[xml(name = b"listVersionID", ty = "attr", value = "1")]
    #[serde(skip)]
    pub _list_version_id: ConstStr,
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"kennung")]
pub struct Kennung { 
    #[xml(name = b"kennung", ty = "child")]
    pub kennung: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"autor")]
pub struct Autor {
    #[xml(name = b"verzeichnisdienst", ty = "child")]
    pub verzeichnisdienst: Verzeichnisdienst,
    #[xml(name = b"kennung", ty = "child")]
    pub kennung: Kennung,
    #[xml(name = b"name", ty = "child")]
    pub name: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"nachrichtenkopf.g2g")]
pub struct NachrichtenkopfG2g {
    #[xml(name = b"identifikation.nachricht", ty = "child")]
    pub identifikation_nachricht: IdentifikationNachricht,
    #[xml(name = b"leser", ty = "child")]
    pub leser: Leser,
    #[xml(name = b"autor", ty = "child")]
    pub autor: Autor,
    #[xml(name = b"referenzUUID", ty = "child")]
    pub referenz_uuid: ReferenzUUID,
    #[xml(name = b"dvdvDienstkennung", ty = "child")]
    pub dvdv_dienstkennung: DvdvDienstkennung,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"DvdvDienstkennung")]
pub struct DvdvDienstkennung {
    #[xml(name = b"dvdvDienstkennung", ty = "child")]
    pub dvdv_dienstkennung: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"ReferenzUUID")]
pub struct ReferenzUUID {
    #[xml(name = b"referenzUUID", ty = "child")]
    pub referenz_uuid: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"identifikationVorgang")]
#[xml(tns(b"xwas", b"xwasser"))]
pub struct IdentifikationVorgang {
    #[xml(ns = b"xwas", name = b"vorgangsID", ty = "child")]
    pub vorgangs_id: String,
    #[xml(ns = b"xwas", name = b"aktenzeichen", ty = "child")]
    pub aktenzeichen: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct Untersuchungsverfahren {
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    #[xml(name = b"name", ty = "child")]
    pub name: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct UntersuchterParameter {
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    #[xml(name = b"name", ty = "child")]
    pub name: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct BewertungUntersuchungswert {
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    #[xml(name = b"name", ty = "child")]
    pub name: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"analyseergebnisParameter")]
#[xml(tns(b"xwas", b"xwasser"))]
pub struct AnalyseergebnisParameter {
    #[xml(ns = b"xwas", name = b"analyseergebnisParameterID", ty = "child")]
    pub analyseergebnis_parameter_id: String,
    #[xml(ns = b"xwas", name = b"probeID", ty = "child")]
    pub probe_id: String,
    #[xml(ns = b"xwas", name = b"zugelasseneUntersuchungsstelleID", ty = "child")]
    pub zugelassene_untersuchungsstelle_id: String,
    #[xml(ns = b"xwas", name = b"anschriftID", ty = "child")]
    pub anschrift_id: String,
    #[xml(ns = b"xwas", name = b"analyseImRahmenDerAkkreditierung", ty = "child")]
    pub analyse_im_rahmen_der_akkreditierung: bool,
    #[xml(ns = b"xwas", name = b"untersuchungsverfahren", ty = "child")]
    pub untersuchungsverfahren: Untersuchungsverfahren,
    #[xml(ns = b"xwas", name = b"ergaenzungZumUntersuchungsverfahren", ty = "child")]
    pub ergaenzung_zum_untersuchungsverfahren: String,
    #[xml(ns = b"xwas", name = b"untersuchterParameter", ty = "child")]
    pub untersuchter_parameter: UntersuchterParameter,
    #[xml(ns = b"xwas", name = b"parameterauspraegung", ty = "child")]
    pub parameterauspraegung: Parameterauspraegung,
    #[xml(ns = b"xwas", name = b"parameterUnterauswahl", ty = "child")]
    pub parameter_unterauswahl: ParameterUnterauswahl,
    #[xml(ns = b"xwas", name = b"sensorischerParameterIstAnnehmbar", ty = "child")]
    pub sensorischer_parameter_ist_annehmbar: bool,
    #[xml(ns = b"xwas", name = b"untersuchungswertParameter", ty = "child")]
    pub untersuchungswert_parameter: String,  
    #[xml(ns = b"xwas", name = b"einheitDesUntersuchungswerts", ty = "child")]
    pub einheit_des_untersuchungswerts: EinheitDesUntersuchungswerts,
    #[xml(ns = b"xwas", name = b"ergaenzungZumUntersuchungswertParameter", ty = "child")]
    pub ergaenzung_zum_untersuchungswert_parameter: ErgaenzungZumUntersuchungswertParameter,
    #[xml(ns = b"xwas", name = b"parameterwertErgaenzung", ty = "child")]
    pub parameterwert_ergaenzung: String,
    #[xml(ns = b"xwas", name = b"ausgewertetesAnsatzvolumen", ty = "child")]
    pub ausgewertetes_ansatzvolumen: String,
    #[xml(ns = b"xwas", name = b"shapthParameterNummer", ty = "child")]
    pub shapth_parameter_nummer: String,
    #[xml(ns = b"xwas", name = b"bewertungUntersuchungswert", ty = "child")]
    pub bewertung_untersuchungswert: BewertungUntersuchungswert,
    #[xml(ns = b"xwas", name = b"parameterauffaelligkeit", ty = "child")]
    pub parameterauffaelligkeit: String,
    #[xml(ns = b"xwas", name = b"messunsicherheitUntersuchungswert", ty = "child")]
    pub messunsicherheitUntersuchungswert: String, // not null
    #[xml(ns = b"xwas", name = b"bestimmungsgrenzeLoQ", ty = "child")]
    pub bestimmungsgrenzeLoQ: String, // erfüllung von anderem feld abhängig 
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct ErgaenzungZumUntersuchungswertParameter {
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    #[xml(name = b"name", ty = "child")]
    pub name: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct EinheitDesUntersuchungswerts {
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    #[xml(name = b"name", ty = "child")]
    pub name: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct ParameterUnterauswahl {
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    #[xml(name = b"name", ty = "child")]
    pub name: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct Parameterauspraegung {
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    #[xml(name = b"name", ty = "child")]
    pub name: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"probe")]
#[xml(tns(b"xwas", b"xwasser"))]
pub struct Probe {
    #[xml(ns = b"xwas", name = b"probeID", ty = "child")]
    pub probe_id: String,
    #[xml(ns = b"xwas", name = b"probennahmestelleID", ty = "child")]
    pub probennahmestelle_id: String,
    #[xml(ns = b"xwas", name = b"untersuchungsplanID", ty = "child")]
    pub untersuchungsplan_id: String,
    #[xml(ns = b"xwas", name = b"analyseergebnisParameterID", ty = "child")]
    pub analyseergebnis_parameter_id: String,
    #[xml(ns = b"xwas", name = b"analyseergebnisParameter", ty = "child")]
    pub analyseergebnis_parameter: AnalyseergebnisParameter,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"probennahmestelle")]
#[xml(tns(b"xwas", b"xwasser"))]
pub struct Probennahmestelle {
    #[xml(ns = b"xwas", name = b"probennahmestelleID", ty = "child")]
    pub probennahmestelle_id: String,
    #[xml(ns = b"xwas", name = b"objektID", ty = "child")]
    pub objekt_id: String,
    #[xml(ns = b"xwas", name = b"probe", ty = "child")]
    pub probe: Vec<Probe>,
    #[xml(ns = b"xwas", name = b"nameProbennahmestelle", ty = "child")]
    pub name_probennahmestelle: String,
    #[xml(ns = b"xwas", name = b"artProbennahmestelle", ty = "child")]
    pub art_probennahmestelle: String,
    #[xml(ns = b"xwas", name = b"mediumAnDerProbennahmestelle", ty = "child")]
    pub medium_an_der_probennahmestelle: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"pruefbericht")]
#[xml(tns(b"xwas", b"xwasser"))]
pub struct Pruefbericht {
    #[xml(ns = b"xwas", name = b"pruefberichtID", ty = "child")]
    pub pruefbericht_id: String,
    #[xml(ns = b"xwas", name = b"probennahmestelle", ty = "child")]
    pub probennahmestelle: Vec<Probennahmestelle>,
    #[xml(
        ns = b"xwas",
        name = b"nameBeauftragteUntersuchungsstelle",
        ty = "child"
    )]
    pub name_beauftragte_untersuchungsstelle: String,
    #[xml(
        ns = b"xwas",
        name = b"pruefgerichtGemVorgabenAkkredition",
        ty = "child"
    )]
    pub pruefgericht_gem_vorgaben_akkredition: String,
    #[xml(ns = b"xwas", name = b"titel", ty = "child")]
    pub titel: String,
    #[xml(ns = b"xwas", name = b"gesamtbewertung", ty = "child")]
    pub gesamtbewertung: String,
    #[xml(ns = b"xwas", name = b"auffaelligkeiten", ty = "child")]
    pub auffaelligkeiten: String,
    #[xml(ns = b"xwas", name = b"zeitpunktValidierungPruefbericht", ty = "child")]
    pub zeitpunkt_validierung_pruefbericht: String,
    #[xml(
        ns = b"xwas",
        name = b"fuerValidierungVerantwortlichePerson",
        ty = "child"
    )]
    pub fuer_validierung_verantwortliche_person: String,
    #[xml(ns = b"xwas", name = b"pruefberichtIDLabor", ty = "child")]
    pub pruefbericht_id_labor: String,
    #[xml(ns = b"xwas", name = b"swVersion", ty = "child")]
    pub sw_version: String,
    #[xml(ns = b"xwas", name = b"sprachePruefbericht", ty = "child")]
    pub sprache_pruefbericht: String,
    #[xml(ns = b"xwas", name = b"rechtlicherDisclaimer", ty = "child")]
    pub rechtlicher_disclaimer: String,
    #[xml(ns = b"xwas", name = b"zeitpunktUebermittlungAnSHAPTH", ty = "child")]
    pub zeitpunkt_uebermittlung_an_shapth: String,
    #[xml(ns = b"xwas", name = b"auftraggeber_Rel", ty = "child")]
    pub auftraggeber_rel: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"vorgangType")]
#[xml(tns(b"xwas", b"xwasser"))]
pub struct VorgangType {
    #[xml(ns = b"xwas", name = b"pruefbericht", ty = "child")]
    pub pruefbericht: Pruefbericht,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"vorgang")]
#[xml(tns(b"xwas", b"xwasser"))]
pub struct Vorgang {
    #[xml(ns = b"xwas", name = b"identifikationVorgang", ty = "child")]
    pub identifikation_vorgang: IdentifikationVorgang,
    #[xml(ns = b"xwas", name = b"vorgangType", ty = "child")]
    pub vorgang_type: VorgangType,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[xml(root = b"vorgang.transportieren.2010")]
#[xml(tns(b"xwas", b"xwasser"))]
pub struct QualityReport {
    #[xml(
        ns = b"xmlns",
        name = b"xsi",
        ty = "attr",
        value = "http://www.w3.org/2001/XMLSchema-instance"
    )]
    #[serde(skip)]
    _xmlns_xsi: ConstStr,
    #[xml(
        ns = b"xsi",
        name = b"schemaLocation",
        ty = "attr",
        value = "xwasser xwasser.xsd"
    )]
    #[serde(skip)]
    schema_location: ConstStr,
    #[xml(ns = b"xmlns", name = b"xwas", ty = "attr", value = "xwasser")]
    #[serde(skip)]
    _xmlns: ConstStr,
    #[xml(name = b"produkt", ty = "attr")]
    pub produkt: String,
    #[xml(name = b"produkthersteller", ty = "attr", value = "H & D GmbH")]
    #[serde(skip)]
    _produkthersteller: ConstStr,
    #[xml(name = b"produktversion", ty = "attr", value = "H & D GmbH")]
    #[serde(skip)]
    _produktversion: ConstStr,
    #[xml(name = b"standard", ty = "attr", value = "XWasser")]
    #[serde(skip)]
    _standard: ConstStr,
    #[xml(name = b"test", ty = "attr")]
    #[serde(default)]
    pub test: bool,
    #[xml(name = b"version", ty = "attr", value = "0.2.0")]
    #[serde(skip)]
    _version: ConstStr,

    #[xml(name = b"nachrichtenkopf.g2g", ty = "child")]
    pub nachrichtenkopf_g2g: NachrichtenkopfG2g,

    #[xml(ns = b"xwas", name = b"vorgang", ty = "child")]
    pub vorgang: Vorgang,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"administration.quittung.0020")]
pub struct AdministrationQuittung {
    #[xml(name = b"nachrichtenkopf.g2g", ty = "child")]
    pub nachrichtenkopf_g2g: NachrichtenkopfG2g,

    #[xml(ns = b"xwas", name = b"identifikationVorgang", ty = "child")]
    pub identifikation_vorgang: IdentifikationVorgang,

    #[xml(ns = b"xwas", name = b"quittung", ty = "child")]
    pub quittung: Quittung,
    
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"quittung")]
pub struct Quittung {
    #[xml(ns = b"xwas", name = b"aktuellerStatusTechnisch", ty = "child")]
    pub aktueller_status_technisch: AktuellerStatusTechnisch,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"aktueller_status_technisch")]
pub struct AktuellerStatusTechnisch {
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
}

#[wasm_bindgen]
pub fn create_quality_report_xml(data: QualityReport) -> Result<String, JsValue> {
    Ok(raxb::ser::to_string_pretty_with_decl(&data)
        .map_err(|err| JsValue::from_str(&err.to_string()))?)
}

#[wasm_bindgen]
pub fn parse_quality_report_xml(xml: String) -> Result<QualityReport, JsValue> {
    Ok(raxb::de::from_str(&xml).map_err(|err| JsValue::from_str(&err.to_string()))?)
}
