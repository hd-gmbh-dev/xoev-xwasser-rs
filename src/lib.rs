#![allow(non_snake_case, dead_code)]

use std::default;

use _raxb::quick_xml::{events::Event, name::ResolveResult};
use raxb::ty::S;
use raxb::{de::XmlDeserializeError, value::ConstStr, XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

// #[cfg(test)]
// mod gen;

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
#[xml(root = b"nachrichtenkopf.g2g")]
pub struct NachrichtenkopfG2g {
    #[xml(name = b"identifikation.nachricht", ty = "child")]
    pub identifikation_nachricht: IdentifikationNachricht,
    #[xml(name = b"leser", ty = "child")]
    pub leser: Leser,
    #[xml(name = b"autor", ty = "child")]
    pub autor: Autor,
    #[xml(name = b"referenzUUID", ty = "child")]
    pub referenz_uuid: String,
    #[xml(name = b"dvdvDienstkennung", ty = "child")]
    pub dvdv_dienstkennung: String, //DvdvDienstkennung,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"identifikationNachricht")]
pub struct IdentifikationNachricht {
    #[xml(name = b"nachrichtenUUID", ty = "child")]
    // #[serde(skip)]
    pub nachrichten_uuid: Option<String>, //ConstStr,
    #[xml(name = b"nachrichtentyp", ty = "child")]
    pub nachrichten_typ: Option<NachrichtenTyp>,
    #[xml(name = b"erstellungszeitpunkt", ty = "child")]
    pub erstellungszeitpunkt: Option<String>,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"")]
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
#[xml(root = b"autor")]
pub struct Autor {
    #[xml(name = b"verzeichnisdienst", ty = "child")]
    pub verzeichnisdienst: Verzeichnisdienst,
    #[xml(name = b"kennung", ty = "child")]
    pub kennung: String,
    #[xml(name = b"name", ty = "child")]
    pub name: String,
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
    pub kennung: String,
    #[xml(name = b"name", ty = "child")]
    pub name: String,
    // #[xml(name = b"erreichbarkeit", ty = "child")]
    // pub erreichbarkeit: Erreichbarkeit,
}


/// Unter "Identifikation" werden die Informationen zusammengefasst, die die eindeutige
/// Identifikation von Objekten in einem fachlichen Kontext erlauben.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"IdentifikationType")]
pub struct IdentifikationType {
    #[xml(name = b"id", ty = "child")]
    pub id: Option<String>,
    #[xml(name = b"beschreibung", ty = "child")]
    pub beschreibung: Option<String>,
    #[xml(name = b"gueltigkeit", ty = "child")]
    pub gueltigkeit: Option<ZeitraumType>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[xml(root = b"vorgang.transportieren.2010")]
// #[xml(tns(b"xwas", b"xwasser"))]
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
        // value = "xwasser xwasser.xsd"
        value = "xwasser ../schemas/V0_2_0/xwasser.xsd"
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

/// Dieser Datentyp enthält die Angaben zu einem Vorgang.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"vorgang")]
// #[tsify(into_wasm_abi, from_wasm_abi)]
// #[xml(tns(b"xwas", b"xwasser"))]
pub struct Vorgang {
    #[xml(ns = b"xwas", name = b"identifikationVorgang", ty = "child")]
    pub identifikation_vorgang: IdentifikationVorgang,
    #[xml(ns = b"xwas", name = b"vorgangType", ty = "child")]
    pub vorgang_type: VorgangType,
    #[xml(name = b"bemerkung", ty = "child")]
    pub bemerkung: Option<String>,
    // #[xml(name = b"erweiterung", ty = "child")]
    // pub erweiterung: Option<Erweiterung>,
    // #[xml(name = b"anlage", ty = "child")]
    // pub anlage: Vec<Anlage>,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"identifikationVorgang")]
pub struct IdentifikationVorgang {
    #[xml(ns = b"xwas", name = b"vorgangsID", ty = "child")]
    pub vorgangs_id: String,
    // #[xml(ns = b"xwas", name = b"aktenzeichen", ty = "child")]
    // pub aktenzeichen: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct RegisterTyp {
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    #[xml(name = b"name", ty = "child")]
    pub name: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeUntersuchungsverfahrenType")]
pub struct CodeUntersuchungsverfahrenType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeSHAPTH-Parameter-EinheitType")]
pub struct CodeShapthParameterEinheitType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeParameterauspraegungType")]
pub struct CodeParameterauspraegungType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeParameterunterauswahlType")]
pub struct CodeParameterunterauswahlType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeMesswertergaenzungType")]
pub struct CodeMesswertergaenzungType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeBewertungUntersuchungswertType")]
pub struct CodeBewertungUntersuchungswertType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}


#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"analyseergebnisParameter")]
// #[xml(tns(b"xwas", b"xwasser"))]
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
    pub untersuchungsverfahren: Vec<CodeUntersuchungsverfahrenType>,
    #[xml(
        ns = b"xwas",
        name = b"ergaenzungZumUntersuchungsverfahren",
        ty = "child"
    )]
    pub ergaenzung_zum_untersuchungsverfahren: Option<String>,
    #[xml(ns = b"xwas", name = b"untersuchterParameter", ty = "child")]
    pub untersuchter_parameter: CodeShapthParameterEinheitType,
    #[xml(ns = b"xwas", name = b"parameterauspraegung", ty = "child")]
    pub parameterauspraegung: Option<CodeParameterauspraegungType>,
    #[xml(ns = b"xwas", name = b"parameterUnterauswahl", ty = "child")]
    pub parameter_unterauswahl: Option<CodeParameterunterauswahlType>,
    #[xml(ns = b"xwas", name = b"sensorischerParameterIstAnnehmbar", ty = "child")]
    pub sensorischer_parameter_ist_annehmbar: Option<bool>,
    #[xml(ns = b"xwas", name = b"untersuchungswertParameter", ty = "child")]
    pub untersuchungswert_parameter: Option<String>,
    #[xml(ns = b"xwas", name = b"einheitDesUntersuchungswerts", ty = "child")]
    pub einheit_des_untersuchungswerts: Option<CodeShapthParameterEinheitType>,
    #[xml(ns = b"xwas", name = b"ergaenzungZumUntersuchungswertParameter", ty = "child")]
    pub ergaenzung_zum_untersuchungswert_parameter: Option<CodeMesswertergaenzungType>,
    #[xml(ns = b"xwas", name = b"parameterwertErgaenzung", ty = "child")]
    pub parameterwert_ergaenzung: Option<String>,
    #[xml(ns = b"xwas", name = b"ausgewertetesAnsatzvolumen", ty = "child")]
    pub ausgewertetes_ansatzvolumen: Option<String>,
    #[xml(ns = b"xwas", name = b"shapthParameterNummer", ty = "child")]
    pub shapth_parameter_nummer: Vec<String>,
    #[xml(ns = b"xwas", name = b"bewertungUntersuchungswert", ty = "child")]
    pub bewertung_untersuchungswert: CodeBewertungUntersuchungswertType,
    #[xml(ns = b"xwas", name = b"parameterauffaelligkeit", ty = "child")]
    pub parameterauffaelligkeit: Option<String>,
    #[xml(ns = b"xwas", name = b"messunsicherheitUntersuchungswert", ty = "child")]
    pub messunsicherheit_untersuchungswert: Option<String>,
    #[xml(ns = b"xwas", name = b"bestimmungsgrenzeLoQ", ty = "child")]
    pub bestimmungsgrenze_lo_q: Option<String>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
}


#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct CodeUntersuchterParameter {
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    #[xml(name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct CodeBewertungUntersuchungswert {
    #[xml(name = b"code", ty = "child")]
    pub code: String,
    #[xml(name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeProbennahmeverfahrenType")]
pub struct CodeProbennahmeverfahrenType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}


#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeMediumType")]
pub struct CodeMediumType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}



#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeAnlassUntersuchungType")]
pub struct CodeAnlassUntersuchungType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct Laenderkuerzel {
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    // #[xml(name = b"name", ty = "child")]
    // pub name: String,
}


#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct Vorname {
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: String,
    // #[xml(name = b"nichtVorhanden", ty = "child")]
    // pub nicht_vorhanden: bool,
}


#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct Familienname {
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: String,
    // #[xml(name = b"nichtVorhanden", ty = "child")]
    // pub nicht_vorhanden: bool,
    // #[xml(name = b"namensart", ty = "child")]
    // pub namensart: Namensart,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeRechtsformenType")]
pub struct CodeRechtsformenType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

/// "NameOrganisation" fasst die Angaben zum Namen einer Organisation zusammen.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"NameOrganisationType")]
pub struct NameOrganisationType {
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
    #[xml(ns = b"xwas", name = b"kurzbezeichnung", ty = "child")]
    pub kurzbezeichnung: Option<String>,
    #[xml(ns = b"xwas", name = b"gueltigkeit", ty = "child")]
    pub gueltigkeit: Option<ZeitraumType>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeKommunikationType")]
pub struct CodeKommunikationType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

/// "Kommunikation" fasst Angaben zur Erreichbarkeit über elektronische
/// Kommunikationskanäle (z.B. Telefon, Fax, E-Mail) zusammen.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"KommunikationType")]
pub struct KommunikationType {
    #[xml(ns = b"xwas", name = b"kanal", ty = "child")]
    pub kanal: Option<CodeKommunikationType>,
    #[xml(ns = b"xwas", name = b"kennung", ty = "child")]
    pub kennung: Option<String>,
    #[xml(ns = b"xwas", name = b"istDienstlich", ty = "child")]
    pub ist_dienstlich: Option<bool>,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    pub zusatz: Option<String>,
}


#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeRegisterType")]
pub struct CodeRegisterType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeBehoerdeType")]
pub struct CodeBehoerdeType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

/// Die Organisationseinheit fasst Angaben zur Darstellung der internen hierarchischen
/// Organisationsstruktur einer Institution zusammen, z.B. zur Darstellung von
/// Abteilungen oder Referaten.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"organisationseinheitType")]
pub struct OrganisationseinheitType {
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: String,
    #[xml(ns = b"xwas", name = b"hierarchieebene", ty = "child")]
    pub hierarchieebene: Option<u8>, //Option<u8>,
    #[xml(ns = b"xwas", name = b"hierarchiename", ty = "child")]
    pub hierarchiename: Option<String>, //Option<String>,
}

/// Die zu verwendende Schlüsseltabelle ergibt sich aus dem Nachrichtenkontext. Zum
/// Beispiel ist bei der Datenübermittlung an ein Standesamt die Schlüsseltabelle der
/// Standesamtsnummern und somit eine Standesamtsnummer als Behördenkennung zu verwenden.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeBehoerdenkennungType")]
pub struct CodeBehoerdenkennungType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

/// Die mit der genutzten Codeliste beschriebenen Präfixe werden für die fachliche
/// Adressierung über das DVDV verwendet.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codePraefixType")]
pub struct CodePraefixType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}



/// Die Behoerdenkennung fasst die Elemente zusammen, über die eine Behörde identifiziert
/// werden kann. Die "Behoerdenkennung" ist prioritär zur Übermittlung der im DVDV
/// verzeichneten Behördenschlüssel vorgesehen, kann aber auch für andere
/// Behördenkennungen, bspw. die BKZ der Justizverwaltung eingesetzt werden. Eine
/// Behördenkennung im DVDV besteht aus einem Präfix und der eigentlichen Kennung. Die
/// Codelisten für die Präfixe sowie die Kennungen pro Präfix werden durch die
/// koordinierende Stelle für das DVDV verwaltet. Anmerkung: Beispiel für die
/// Übermittlung einer Behördenkennung des DVDV: Bei einer Identifikation von Behörden
/// auf kommunaler Ebene anhand des amtlichen Gemeindeschlüssels (AGS) der Gemeinde, für
/// die die Behörde zuständig ist, lautet der Präfix "ags:", die Kennung ist dann der AGS
/// der jeweiligen Gemeinde.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"BehoerdenkennungType")]
pub struct BehoerdenkennungType {
    #[xml(ns = b"xwas", name = b"kennung", ty = "child")]
    pub kennung: Vec<CodeBehoerdenkennungType>,
    #[xml(ns = b"xwas", name = b"praefix", ty = "child")]
    pub praefix: Vec<CodePraefixType>,
}



/// Eine Behörde ist ein Organ eines Verwaltungsträgers, das gegenüber dem
/// Verwaltungsträger berechtigt ist, mit Außenwirkung Aufgaben öffentlichen Handelns
/// (insbes. der Erlass von Verwaltungsakten und das Schließen öffentlich-rechtlicher
/// Verträge) wahrzunehmen.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"BehoerdeType")]
pub struct BehoerdeType {
    #[xml(ns = b"xwas", name = b"id", ty = "child")]
    pub id: Option<String>,
    #[xml(ns = b"xwas", name = b"typ", ty = "child")]
    pub typ: Option<CodeBehoerdeType>,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    pub zusatz: Option<String>,
    #[xml(ns = b"xwas", name = b"behoerdenkennung", ty = "child")]
    pub behoerdenkennung: Option<BehoerdenkennungType>,
    #[xml(ns = b"xwas", name = b"kommunikation", ty = "child")]
    pub kommunikation: Vec<KommunikationType>,
    #[xml(ns = b"xwas", name = b"behoerdenidentifikation", ty = "child")]
    pub behoerdenidentifikation: Option<IdentifikationType>,
    #[xml(ns = b"xwas", name = b"behoerdenname", ty = "child")]
    pub behoerdenname: Vec<NameOrganisationType>,//Option<NameOrganisationType>,
    #[xml(ns = b"xwas", name = b"nachgeordneteBehoerde", ty = "child")]
    pub nachgeordnete_behoerde: Vec<BehoerdeType>,
    #[xml(ns = b"xwas", name = b"verwaltungspolitischeZustaendigkeit", ty = "child")]
    pub verwaltungspolitische_zustaendigkeit: Vec<VerwaltungspolitischeKodierungType>,
    #[xml(ns = b"xwas", name = b"anschrift", ty = "child")]
    pub anschrift: Vec<AnschriftType>,
    #[xml(ns = b"xwas", name = b"organisationsstruktur", ty = "child")]
    pub organisationsstruktur: Vec<OrganisationseinheitType>,
}


/// Angaben zum Registereintrag.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"RegistrierungType")]
pub struct RegistrierungType {
    #[xml(ns = b"xwas", name = b"id", ty = "child")]
    pub id: Option<String>,
    #[xml(ns = b"xwas", name = b"registertyp", ty = "child")]
    pub registertyp: Option<CodeRegisterType>,
    #[xml(ns = b"xwas", name = b"registrierendeBehoerde", ty = "child")]
    pub registrierende_behoerde: Option<BehoerdeType>,
    #[xml(ns = b"xwas", name = b"gueltigkeit", ty = "child")]
    pub gueltigkeit: Option<ZeitraumType>,
}



/// Eine Organisation ist eine Vereinigung mehrerer natürlicher oder juristischer
/// Personen bzw. eine rechtsfähige Personengesellschaft zu einem gemeinsamen Zweck, z.B.
/// im wirtschaftlichen, gemeinnützigen, religiösen, öffentlichen oder politischen
/// Bereich. Behörden werden über eine eigene Kernkomponente "Behoerde" abgebildet.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"OrganisationType")]
pub struct OrganisationType {
    #[xml(ns = b"xwas", name = b"rechtsform", ty = "child")]
    pub rechtsform: Option<CodeRechtsformenType>,
    #[xml(ns = b"xwas", name = b"branche", ty = "child")]
    pub branche: Vec<CodeBrancheType>,
    #[xml(ns = b"xwas", name = b"zweck", ty = "child")]
    pub zweck: Vec<CodeZweckType>,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<NameOrganisationType>,
    #[xml(ns = b"xwas", name = b"unterorganisation", ty = "child")]
    pub unterorganisation: Vec<OrganisationType>,
    #[xml(ns = b"xwas", name = b"kommunikation", ty = "child")]
    pub kommunikation: Vec<KommunikationType>,
    #[xml(ns = b"xwas", name = b"registrierung", ty = "child")]
    pub registrierung: Vec<RegistrierungType>,
    #[xml(ns = b"xwas", name = b"identifikation", ty = "child")]
    pub identifikation: Vec<IdentifikationType>,
    #[xml(ns = b"xwas", name = b"existenzzeitraum", ty = "child")]
    pub existenzzeitraum: Option<ZeitraumType>,
    #[xml(ns = b"xwas", name = b"anschrift", ty = "child")]
    pub anschrift: Vec<AnschriftType>,
}


#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct RegistrierendeBehoerde {
    #[xml(name = b"id", ty = "child")]
    pub id: String, 
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeAnschrifttypType")]
pub struct CodeAnschrifttypType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeStaatType")]
pub struct CodeStaatType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}


#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeKreisType")]
pub struct CodeKreisType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeBezirkType")]
pub struct CodeBezirkType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeBundeslandType")]
pub struct CodeBundeslandType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeAGSType")]
pub struct CodeAGSType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeRegionalschluesselType")]
pub struct CodeRegionalschluesselType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}


/// Die Komponente "VerwaltungspolitischeKodierung" beinhaltet Information, die eine
/// verwaltungspolitisch eindeutige Zuordnung ermöglichen.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"VerwaltungspolitischeKodierungType")]
pub struct VerwaltungspolitischeKodierungType {
    #[xml(ns = b"xwas", name = b"kreis", ty = "child")]
    pub kreis: Option<CodeKreisType>,
    #[xml(ns = b"xwas", name = b"bezirk", ty = "child")]
    pub bezirk: Option<CodeBezirkType>,
    #[xml(ns = b"xwas", name = b"bundesland", ty = "child")]
    pub bundesland: Option<CodeBundeslandType>,
    #[xml(ns = b"xwas", name = b"gemeindeschluessel", ty = "child")]
    pub gemeindeschluessel: Option<CodeAGSType>,
    #[xml(ns = b"xwas", name = b"regionalschluessel", ty = "child")]
    pub regionalschluessel: Option<CodeRegionalschluesselType>,
    #[xml(ns = b"xwas", name = b"nation", ty = "child")]
    pub nation: Option<CodeStaatType>,
}


/// Eine Anschrift beschreibt einen Ort mit den klassischen Ordnungsbegriffen wie Orts-
/// und Straßennamen sowie ergänzenden Informationen wie Ortsteil und Postfach. Eine
/// Anschrift kann genutzt werden, um Orte zu benennen, an denen sich Personen aufhalten,
/// an denen Objekte zu finden sind, oder an denen Ereignisse stattfinden. Darüber hinaus
/// kann sie genutzt werden, um Post oder Waren zuzustellen. Daher enthält sie auch die
/// notwendigen Attribute um Postfächer zu adressieren. Die Anschrift kann außerdem über
/// eine Subkomponente verfügen, die eine Beschreibung des Ortes mittels Geokoordinaten
/// erlaubt. Die Anschrift kann auch über eine Subkomponente verfügen, die eine
/// verwaltungspolitische Zuordnung des Ortes erlaubt (Zuordnung zu einer Gemeinde über
/// den AGS, eines Bundesland, etc.).
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(root = b"AnschriftType")]
pub struct AnschriftType {
    #[xml(name = b"id", ty = "attr")]
    pub id: Option<String>,
    #[xml(name = b"strassenschluessel", ty = "child")]
    pub strassenschluessel: Code,
    #[xml(name = b"strasse", ty = "child")]
    pub strasse: Option<String>,
    #[xml(name = b"hausnummer", ty = "child")]
    pub hausnummer: Option<String>,
    #[xml(name = b"postfach", ty = "child")]
    pub postfach:  Option<String>,
    #[xml(name = b"postleitzahl", ty = "child")]
    pub postleitzahl: Option<String>,
    #[xml(name = b"ort", ty = "child")]
    pub ort: Option<String>,
    #[xml(name = b"ortsteil", ty = "child")]
    pub ortsteil: Option<String>,
    #[xml(name = b"ortFruehererGemeindename", ty = "child")]
    pub ort_frueherer_gemeindename: Option<String>,
    #[xml(name = b"wohnungsgeber", ty = "child")]
    pub wohnungsgeber: Option<String>,
    #[xml(name = b"zusatz", ty = "child")]
    pub zusatz: Option<String>,
    #[xml(name = b"typ", ty = "child")]
    pub typ: Vec<CodeAnschrifttypType>,
    #[xml(name = b"staat", ty = "child")]
    pub staat: Option<CodeStaatType>,
    #[xml(name = b"verwaltungspolitischeKodierung", ty = "child")]
    pub verwaltungspolitische_kodierung: Option<VerwaltungspolitischeKodierungType>,
}


#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct CodeBrancheType {
    // typ xoev-code:Code
    #[xml(name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}


#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeArtProbennahmestelleType")]
pub struct CodeArtProbennahmestelleType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}


#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeAufbereitungsstoffDesinfektionsverfahrenType")]
pub struct CodeAufbereitungsstoffDesinfektionsverfahrenType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}



/// Klasse für den Transport von Informationen zu einer Probennahmestelle.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"ProbennahmestelleType")]
pub struct ProbennahmestelleType {
    #[xml(ns = b"xwas", name = b"probennahmestelleID", ty = "child")]
    pub probennahmestelle_id: String,
    #[xml(ns = b"xwas", name = b"objektID", ty = "child")]
    pub objekt_id: String,
    #[xml(ns = b"xwas", name = b"probe", ty = "child")]
    pub probe: Vec<ProbeType>,
    #[xml(ns = b"xwas", name = b"terminplanID", ty = "child")]
    pub terminplan_id: Vec<String>,
    #[xml(ns = b"xwas", name = b"nameProbennahmestelle", ty = "child")]
    pub name_probennahmestelle: String,
    #[xml(ns = b"xwas", name = b"artProbennahmestelle", ty = "child")]
    pub art_probennahmestelle: CodeArtProbennahmestelleType,
    #[xml(ns = b"xwas", name = b"stockwerkProbennahmestelle", ty = "child")]
    pub stockwerk_probennahmestelle: Option<u8>,
    #[xml(ns = b"xwas", name = b"mediumAnDerProbennahmestelle", ty = "child")]
    pub medium_an_der_probennahmestelle: Vec<CodeMediumType>,
    #[xml(ns = b"xwas", name = b"desinfektionUndAufbereitungDesWassers", ty = "child")]
    pub desinfektion_und_aufbereitung_des_wassers: Vec<CodeAufbereitungsstoffDesinfektionsverfahrenType>,
    #[xml(ns = b"xwas", name = b"altID", ty = "child")]
    pub alt_id: Option<String>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
    #[xml(ns = b"xwas", name = b"probennehmerID", ty = "attr")]
    #[serde(skip)]
    pub _id: ConstStr,

}


#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct CodeZweckType { 
    // xoev-code:Code
    #[xml(name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}


#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeUntersuchungsstelleType")]
pub struct CodeUntersuchungsstelleType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}


/// Klasse für den Transport von Informationen zu einer zugelassenen Untersuchungsstelle.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"zugelasseneUntersuchungsstelleType")]
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
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"BeauftragteUntersuchungsstelleType")]
pub struct BeauftragteUntersuchungsstelleType {
    #[xml(ns = b"xwas", name = b"zugelasseneUntersuchungsstelle", ty = "child")]
    pub zugelassene_untersuchungsstelle: ZugelasseneUntersuchungsstelleType,
    #[xml(ns = b"xwas", name = b"kommentarBeauftragteUntersuchungsstelle", ty = "child")]
    pub kommentar_beauftragte_untersuchungsstelle: Option<String>,
}


#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"ZugelasseneUntersuchungsstelleType")]
pub struct OrtDerLabortaetigkeiten {
    #[xml(name = b"id", ty = "attr", value = "H & D GmbH")]
    _id: String,
}


#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeGesamtbewertungType")]
pub struct CodeGesamtbewertungType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeAmtsspracheEUType")]
pub struct CodeAmtsspracheEUType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

/// Erweiterung.XML darf nur dazu genutzt werden, weitere (z. B. fachspezifische)
/// Metadaten zu spezifizieren, deren Übermittlung mit den bereits in XWasser
/// spezifizierten Metadaten nicht möglich ist. Erweiterung.XML bietet über ein
/// xs:any-Element die Möglichkeit, mittels Einbindung externer XML-Schemata diese
/// Metadaten zu spezifizieren. Es können beliebige XML-Schemata mit unterschiedlichen
/// Namensräumen angegeben werden. Die XML-Schema-Validierung der weiterführenden
/// Metadaten erfolgt innerhalb der XWasser-Nachricht selbst (prozessContents = "lax").
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"erweiterungXMLType")]
pub struct ErweiterungXmlType {
    // weiteres xml schema 
    #[xml(ns = b"xwas", name = b"any", ty = "child")]
    pub any: Vec<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeDatentypType")]
pub struct CodeDatentypType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}


/// Ein Feld ist ein anwendungsspezifisches Metadatum. Die Konfiguration eines Feldes
/// muss zwischen den Kommunikationspartnern abgesprochen sein.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"erweiterungFeldType")]
pub struct ErweiterungFeldType {
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: String,
    #[xml(ns = b"xwas", name = b"beschreibung", ty = "child")]
    pub beschreibung: Option<String>,
    #[xml(ns = b"xwas", name = b"datentyp", ty = "child")]
    pub datentyp: CodeDatentypType,
    #[xml(ns = b"xwas", name = b"wert", ty = "child")]
    pub wert: String,
}


/// Eine Erweiterungsgruppe fasst mehrere Felder (Metadaten) zu einem Objekt zusammen .
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"erweiterungGruppeType")]
pub struct ErweiterungGruppeType {
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: String,
    #[xml(ns = b"xwas", name = b"beschreibung", ty = "child")]
    pub beschreibung: String,
    #[xml(ns = b"xwas", name = b"untergruppe", ty = "child")]
    pub untergruppe: Vec<ErweiterungGruppeType>,
    #[xml(ns = b"xwas", name = b"feld", ty = "child")]
    pub feld: Vec<ErweiterungFeldType>,
}


/// Die Klasse Erweiterung dient zur Übertragung generischer Informationen. Die
/// Verwendung ist für folgende Fälle vorgesehen: Es müssen zeitnah Informationen
/// übertragen werden (zum Beispiel aufgrund einer Gesetzesänderung), für die es in der
/// aktuellen Version des Standards noch keine geeigneten Klassen und Elemente gibt.
/// Zwischen Sender und Empfänger wird bilateral die Übermittlung einer strukturierten
/// Information vereinbart, die sich mit den vorhandenen Klassen und Elemente nicht
/// abbilden lässt.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"ErweiterungType")]
pub struct ErweiterungType {
    #[xml(ns = b"xwas", name = b"feld", ty = "child")]
    pub feld: Vec<ErweiterungFeldType>,
    #[xml(ns = b"xwas", name = b"gruppe", ty = "child")]
    pub gruppe: Vec<ErweiterungGruppeType>,
    #[xml(ns = b"xwas", name = b"xml", ty = "child")]
    pub xml: Option<ErweiterungXmlType>,
}



/// Klasse zur Erfassung bzw. zum Transport der Daten eines Prüfberichts. Prüfberichte
/// werden erstellt, nachdem eine Wasserprobe im Labor analysiert wurde.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"PruefberichtType")]
pub struct PruefberichtType {
    #[xml(ns = b"xwas", name = b"pruefberichtUUID", ty = "child")]
    pub pruefbericht_uuid: String,
    #[xml(ns = b"xwas", name = b"untersuchungsplanID", ty = "child")]
    pub untersuchungsplan_id: Option<String>,
    #[xml(ns = b"xwas", name = b"probennahmestelle", ty = "child")]
    pub probennahmestelle: Vec<ProbennahmestelleType>,
    #[xml(ns = b"xwas", name = b"nameBeauftragteUntersuchungsstelle", ty = "child")]
    pub name_beauftragte_untersuchungsstelle: CodeNameBeauftragteUntersuchungsstelle,
    #[xml(ns = b"xwas", name = b"pruefberichtEnthaeltTeilergebnisse", ty = "child")]
    pub pruefbericht_enthaelt_teilergebnisse: Option<bool>,
    #[xml(ns = b"xwas", name = b"pruefgerichtGemVorgabenAkkredition", ty = "child")]
    pub pruefgericht_gem_vorgaben_akkredition: bool,
    #[xml(ns = b"xwas", name = b"titel", ty = "child")]
    pub titel: String,
    #[xml(ns = b"xwas", name = b"gesamtbewertung", ty = "child")]
    pub gesamtbewertung: CodeGesamtbewertungType,
    #[xml(ns = b"xwas", name = b"auffaelligkeiten", ty = "child")]
    pub auffaelligkeiten: Vec<String>,
    #[xml(ns = b"xwas", name = b"zeitpunktValidierungPruefbericht", ty = "child")]
    pub zeitpunkt_validierung_pruefbericht: String, //datetime
    #[xml(ns = b"xwas", name = b"fuerValidierungVerantwortlichePerson", ty = "child")]
    pub fuer_validierung_verantwortliche_person: Vec<NatuerlichePersonType>,
    #[xml(ns = b"xwas", name = b"freigabeUebermittlungBetreiber", ty = "child")]
    pub freigabe_uebermittlung_betreiber: Option<bool>,
    #[xml(ns = b"xwas", name = b"pruefberichtIDLabor", ty = "child")]
    pub pruefbericht_id_labor: String,
    #[xml(ns = b"xwas", name = b"swVersion", ty = "child")]
    pub sw_version: CodeAmtsspracheEUType,
    #[xml(ns = b"xwas", name = b"sprachePruefbericht", ty = "child")]
    pub sprache_pruefbericht: CodeAmtsspracheEUType,
    #[xml(ns = b"xwas", name = b"rechtlicherDisclaimer", ty = "child")]
    pub rechtlicher_disclaimer: String, //datetime
    #[xml(ns = b"xwas", name = b"zeitpunktUebermittlungAnSHAPTH", ty = "child")]
    pub zeitpunkt_uebermittlung_an_shapth: String, //datetime
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
    #[xml(ns = b"xwas", name = b"auftraggeber", ty = "child")]
    pub auftraggeber: AuftraggeberType,
    #[xml(ns = b"xwas", name = b"zustaendigeBehoerde", ty = "child")]
    pub zustaendige_behoerde: Vec<ZustaendigeBehoerdeType>,
    #[xml(ns = b"xwas", name = b"beauftragteUntersuchungsstelle", ty = "child")]
    pub beauftragte_untersuchungsstelle: BeauftragteUntersuchungsstelleType,
    #[xml(ns = b"xwas", name = b"ortDerLabortaetigkeiten", ty = "child")]
    pub ort_der_labortaetigkeiten: Vec<AnschriftType>,
    #[xml(ns = b"xwas", name = b"anhang", ty = "child")]
    pub anhang: Vec<String>,
    #[xml(ns = b"xwas", name = b"erweiterung", ty = "child")]
    pub erweiterung: Option<ErweiterungType>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeWasserversorgungsgebietType")]
pub struct CodeWasserversorgungsgebietType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

/// Klasse zur Abbildung von SHAPTH-spezifischen Geokoordinaten.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"geokoordinatenShapthType")]
pub struct GeokoordinatenShapthType {
    #[xml(ns = b"xwas", name = b"geografischePositionUndAusdehnung", ty = "child")]
    pub geografische_position_und_ausdehnung: Option<String>, // xs:base64Binary
    #[xml(ns = b"xwas", name = b"nameShapefile", ty = "child")]
    pub name_shapefile: Option<String>,
    #[xml(ns = b"xwas", name = b"geokoordinatenBreitengrad", ty = "child")]
    pub geokoordinaten_breitengrad: Option<f64>,
    #[xml(ns = b"xwas", name = b"geokoordinatenLaengengrad", ty = "child")]
    pub geokoordinaten_laengengrad: Option<f64>,
    #[xml(ns = b"xwas", name = b"geokoordinatenRechtswert", ty = "child")]
    pub geokoordinaten_rechtswert: Option<u8>,
    #[xml(ns = b"xwas", name = b"geokoordinatenHochwert", ty = "child")]
    pub geokoordinaten_hochwert: Option<u8>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeGrundSchliessungWasserversorgungsgebietType")]
pub struct CodeGrundSchliessungWasserversorgungsgebietType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeArtWasserressourceType")]
pub struct CodeArtWasserressourceType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeGrundAusnahmeregelungType")]
pub struct CodeGrundAusnahmeregelungType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeAbhilfemassnahmeType")]
pub struct CodeAbhilfemassnahmeType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}


/// Transport von Informationen, die für die Maßnahmen zur Ausnahmeregelungen gem. de
/// neuen Vorgaben für das EU-Berichtsformat benötigt werden.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"DerogationRemedialActionType")]
pub struct DerogationRemedialActionType {
    #[xml(ns = b"xwas", name = b"derogationRemedialActionIdentifier", ty = "child")]
    pub derogation_remedial_action_identifier: String, // xs:ID
    #[xml(ns = b"xwas", name = b"derogationRemedialAction", ty = "child")]
    pub derogation_remedial_action: CodeAbhilfemassnahmeType,
    #[xml(ns = b"xwas", name = b"derogationRemedialActionStartDate", ty = "child")]
    pub derogation_remedial_action_start_date: String,
    #[xml(ns = b"xwas", name = b"derogationRemedialActionEndDate", ty = "child")]
    pub derogation_remedial_action_end_date: String,
    #[xml(ns = b"xwas", name = b"derogationRemedialActionCost", ty = "child")]
    pub derogation_remedial_action_cost: f64,
    #[xml(ns = b"xwas", name = b"remarks", ty = "child")]
    pub remarks: Option<String>,
    #[xml(ns = b"xwas", name = b"_id", ty = "attr")]
    #[serde(skip)]
    pub _id: ConstStr,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeShapthParameterType")]
pub struct CodeShapthParameterType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeProbennahmezeitraumType")]
pub struct CodeProbennahmezeitraumType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeArtProbennahmestelleEUType")]
pub struct CodeArtProbennahmestelleEUType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}


/// Transport der Informationen, die für Qualität und Überwachung gem. dem neuen Vorgaben
/// für das EU-Berichtsformat benötigt werden.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"QualityAndMonitoringType")]
pub struct QualityAndMonitoringType {
    #[xml(ns = b"xwas", name = b"qualityAndMonitoringRequirementIdentifier", ty = "child")]
    pub quality_and_monitoring_requirement_identifier: String, // bn-uq-g2g:UUID
    #[xml(ns = b"xwas", name = b"parameterCode", ty = "child")]
    pub parameter_code: CodeShapthParameterType,
    #[xml(ns = b"xwas", name = b"parameterThresholdValue", ty = "child")]
    pub parameter_threshold_value: f64,
    #[xml(ns = b"xwas", name = b"parameterThresholdValueUnit", ty = "child")]
    pub parameter_threshold_value_unit: CodeShapthParameterEinheitType,
    #[xml(ns = b"xwas", name = b"samplingFrequency", ty = "child")]
    pub sampling_frequency: u32,
    #[xml(ns = b"xwas", name = b"samplingPeriod", ty = "child")]
    pub sampling_period: CodeProbennahmezeitraumType,
    #[xml(ns = b"xwas", name = b"samplingLocationType", ty = "child")]
    pub sampling_location_type: Vec<CodeArtProbennahmestelleEUType>,
    #[xml(ns = b"xwas", name = b"remarks", ty = "child")]
    pub remarks: Option<String>,
    #[xml(ns = b"xwas", name = b"_id", ty = "attr")]
    #[serde(skip)]
    pub _id: ConstStr,
}

/// Transport solcher Informationen, die für die Ausnahmeregelungen gem. den neuen
/// Vorgaben für das EU-Berichtsformat benötigt werden.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"DerogationType")]
pub struct DerogationType {
    #[xml(ns = b"xwas", name = b"derogationIdentifier", ty = "child")]
    pub derogation_identifier: String, // bn-uq-g2g:UUID
    #[xml(ns = b"xwas", name = b"trivialDerogation", ty = "child")]
    pub trivial_derogation: bool,
    #[xml(ns = b"xwas", name = b"trivialDerogationJustification", ty = "child")]
    pub trivial_derogation_justification: Option<String>,
    #[xml(ns = b"xwas", name = b"derogationStartDate", ty = "child")]
    pub derogation_start_date: Option<String>, // xs:date
    #[xml(ns = b"xwas", name = b"derogationEndDate", ty = "child")]
    pub derogation_end_date: Option<String>, // xs:date
    #[xml(ns = b"xwas", name = b"volumeOfWaterSupplied", ty = "child")]
    pub volume_of_water_supplied: Option<f64>,
    #[xml(ns = b"xwas", name = b"derogationAffectedPopulation", ty = "child")]
    pub derogation_affected_population: Option<u32>,
    #[xml(ns = b"xwas", name = b"foodProductionAffected", ty = "child")]
    pub food_production_affected: Option<bool>,
    #[xml(ns = b"xwas", name = b"derogationUnderRecastDWD", ty = "child")]
    pub derogation_under_recast_dwd: bool,
    #[xml(ns = b"xwas", name = b"derogationGrounds", ty = "child")]
    pub derogation_grounds: Option<CodeGrundAusnahmeregelungType>,
    #[xml(ns = b"xwas", name = b"previousDerogationIdentifier", ty = "child")]
    pub previous_derogation_identifier: Option<String>, // xs:IDREF
    #[xml(ns = b"xwas", name = b"previousDerogationConclusions", ty = "child")]
    pub previous_derogation_conclusions: Option<String>, 
    #[xml(ns = b"xwas", name = b"previousDerogationStartDate", ty = "child")]
    pub previous_derogation_start_date: Option<String>, // xs:date
    #[xml(ns = b"xwas", name = b"previousDerogationEndDate", ty = "child")]
    pub previous_derogation_end_date: Option<String>, // xs:date
    #[xml(ns = b"xwas", name = b"previousDerogationGrounds", ty = "child")]
    pub previous_derogation_grounds: Option<CodeGrundAusnahmeregelungType>,
    #[xml(ns = b"xwas", name = b"remarks", ty = "child")]
    pub remarks: Option<String>,
    #[xml(ns = b"xwas", name = b"derogationRemedialAction", ty = "child")]
    pub derogation_remedial_action: Vec<DerogationRemedialActionType>,
    #[xml(ns = b"xwas", name = b"qualityAndMonitoring", ty = "child")]
    pub quality_and_monitoring: Vec<QualityAndMonitoringType>,
    #[xml(ns = b"xwas", name = b"_id", ty = "attr")]
    #[serde(skip)]
    pub _id: ConstStr,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeMassnahmeType")]
pub struct CodeMassnahmeType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeIncidentExceedanceCauseType")]
pub struct CodeIncidentExceedanceCauseType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}


/// Klasse für den Transport von Informationen, die für Ursachen und Maßnahmen von
/// Überschreitungen gem. den neuen Vorgaben für das EU-Berichtsformat benötigt werden.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"ExceedanceCauseAndRemedialActionType")]
pub struct ExceedanceCauseAndRemedialActionType {
    #[xml(ns = b"xwas", name = b"exceedanceCauseAndRemedialActionIdentifier", ty = "child")]
    pub exceedance_cause_and_remedial_action_identifier: String, // bn-uq-g2g:UUID
    #[xml(ns = b"xwas", name = b"exceedanceCause", ty = "child")]
    pub exceedance_cause: CodeIncidentExceedanceCauseType,
    #[xml(ns = b"xwas", name = b"exceedanceRemedialAction", ty = "child")]
    pub exceedance_remedial_action: CodeMassnahmeType,
    #[xml(ns = b"xwas", name = b"exceedanceRemedialActionStartDate", ty = "child")]
    pub exceedance_remedial_action_start_date: String, // xs:date
    #[xml(ns = b"xwas", name = b"exceedanceRemedialActionEndDate", ty = "child")]
    pub exceedance_remedial_action_end_date: String, // xs:date
    #[xml(ns = b"xwas", name = b"remarks", ty = "child")]
    pub remarks: Option<String>,
    #[xml(ns = b"xwas", name = b"_id", ty = "attr")]
    #[serde(skip)]
    pub _id: ConstStr,
}


/// Klasse für den Transport von Informationen, die für Überschreitungen gem. den neuen
/// Vorgaben für das EU-Berichtsformat benötigt werden.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"ExceedanceType")]
pub struct ExceedanceType {
    #[xml(ns = b"xwas", name = b"exceedanceIdentifier", ty = "child")]
    pub exceedance_identifier: String, // bn-uq-g2g:UUID
    #[xml(ns = b"xwas", name = b"trivialExceedance", ty = "child")]
    pub trivial_exceedance: bool,
    #[xml(ns = b"xwas", name = b"trivialExceedanceJustification", ty = "child")]
    pub trivial_exceedance_justification: Option<String>,
    #[xml(ns = b"xwas", name = b"parameterCode", ty = "child")]
    pub parameter_code: CodeShapthParameterType,
    #[xml(ns = b"xwas", name = b"exceedanceStartDate", ty = "child")]
    pub exceedance_start_date: String, // xs:date
    #[xml(ns = b"xwas", name = b"exceedanceEndDate", ty = "child")]
    pub exceedance_end_date: String, // xs:date
    #[xml(ns = b"xwas", name = b"exceedanceAffectedPopulation", ty = "child")]
    pub exceedance_affected_population: u32,
    #[xml(ns = b"xwas", name = b"pointOfComplianceType", ty = "child")]
    pub point_of_compliance_type: Vec<CodeArtProbennahmestelleEUType>,
    #[xml(ns = b"xwas", name = b"numberOfSamplesPerYear", ty = "child")]
    pub number_of_samples_per_year: u32,
    #[xml(ns = b"xwas", name = b"incidentIdentifier", ty = "child")]
    pub incident_identifier: Option<String>, // xs:IDREF
    #[xml(ns = b"xwas", name = b"derogationIdentifier", ty = "child")]
    pub derogation_identifier: Option<String>, //xs:IDREF
    #[xml(ns = b"xwas", name = b"remarks", ty = "child")]
    pub remarks: Option<String>,
    #[xml(ns = b"xwas", name = b"exceedanceCauseAndRemedialAction", ty = "child")]
    pub exceedance_cause_and_remedial_action: Vec<ExceedanceCauseAndRemedialActionType>,
    #[xml(ns = b"xwas", name = b"_id", ty = "attr")]
    #[serde(skip)]
    pub _id: ConstStr,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeIncidentCategoryType")]
pub struct CodeIncidentCategoryType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

/// Klasse für den Transport von Informationen, die zur Beschreibung von Ursache und
/// Maßnahmen von Vorfällen gem. den neuen Vorgaben für das EU-Berichtsformat benötigt
/// werden.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"IncidentCauseAndRemedialActionType")]
pub struct IncidentCauseAndRemedialActionType {
    #[xml(ns = b"xwas", name = b"incidentCauseAndActionIdentifier", ty = "child")]
    pub incident_cause_and_action_identifier: String, // bn-uq-g2g:UUID
    #[xml(ns = b"xwas", name = b"incidentCause", ty = "child")]
    pub incident_cause: CodeIncidentExceedanceCauseType,
    #[xml(ns = b"xwas", name = b"incidentRemedialAction", ty = "child")]
    pub incident_remedial_action: CodeMassnahmeType,
    #[xml(ns = b"xwas", name = b"incidentRemedialActionStartDate", ty = "child")]
    pub incident_remedial_action_start_date: String, // xs:date
    #[xml(ns = b"xwas", name = b"incidentRemedialActionEndDate", ty = "child")]
    pub incident_remedial_action_end_date: String, // xs:date
    #[xml(ns = b"xwas", name = b"remarks", ty = "child")]
    pub remarks: Option<String>,
    #[xml(ns = b"xwas", name = b"_id", ty = "attr")]
    #[serde(skip)]
    pub _id: ConstStr,
}


/// Klasse für den Transport von Informationen, die für Vorfälle gem. den neuen Vorgaben
/// für das EU-Berichtsformat benötigt werden.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"IncidentType")]
pub struct IncidentType {
    #[xml(ns = b"xwas", name = b"incidentIdentifier", ty = "child")]
    pub incident_identifier: String, // bn-uq-g2g:UUID
    #[xml(ns = b"xwas", name = b"exceedance", ty = "child")]
    pub exceedance: Vec<String>, // xs:IDREF
    #[xml(ns = b"xwas", name = b"incidentStartDate", ty = "child")]
    pub incident_start_date: String, // xs:date
    #[xml(ns = b"xwas", name = b"incidentEndDate", ty = "child")]
    pub incident_end_date: String, // xs:date
    #[xml(ns = b"xwas", name = b"incidentCategory", ty = "child")]
    pub incident_category: Vec<CodeIncidentCategoryType>,
    #[xml(ns = b"xwas", name = b"incidentAffectedPopulation", ty = "child")]
    pub incident_affected_population: u32,
    #[xml(ns = b"xwas", name = b"remarks", ty = "child")]
    pub remarks: Option<String>,
    #[xml(ns = b"xwas", name = b"incidentCauseAndRemedialAction", ty = "child")]
    pub incident_cause_and_remedial_action: Vec<IncidentCauseAndRemedialActionType>,
    #[xml(ns = b"xwas", name = b"_id", ty = "attr")]
    #[serde(skip)]
    pub _id: ConstStr,
}


/// Klasse für den Transport von Informationen zu einem Wasserversorgungsgebiet [Soweit
/// möglich in Register zu pflegen].
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"WasserversorgungsgebietType")]
pub struct WasserversorgungsgebietType {
    #[xml(ns = b"xwas", name = b"wasserversorgungsgebietID", ty = "child")]
    pub wasserversorgungsgebiet_id: String,
    #[xml(ns = b"xwas", name = b"nameWasserversorgungsgebiet", ty = "child")]
    pub name_wasserversorgungsgebiet: CodeWasserversorgungsgebietType,
    #[xml(ns = b"xwas", name = b"lau2Code", ty = "child")]
    pub lau2_code: Option<String>,
    #[xml(ns = b"xwas", name = b"zustaendigeBehoerde", ty = "child")]
    pub zustaendige_behoerde: Vec<ZustaendigeBehoerdeType>,
    #[xml(ns = b"xwas", name = b"geokoordinatenSHAPTH", ty = "child")]
    pub geokoordinaten_shapth: Option<GeokoordinatenShapthType>,
    #[xml(ns = b"xwas", name = b"datumDerEinrichtung", ty = "child")]
    pub datum_der_einrichtung: String, // xs:date
    #[xml(ns = b"xwas", name = b"datumDerSchliessung", ty = "child")]
    pub datum_der_schliessung: Option<String>, //xs:date
    #[xml(ns = b"xwas", name = b"grundDerSchliessung", ty = "child")]
    pub grund_der_schliessung: Option<CodeGrundSchliessungWasserversorgungsgebietType>,
    #[xml(ns = b"xwas", name = b"nachfolgerWVGbeiSchliessung", ty = "child")]
    pub nachfolger_wvg_bei_schliessung: Vec<String>, // xs:IDREF
    #[xml(ns = b"xwas", name = b"wvgFremdbezogen", ty = "child")]
    pub wvg_fremdbezogen: Vec<String>, // xs:IDREF
    #[xml(ns = b"xwas", name = b"abgegebeneWassermenge", ty = "child")]
    pub abgegebene_wassermenge: f64,
    #[xml(ns = b"xwas", name = b"anzahlVersorgtePersonenWVG", ty = "child")]
    pub anzahl_versorgte_personen_wvg: u32,
    #[xml(ns = b"xwas", name = b"referenzjahrAngabenWVG", ty = "child")]
    pub referenzjahr_angaben_wvg: u32,
    #[xml(ns = b"xwas", name = b"artDerWasserressource", ty = "child")]
    pub art_der_wasserressource: Vec<CodeArtWasserressourceType>,
    #[xml(ns = b"xwas", name = b"anteilDerWasserressource", ty = "child")]
    pub anteil_der_wasserressource: Vec<u32>,
    #[xml(ns = b"xwas", name = b"vorgeschriebeneUntersuchungshaeufigkeitParameterA", ty = "child")]
    pub vorgeschriebene_untersuchungshaeufigkeit_parameter_a: u32,
    #[xml(ns = b"xwas", name = b"vorgeschriebeneUntersuchungshaeufigkeitParameterB", ty = "child")]
    pub vorgeschriebene_untersuchungshaeufigkeit_parameter_b: u32,
    #[xml(ns = b"xwas", name = b"altID", ty = "child")]
    pub alt_id: Option<String>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
    #[xml(ns = b"xwas", name = b"derogation", ty = "child")]
    pub derogation: Vec<DerogationType>,
    #[xml(ns = b"xwas", name = b"exceedance", ty = "child")]
    pub exceedance: Vec<ExceedanceType>,
    #[xml(ns = b"xwas", name = b"incident", ty = "child")]
    pub incident: Vec<IncidentType>,
    #[xml(ns = b"xwas", name = b"_id", ty = "attr")]
    #[serde(skip)]
    pub _id: ConstStr,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeWVAType")]
pub struct CodeWVAType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeErlaeuterungWasserabgabemengeType")]
pub struct CodeErlaeuterungWasserabgabemengeType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeFlockungType")]
pub struct CodeFlockungType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeDesinfektionsartType")]
pub struct CodeDesinfektionsartType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeNachweisartType")]
pub struct CodeNachweisartType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeStatusUntersuchungsplanType")]
pub struct CodeStatusUntersuchungsplanType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeKategorieProbennahmestelleType")]
pub struct CodeKategorieProbennahmestelleType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeUeberwachungAufbereitungType")]
pub struct CodeUeberwachungAufbereitungType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}



/// Klasse für den Transport von Informationen, die für die Erstellung von Terminplänen
/// als Teil des Untersuchungsplans für a- und b-Anlagen relevant sind.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"TerminplanType")]
pub struct TerminplanType {
    #[xml(ns = b"xwas", name = b"terminplanID", ty = "child")]
    pub terminplan_id: String,
    #[xml(ns = b"xwas", name = b"probennahmestelleID", ty = "child")]
    pub probennahmestelle_id: Option<String>,
    #[xml(ns = b"xwas", name = b"datumZeitraum", ty = "child")]
    pub datum_zeitraum: Vec<String>,
    #[xml(ns = b"xwas", name = b"probennahmestelleKategorie", ty = "child")]
    pub probennahmestelle_kategorie: CodeKategorieProbennahmestelleType,
    #[xml(ns = b"xwas", name = b"weitereBeschreibungDerProbennahmestelle", ty = "child")]
    pub weitere_beschreibung_der_probennahmestelle: Option<String>,
    #[xml(ns = b"xwas", name = b"untersuchungDurch", ty = "child")]
    pub untersuchung_durch: Vec<CodeUeberwachungAufbereitungType>,
    #[xml(ns = b"xwas", name = b"zuUntersuchendeParameter", ty = "child")]
    pub zu_untersuchende_parameter: Vec<CodeShapthParameterType>,
    #[xml(ns = b"xwas", name = b"probennahmeverfahren", ty = "child")]
    pub probennahmeverfahren: Vec<CodeProbennahmeverfahrenType>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeArtTrinkwasseranlageType")]
pub struct CodeArtTrinkwasseranlageType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeArtObjektType")]
pub struct CodeArtObjektType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeBetriebszustandType")]
pub struct CodeBetriebszustandType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeRahmenTrinkwasserbereitstellungType")]
pub struct CodeRahmenTrinkwasserbereitstellungType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

/// Informationen zu einem Auftraggeber [Ergänzende Angaben zu den jeweiligen
/// Informationen aus den Registern von Betreibern/Behörden].
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[serde(tag = "t", content = "c")]
pub enum ArtDerPerson {
    // #[xml(ns = b"xwas", name = b"organisation", ty = "child")]
    // pub organisation: OrganisationType,
    // #[xml(ns = b"xwas", name = b"zustaendigeBehoerde", ty = "child")]
    // pub zustaendige_behoerde: ZustaendigeBehoerdeType,
    #[xml(ns = b"xwas", name = b"natuerlichePerson")]
    NatuerlichePerson(NatuerlichePersonType),
    #[default]
    #[xml(ns = b"xwas", name = b"unknown")]
    None,

}

/// Klasse zum Transport von Informationen zu einem Betreiber einer WVA [Soweit möglich
/// in Register zu pflegen].
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"BetreiberType")]
pub struct BetreiberType {
    #[xml(ns = b"xwas", name = b"betreiberID", ty = "child")]
    pub betreiber_id: String,
    #[xml(ns = b"xwas", name = b"artDerPerson", ty = "child")]
    pub art_der_person: ArtDerPerson, // enum .. heisst eigentlich ArtDerPerson
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
}

/// Klasse für den Transport von Informationen zu einem Objekt.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"ObjektType")]
pub struct ObjektType {
    #[xml(ns = b"xwas", name = b"objektID", ty = "child")]
    pub objekt_id: String,
    #[xml(ns = b"xwas", name = b"wasserversorgungsgebiet", ty = "child")]
    pub wasserversorgungsgebiet: Option<String>,
    #[xml(ns = b"xwas", name = b"anschriftObjekt", ty = "child")]
    pub anschrift_objekt: Vec<AnschriftType>,
    #[xml(ns = b"xwas", name = b"artObjekt", ty = "child")]
    pub art_objekt: CodeArtObjektType,
    #[xml(ns = b"xwas", name = b"nameObjekt", ty = "child")]
    pub name_objekt: String,
    #[xml(ns = b"xwas", name = b"betriebszustandDesObjekts", ty = "child")]
    pub betriebszustand_des_objekts: Option<CodeBetriebszustandType>,
    #[xml(ns = b"xwas", name = b"datumInBetriebnahme", ty = "child")]
    pub datum_in_betriebnahme: Option<String>,
    #[xml(ns = b"xwas", name = b"datumAusserBetriebnahme", ty = "child")]
    pub datum_ausser_betriebnahme: Option<String>,
    #[xml(ns = b"xwas", name = b"rahmenDerTrinkwasserbereitstellung", ty = "child")]
    pub rahmen_der_trinkwasserbereitstellung: Vec<CodeRahmenTrinkwasserbereitstellungType>,
    #[xml(ns = b"xwas", name = b"geokoordinatenObjekt", ty = "child")]
    pub geokoordinaten_objekt: GeokoordinatenShapthType,
    #[xml(ns = b"xwas", name = b"altID", ty = "child")]
    pub alt_id: Option<String>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
    #[xml(ns = b"xwas", name = b"betreiber", ty = "child")]
    pub betreiber: Vec<BetreiberType>,
    #[xml(ns = b"xwas", name = b"objekt_probennahmestelle", ty = "child")]
    pub objekt_probennahmestelle: Vec<ProbennahmestelleType>,
}


/// Klasse für den Transport von Informationen zu einer Trinkwasserversorgungsanlage.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"AnlageNachTrinkwVType")]
pub struct AnlageNachTrinkwVType {
    #[xml(ns = b"xwas", name = b"anlageNachTrinkwVID", ty = "child")]
    pub anlage_nach_trinkw_vid: String,
    #[xml(ns = b"xwas", name = b"zustaendigeBehoerdeID", ty = "child")]
    pub zustaendige_behoerde_id: String,
    #[xml(ns = b"xwas", name = b"untersuchungsplanID", ty = "child")]
    pub untersuchungsplan_id: Vec<String>,
    #[xml(ns = b"xwas", name = b"artAnlage", ty = "child")]
    pub art_anlage: CodeArtTrinkwasseranlageType,
    #[xml(ns = b"xwas", name = b"nameDerAnlage", ty = "child")]
    pub name_der_anlage: String,
    #[xml(ns = b"xwas", name = b"abgegebeneWassermengeDerAnlageProTag", ty = "child")]
    pub abgegebene_wassermenge_der_anlage_pro_tag: Option<f64>,
    #[xml(ns = b"xwas", name = b"anzahlDurchAnlageVersorgtePersonen", ty = "child")]
    pub anzahl_durch_anlage_versorgte_personen: Option<u32>,
    #[xml(ns = b"xwas", name = b"altID", ty = "child")]
    pub alt_id: Option<String>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
    #[xml(ns = b"xwas", name = b"wasserversorgungsgebiet", ty = "child")]
    pub wasserversorgungsgebiet: Vec<WasserversorgungsgebietType>,
    #[xml(ns = b"xwas", name = b"anlageNachTrinwV_Objekt", ty = "child")]
    pub anlage_nach_trinw_v_objekt: Vec<ObjektType>,
}


/// Klasse für den Transport von Informationen, die für die Erstellung eines
/// Untersuchungsplans für a- und b-Anlagen relevant sind.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"UntersuchungsplanType")]
pub struct UntersuchungsplanType {
    #[xml(ns = b"xwas", name = b"untersuchungsplanID", ty = "child")]
    pub untersuchungsplan_id: String,
    #[xml(ns = b"xwas", name = b"wasserversorgungsgebiet", ty = "child")]
    pub wasserversorgungsgebiet: Vec<String>,
    #[xml(ns = b"xwas", name = b"jahr", ty = "child")]
    pub jahr: Vec<String>, // JahrType
    #[xml(ns = b"xwas", name = b"wasserabgabeVorjahr", ty = "child")]
    pub wasserabgabe_vorjahr: f64,
    #[xml(ns = b"xwas", name = b"artVonWVAundWVG", ty = "child")]
    pub art_von_wva_und_wvg: CodeWVAType,
    #[xml(ns = b"xwas", name = b"erlaeuterungZurWasserabgabemenge", ty = "child")]
    pub erlaeuterung_zur_wasserabgabemenge: CodeErlaeuterungWasserabgabemengeType,
    #[xml(ns = b"xwas", name = b"flockung", ty = "child")]
    pub flockung: CodeFlockungType,
    #[xml(ns = b"xwas", name = b"oberflaechenwassereinfluss", ty = "child")]
    pub oberflaechenwassereinfluss: bool,
    #[xml(ns = b"xwas", name = b"desinfektionDurchgefuehrtMit", ty = "child")]
    pub desinfektion_durchgefuehrt_mit: CodeDesinfektionsartType,
    #[xml(ns = b"xwas", name = b"abfuellungZurAbgabeInVerschlossenenBehaeltnissen", ty = "child")]
    pub abfuellung_zur_abgabe_in_verschlossenen_behaeltnissen: bool,
    #[xml(ns = b"xwas", name = b"acrylamid", ty = "child")]
    pub acrylamid: CodeNachweisartType,
    #[xml(ns = b"xwas", name = b"epichlorhydrin", ty = "child")]
    pub epichlorhydrin: CodeNachweisartType,
    #[xml(ns = b"xwas", name = b"vinylchlorid", ty = "child")]
    pub vinylchlorid: CodeNachweisartType,
    #[xml(ns = b"xwas", name = b"phWertWasserwerksausgang", ty = "child")]
    pub ph_wert_wasserwerksausgang: bool,
    #[xml(ns = b"xwas", name = b"wasserabgabeVorjahrProTag", ty = "child")]
    pub wasserabgabe_vorjahr_pro_tag: f64,
    #[xml(ns = b"xwas", name = b"anzahlUntersuchungenproJahrGruppeA", ty = "child")]
    pub anzahl_untersuchungenpro_jahr_gruppe_a: u32,
    #[xml(ns = b"xwas", name = b"abzudeckenDurchBetreiberGruppeA", ty = "child")]
    pub abzudecken_durch_betreiber_gruppe_a: Option<u32>,
    #[xml(ns = b"xwas", name = b"anzahlUntersuchungenproJahrGruppeB", ty = "child")]
    pub anzahl_untersuchungenpro_jahr_gruppe_b: u32,
    #[xml(ns = b"xwas", name = b"abzudeckenDurchBetreiberGruppeB", ty = "child")]
    pub abzudecken_durch_betreiber_gruppe_b: Option<u32>,
    #[xml(ns = b"xwas", name = b"rapDurchgefuehrt", ty = "child")]
    pub rap_durchgefuehrt: bool,
    #[xml(ns = b"xwas", name = b"statusUntersuchungsplan", ty = "child")]
    pub status_untersuchungsplan: CodeStatusUntersuchungsplanType,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
    #[xml(ns = b"xwas", name = b"terminplan", ty = "child")]
    pub terminplan: Vec<TerminplanType>,
    #[xml(ns = b"xwas", name = b"anlageNachTrinkwV", ty = "child")]
    pub anlage_nach_trinkw_v: AnlageNachTrinkwVType,
    #[xml(ns = b"xwas", name = b"auftraggeber", ty = "child")]
    pub auftraggeber: Auftraggeber,
    #[xml(ns = b"xwas", name = b"zustaendigeBehoerde", ty = "child")]
    pub zustaendige_behoerde: ZustaendigeBehoerdeType,
    #[xml(ns = b"xwas", name = b"probe_Rel", ty = "child")]
    pub probe_rel: Vec<ProbeType>,
    #[xml(ns = b"xwas", name = b"erweiterung", ty = "child")]
    pub erweiterung: Option<ErweiterungType>,
}


#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeAuftraggeberartType")]
pub struct CodeAuftraggeberartType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

/// Informationen zu einem Auftraggeber [Ergänzende Angaben zu den jeweiligen
/// Informationen aus den Registern von Betreibern/Behörden].
// #[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(root = b"auftraggeber")]
// pub struct Auftraggeber {
//     // #[xml(ns = b"xwas", name = b"organisation", ty = "child")]
//     // pub organisation: OrganisationType,
//     #[xml(ns = b"xwas", name = b"natuerlichePerson", ty = "child")]
//     pub natuerliche_person: NatuerlichePersonType,
//     // #[xml(ns = b"xwas", name = b"zustaendigeBehoerde", ty = "child")]
//     // pub zustaendige_behoerde: ZustaendigeBehoerdeType,

// }

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[serde(tag = "t", content = "c")]
pub enum Auftraggeber {
    #[xml(ns = b"xwas", name = b"organisation")]
    Organisation(OrganisationType),
    #[xml(ns = b"xwas", name = b"natuerlichePerson")]
    NatuerlichePerson(NatuerlichePersonType),
    #[xml(ns = b"xwas", name = b"zustaendigeBehoerde")]
    ZustaendigeBehoerde(ZustaendigeBehoerdeType),
    #[default]
    #[xml(ns = b"xwas", name = b"unknown")]
    None,
}

/// Informationen zu einem Auftraggeber [Ergänzende Angaben zu den jeweiligen
/// Informationen aus den Registern von Betreibern/Behörden].
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct AuftraggeberType {
    #[xml(ns = b"xwas", name = b"auftraggeberID", ty = "child")]
    pub auftraggeber_id: String,
    #[xml(ns = b"xwas", name = b"auftraggeberart", ty = "child")]
    pub auftraggeberart: CodeAuftraggeberartType,
    #[xml(ns = b"xwas", name = b"auftraggeber", ty = "child")]
    pub auftraggeber: Auftraggeber,
}


#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct CodeNameBeauftragteUntersuchungsstelle {
    #[xml(name = b"code", ty = "child")]
    pub code: String,
    #[xml(name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeLaenderkennzeichenType")]
pub struct CodeLaenderkennzeichenType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}


/// Klasse für den Transport von Informationen zu einer Zuständigen Behörde [Ergänzende
/// Angaben zu einer im Register Behörden gepflegte Behörde].
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"ZustaendigeBehoerdeType")]
pub struct ZustaendigeBehoerdeType {
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub behoerde: BehoerdeType,
    #[xml(ns = b"xwas", name = b"anlageNachTrinkwVID", ty = "child")]
    pub anlage_nach_trinkw_vid: Vec<String>,
    #[xml(ns = b"xwas", name = b"probennehmerID", ty = "child")]
    pub probennehmer_id: Vec<String>,
    #[xml(ns = b"xwas", name = b"laenderkuerzel", ty = "child")]
    pub laenderkuerzel: CodeLaenderkennzeichenType,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
}


/// Der AllgemeineName dient der Darstellung von Vor- und Nachnamen und fasst deren
/// gemeinsame Eigenschaften zusammen.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"AllgemeinerNameType")]
pub struct AllgemeinerNameType {
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
    #[xml(ns = b"xwas", name = b"nichtVorhanden", ty = "child")]
    pub nicht_vorhanden: Option<bool>,
    #[xml(ns = b"xwas", name = b"namensart", ty = "child")]
    pub namensart: Option<Code>,
    #[xml(ns = b"xwas", name = b"alternativeRepraesentation", ty = "child")]
    pub alternative_repraesentation: Vec<AlternativeRepraesentationType>,
}

/// Die "AlternativeRepraesentation" beinhaltet das mit ihm verbundene Objekt in einer
/// alternativen Form, die einer festgelegten Konvention folgt. Das Element kann Inhalte
/// anderer Elemente des verbundenen Objekts beinhalten. Die in der Komponente
/// "AlternativeRepraesentation" übermittelten Informationen müssen redundant zu den
/// anderen Elementen des mit ihm verbundenen Objekts sein. Eine
/// "AlternativeRepraesentation" kann auch eine multimediale Abbildung des Objektes
/// darstellen. Hierzu zählen beispielsweise Logos oder Bilder. Beispiel: Ein Beispiel
/// für die Verwendung einer alternativen Repraesentation ist die Übermittlung von Namen.
/// Der Name "Andrè Müller" würde nach ICAO-Standard, in dem keine Umlaute erlaubt sind,
/// daher alternativ als "ANDRE MUELLER" übertragen.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"AlternativeRepraesentationType")]
pub struct AlternativeRepraesentationType {
    #[xml(ns = b"xwas", name = b"repraesentation", ty = "child")]
    pub repraesentation: String,
    #[xml(ns = b"xwas", name = b"algorithmus", ty = "child")]
    pub algorithmus: Option<String>,
    #[xml(ns = b"xwas", name = b"hinweis", ty = "child")]
    pub hinweis: Option<String>,
}

/// Der Name einer Person ist eine Benennung dieser Person, die dazu dient, diese Person
/// von anderen Personen zu unterscheiden.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"nameNatuerlichePersonType")]
pub struct NameNatuerlichePersonType {
    #[xml(name = b"titel", ty = "child")]
    pub titel: Option<String>,
    #[xml(name = b"anrede", ty = "child")]
    pub anrede: Vec<String>,
    #[xml(name = b"namenssuffix", ty = "child")]
    pub namenssuffix: Vec<String>,
    #[xml(ns = b"xwas", name = b"familienname", ty = "child")]
    pub familienname: Option<AllgemeinerNameType>,
    #[xml(name = b"ehename", ty = "child")]
    pub ehename: Option<AllgemeinerNameType>,
    #[xml(name = b"lebenspartnerschaftsname", ty = "child")]
    pub lebenspartnerschaftsname: Option<AllgemeinerNameType>,
    #[xml(name = b"geburtsname", ty = "child")]
    pub geburtsname: Option<AllgemeinerNameType>,
    #[xml(name = b"fruehererFamilienname", ty = "child")]
    pub frueherer_familienname: Vec<AllgemeinerNameType>,
    #[xml(ns = b"xwas", name = b"vorname", ty = "child")]
    pub vorname: Option<AllgemeinerNameType>,
    #[xml(name = b"rufname", ty = "child")]
    pub rufname: Option<AllgemeinerNameType>,
    #[xml(name = b"fruehererVorname", ty = "child")]
    pub frueherer_vorname: Option<AllgemeinerNameType>,
    #[xml(name = b"alternativeRepraesentation", ty = "child")]
    pub alternative_repraesentation: Option<AlternativeRepraesentationType>,
    #[xml(name = b"ordensname", ty = "child")]
    pub ordensname: Option<AllgemeinerNameType>,
    #[xml(name = b"kuenstlername", ty = "child")]
    pub kuenstlername: Vec<AllgemeinerNameType>,
    #[xml(name = b"weitererName", ty = "child")]
    pub weiterer_name: Vec<AllgemeinerNameType>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeAuskunftssperreType")]
pub struct CodeAuskunftssperreType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}


/// Der Zeitraum kennzeichnet einen Abschnitt auf einem Zeitstrahl durch Angabe von
/// Beginn und/oder Ende.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"ZeitraumType")]
pub struct ZeitraumType {
    #[xml(ns = b"xwas", name = b"beginn", ty = "child")]
    pub beginn: Option<String>, // eigentlich of tyype xs:date
    #[xml(ns = b"xwas", name = b"ende", ty = "child")]
    pub ende: Option<String>,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    pub zusatz: Option<String>,
}

/// Die Auskunftssperre beschränkt die Weitergabe von Informationen an Dritte.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"AuskunftssperreType")]
pub struct AuskunftssperreType {
    #[xml(ns = b"xwas", name = b"grund", ty = "child")]
    pub grund: Option<CodeAuskunftssperreType>,
    #[xml(ns = b"xwas", name = b"gueltigkeit", ty = "child")]
    pub gueltigkeit: Option<ZeitraumType>,
}


#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeFamilienstandType")]
pub struct CodeFamilienstandType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}


/// Unter "Geburt" werden geburtsbezogene Informationen zusammengefasst.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"Geburt")]
pub struct GeburtType {
    #[xml(ns = b"xwas", name = b"datum", ty = "child")]
    pub datum: Option<String>,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    pub zusatz: Option<String>,
    #[xml(ns = b"xwas", name = b"geburtsort", ty = "child")]
    pub geburtsort: Option<AnschriftType>,
}

/// Dieser Datentyp erlaubt die Angabe von Doktorgraden. Es sind nur diejenigen
/// Doktorgrade anzugeben, die in Pässe eingetragen werden dürfen. Sind mehrere
/// Doktorgrade anzugeben, so sind sie durch ein Leerzeichen zu trennen. Zulässig sind
/// derzeit: „DR.“, „Dr.“, „DR.HC.“, „Dr.hc.“, „Dr.EH.“ und „Dr.eh.“.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"DoktorgradType")]
pub struct DoktorgradType {
    // hier gibt es eine striction mit max length 120, wie umsetzen ?
    #[xml(ns = b"xwas", name = b"bezeichnung", ty = "child")]
    pub bezeichnung: String,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeStaatsangehoerigkeitType")]
pub struct CodeStaatsangehoerigkeitType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeAusweisdokumenteType")]
pub struct CodeAusweisdokumenteType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeGeschlechtType")]
pub struct CodeGeschlechtType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

/// Unter "Sprache" werden Informationen über Sprachen zusammengefasst.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"SpracheType")]
pub struct SpracheType {
    #[xml(ns = b"xwas", name = b"sprache", ty = "child")]
    pub sprache: String,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    pub zusatz: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeVertretungsartType")]
pub struct CodeVertretungsartType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,

}


/// Mit diesem Datentyp wird ein gesetzlicher Vertreter oder ein Bevollmächtigter einer
/// nichtnatürlichen Person abgebildet.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"VertreterBevollmaechtigterType")]
pub struct VertreterBevollmaechtigterType {
    #[xml(ns = b"xwas", name = b"vertreterBevollmaechtigterID", ty = "child")]
    pub vertreter_bevollmaechtigter_id: String,
    #[xml(ns = b"xwas", name = b"artVertreter", ty = "child")]
    pub art_vertreter: CodeVertretungsartType,
}


/// Eine natürliche Person ist der Mensch in seiner Rolle als Rechtssubjekt, d. h. als
/// Träger von Rechten und Pflichten. Mit der Vollendung seiner Geburt wird ein Mensch
/// rechtsfähig und damit zu einer natürlichen Person (§ 1 BGB). Der Mensch verliert
/// seine Rechtsfähigkeit mit dem Tod. Rechtssubjekte, die keine natürlichen Personen
/// sind, nennt man juristische Personen.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"natuerlichePersonType")]
pub struct NatuerlichePersonType {
    #[xml(name = b"auskunftssperre", ty = "child")]
    pub auskunftssperre: Vec<AuskunftssperreType>,
    #[xml(ns = b"xwas", name = b"nameNatuerlichePerson", ty = "child")]
    pub name_natuerliche_person: Option<NameNatuerlichePersonType>,
    #[xml(name = b"familienstand", ty = "child")]
    pub familienstand: Vec<CodeFamilienstandType>,
    #[xml(name = b"geburt", ty = "child")]
    pub geburt: Option<GeburtType>,
    #[xml(name = b"doktorgrad", ty = "child")]
    pub doktorgrad: Option<DoktorgradType>,
    #[xml(name = b"staatsangehoerigkeit", ty = "child")]
    pub staatsangehoerigkeit: Vec<CodeStaatsangehoerigkeitType>,
    #[xml(name = b"ausweisdokument", ty = "child")]
    pub ausweisdokument: Vec<CodeAusweisdokumenteType>,
    #[xml(name = b"anschrift", ty = "child")]
    pub anschrift: Vec<AnschriftType>,
    #[xml(name = b"geschlecht", ty = "child")]
    pub geschlecht: Vec<CodeGeschlechtType>,
    #[xml(name = b"identifikationsnummer", ty = "child")]
    pub identifikationsnummer: Vec<IdentifikationType>,
    #[xml(name = b"kommunikation", ty = "child")]
    pub kommunikation: Vec<KommunikationType>,
    #[xml(name = b"muttersprache", ty = "child")]
    pub muttersprache: Vec<SpracheType>,
    #[xml(name = b"fremdsprache", ty = "child")]
    pub fremdsprache: Vec<SpracheType>,
    #[xml(name = b"vertreterBevollmaechtigter", ty = "child")]
    pub vertreter_bevollmaechtigter: Vec<VertreterBevollmaechtigterType>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(tns(b"xwas", b"xwasser"))]
pub struct ProbennehmerType {
    // #[xml(ns = b"xwas", name = b"organisation", ty = "child")]
    // pub organisation: OrganisationType,
    #[xml(ns = b"xwas", name = b"natuerlichePerson", ty = "child")]
    pub natuerliche_person: NatuerlichePersonType,
    // #[xml(ns = b"xwas", name = b"zustaendigeBehoerde", ty = "child")]
    // pub zustaendige_behoerde: ZustaendigeBehoerdeType,
}

/// Klasse für den Transport von Informationen zu einem Probennehmer [Durch das Labor mit
/// dem Prüfbericht mit zu übermittelnde Informationen].
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"ProbennehmerType")]
pub struct Probennehmer {
    #[xml(ns = b"xwas", name = b"probennehmerID", ty = "child")]
    pub probennehmer_id: String,
    #[xml(ns = b"xwas", name = b"probennehmer", ty = "child")]
    pub probennehmer: ProbennehmerType,
    #[xml(ns = b"xwas", name = b"fremdsystemID_Probennehmer", ty = "child")]
    pub fremdsystem_id_probennehmer: Option<String>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
    #[xml(ns = b"xwas", name = b"_id", ty = "attr")]
    #[serde(skip)]
    pub _id: ConstStr,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeProbenentnahmegeraetType")]
pub struct CodeProbenentnahmegeraetType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeGefaessType")]
pub struct CodeProbengefaessType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}



#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeProbenbewertungType")]
pub struct CodeProbenbewertungType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Option<String>,
}


/// Klasse zum Transport von Informationen, welche über eine Probe vorliegen sollen, die
/// im Rahmen eines Prüfberichts via SHAPTH übermittelt wird.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"Probe")]
pub struct ProbeType {
    #[xml(ns = b"xwas", name = b"probeID", ty = "child")]
    pub probe_id: String,
    #[xml(ns = b"xwas", name = b"analyseergebnisParameter", ty = "child")]
    pub analyseergebnis_parameter: Vec<AnalyseergebnisParameter>,
    #[xml(ns = b"xwas", name = b"probennehmer", ty = "child")]
    pub probennehmer: Probennehmer,
    #[xml(ns = b"xwas", name = b"anlassDerUntersuchung", ty = "child")]
    pub anlass_der_untersuchung: Vec<CodeAnlassUntersuchungType>,
    #[xml(ns = b"xwas", name = b"medium", ty = "child")]
    pub medium: CodeMediumType,
    #[xml(name = b"ergaenzungZumMedium", ty = "child")]
    pub ergaenzung_zum_medium: Option<String>,
    #[xml(ns = b"xwas", name = b"zeitpunktProbennahme", ty = "child")]
    pub zeitpunkt_probennahme: String, //datetime
    #[xml(ns = b"xwas", name = b"probennahmeverfahren", ty = "child")]
    pub probennahmeverfahren: Vec<CodeProbennahmeverfahrenType>,
    #[xml(name = b"probenentnahmegeraet", ty = "child")]
    pub probenentnahmegeraet: Option<CodeProbenentnahmegeraetType>,
    #[xml(name = b"probengefaess", ty = "child")]
    pub probengefaess: Option<CodeProbengefaessType>,
    #[xml(name = b"ergaenzendeInformationenZuProbenentnahmegeraet", ty = "child")]
    pub ergaenzende_informationen_zu_probenentnahmegeraet: Option<String>,
    #[xml(name = b"desinfektionProbenentnahmegeraetDurchgefuehrt", ty = "child")]
    pub desinfektion_probenentnahmegeraet_durchgefuehrt: Option<bool>,
    #[xml(name = b"konservierungAufbereitungDesinfektionProbe", ty = "child")]
    pub konservierung_aufbereitung_desinfektion_probe: Vec<CodeAufbereitungsstoffDesinfektionsverfahrenType>,
    #[xml(ns = b"xwas", name = b"kommentarZurProbennahme", ty = "child")]
    pub kommentar_zur_probennahme: String,
    #[xml(name = b"informationenZumProbentransport", ty = "child")]
    pub informationen_zum_probentransport: Option<String>,
    #[xml(ns = b"xwas", name = b"eingangProbeBeiUntersuchungsstelle", ty = "child")]
    pub eingang_probe_bei_untersuchungsstelle: String, //xs dateTime
    #[xml(ns = b"xwas", name = b"beginnAnalytik", ty = "child")]
    pub beginn_analytik: String, //datetime
    #[xml(ns = b"xwas", name = b"abschlussAnalytik", ty = "child")]
    pub abschluss_analytik: String,
    #[xml(ns = b"xwas", name = b"probenbewertung", ty = "child")]
    pub probenbewertung: CodeProbenbewertungType,
    #[xml(ns = b"xwas", name = b"analyseergebnisParameterID", ty = "child")]
    pub analyseergebnis_parameter_id: String,
    #[xml(name = b"berichtspflichtig", ty = "child")]
    pub berichtspflichtig: Option<bool>,
    #[xml(ns = b"xwas", name = b"vonProbennehmerVergebeneProbeID", ty = "child")]
    pub von_probennehmer_vergebene_probe_id: String,
    #[xml(ns = b"xwas", name = b"probeID_ausLabor", ty = "child")]
    pub probe_id_aus_labor: String,
    #[xml(name = b"anhang", ty = "child")]
    pub anhang: Vec<String>,
    #[xml(name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
    #[xml(ns = b"xwas", name = b"probennehmerID", ty = "attr")]
    #[serde(skip)]
    pub _id: ConstStr,
}

#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[serde(tag = "t", content = "c")]
pub enum VorgangType {
    #[xml(ns = b"xwas", name = b"pruefbericht")]
    Pruefbericht(PruefberichtType),
    #[xml(ns = b"xwas", name = b"untersuchungsplan")]
    Untersuchungsplan(UntersuchungsplanType),
    // #[xml(ns = b"xwas", name = b"olgBericht", ty = "child")]
    // OlgBericht(String),
    #[default]
    #[xml(ns = b"xwas", name = b"unknown")]
    None,
}

extern crate raxb as _raxb;


#[wasm_bindgen]
pub fn create_quality_report_xml(data: QualityReport) -> Result<String, JsValue> {
    Ok(raxb::ser::to_string_pretty_with_decl(&data)
        .map_err(|err| JsValue::from_str(&err.to_string()))?)
}

#[wasm_bindgen]
pub fn parse_quality_report_xml(xml: String) -> Result<QualityReport, JsValue> {
    Ok(raxb::de::from_str(&xml).map_err(|err| JsValue::from_str(&err.to_string()))?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_auftraggeber() {
        let v = AuftraggeberType {
            auftraggeber: Auftraggeber::NatuerlichePerson(NatuerlichePersonType {
                ..Default::default()
            }),
            ..Default::default()
        };
        eprintln!("{}", serde_json::to_string_pretty(&v).unwrap());
        
        // serde_json::json!({

        // })
    }

    #[test]
    fn test_quality_report() {
        let v = serde_json::from_str::<QualityReport>(include_str!("../fixtures/simple_test.json"));
        eprintln!("{v:#?}");

        // serde_json::json!({

        // })
    }
}