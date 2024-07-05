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
    pub nachrichten_uuid: String, //ConstStr,
    #[xml(name = b"nachrichtentyp", ty = "child")]
    pub nachrichten_typ: NachrichtenTyp,
    #[xml(name = b"erstellungszeitpunkt", ty = "child")]
    pub erstellungszeitpunkt: String,
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

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(root = b"kennung")]
// pub struct Kennung {
//     #[xml(name = b"kennung", ty = "child")]
//     pub kennung: String,
// }

/// Unter "Identifikation" werden die Informationen zusammengefasst, die die eindeutige
/// Identifikation von Objekten in einem fachlichen Kontext erlauben.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"IdentifikationType")]
pub struct IdentifikationType {
    #[xml(name = b"id", ty = "child")]
    pub id: Vec<String>,
    #[xml(name = b"beschreibung", ty = "child")]
    pub beschreibung: Vec<String>,
    #[xml(name = b"gueltigkeit", ty = "child")]
    pub gueltigkeit: Vec<Gueltigkeit>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
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
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"vorgang")]
// #[tsify(into_wasm_abi, from_wasm_abi)]
// #[xml(tns(b"xwas", b"xwasser"))]
pub struct Vorgang {
    #[xml(ns = b"xwas", name = b"identifikationVorgang", ty = "child")]
    pub identifikation_vorgang: IdentifikationVorgang,
    #[xml(ns = b"xwas", name = b"vorgangType", ty = "child")]
    pub vorgang_type: VorgangType,
    // #[xml(name = b"bemerkung", ty = "child")]
    // pub bemerkung: Vec<String>,
    // #[xml(name = b"erweiterung", ty = "child")]
    // pub erweiterung: Vec<Erweiterung>,
    // /// Zum Vorgang gehörige Unterlage(n).
    // #[xml(name = b"anlage", ty = "child")]
    // pub anlage: Vec<Anlage>,
}

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(root = b"erstellungszeitpunkt")]
// pub struct Erstellungszeitpunkt {
//     #[xml(name = b"erstellungszeitpunkt", ty = "child")]
//     pub erstellungszeitpunkt: String,
// }

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(root = b"nachrichtenUUID")]
// pub struct NachrichtenUUID {
//     #[xml(name = b"nachrichtenUUID", ty = "child")]
//     pub nachrichten_uuid: String,
// }

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(root = b"erreichbarkeit")]
// pub struct Erreichbarkeit {
//     #[xml(name = b"kanal", ty = "child")]
//     pub kanal: Kanal,
//     #[xml(name = b"kennung", ty = "child")]
//     pub kennung: Kennung,
//     #[xml(name = b"zusatz", ty = "child")]
//     pub zusatz: Zusatz,
// }

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(root = b"zusatz")]
// pub struct Zusatz {
//     #[xml(name = b"zusatz", ty = "child")]
//     pub zusatz: String,
// }

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(root = b"kanal")]
// pub struct Kanal {
//     #[xml(name = b"listVersionID", ty = "attr", value = "1")]
//     #[serde(skip)]
//     pub _list_version_id: ConstStr,
//     #[xml(name = b"code", ty = "child")]
//     pub code: Code,
//     #[xml(name = b"name", ty = "child")]
//     pub name: String,
// }

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(root = b"DvdvDienstkennung")]
// pub struct DvdvDienstkennung {
//     #[xml(name = b"dvdvDienstkennung", ty = "child")]
//     pub dvdv_dienstkennung: String,
// }

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(root = b"ReferenzUUID")]
// pub struct ReferenzUUID {
//     #[xml(name = b"referenzUUID", ty = "child")]
//     pub referenz_uuid: String,
// }

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
    pub untersuchungsverfahren: Untersuchungsverfahren,
    // #[xml(
    //     ns = b"xwas",
    //     name = b"ergaenzungZumUntersuchungsverfahren",
    //     ty = "child"
    // )]
    // pub ergaenzung_zum_untersuchungsverfahren: String,
    #[xml(ns = b"xwas", name = b"untersuchterParameter", ty = "child")]
    pub untersuchter_parameter: UntersuchterParameter,
    // #[xml(ns = b"xwas", name = b"parameterauspraegung", ty = "child")]
    // pub parameterauspraegung: Parameterauspraegung,
    // #[xml(ns = b"xwas", name = b"parameterUnterauswahl", ty = "child")]
    // pub parameter_unterauswahl: ParameterUnterauswahl,
    // #[xml(
    //     ns = b"xwas",
    //     name = b"sensorischerParameterIstAnnehmbar",
    //     ty = "child"
    // )]
    // pub sensorischer_parameter_ist_annehmbar: bool,
    // #[xml(ns = b"xwas", name = b"untersuchungswertParameter", ty = "child")]
    // pub untersuchungswert_parameter: String,
    // #[xml(ns = b"xwas", name = b"einheitDesUntersuchungswerts", ty = "child")]
    // pub einheit_des_untersuchungswerts: EinheitDesUntersuchungswerts,
    // #[xml(
    //     ns = b"xwas",
    //     name = b"ergaenzungZumUntersuchungswertParameter",
    //     ty = "child"
    // )]
    // pub ergaenzung_zum_untersuchungswert_parameter: ErgaenzungZumUntersuchungswertParameter,
    // #[xml(ns = b"xwas", name = b"parameterwertErgaenzung", ty = "child")]
    // pub parameterwert_ergaenzung: String,
    // #[xml(ns = b"xwas", name = b"ausgewertetesAnsatzvolumen", ty = "child")]
    // pub ausgewertetes_ansatzvolumen: String,
    // #[xml(ns = b"xwas", name = b"shapthParameterNummer", ty = "child")]
    // pub shapth_parameter_nummer: String,
    #[xml(ns = b"xwas", name = b"bewertungUntersuchungswert", ty = "child")]
    pub bewertung_untersuchungswert: BewertungUntersuchungswert,
    // #[xml(ns = b"xwas", name = b"parameterauffaelligkeit", ty = "child")]
    // pub parameterauffaelligkeit: String,
    // #[xml(
    //     ns = b"xwas",
    //     name = b"messunsicherheitUntersuchungswert",
    //     ty = "child"
    // )]
    // pub messunsicherheitUntersuchungswert: String, // not null
    // #[xml(ns = b"xwas", name = b"bestimmungsgrenzeLoQ", ty = "child")]
    // pub bestimmungsgrenzeLoQ: String, // erfüllung von anderem feld abhängig
    // #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    // pub kommentar: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct Untersuchungsverfahren {
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    // #[xml(name = b"name", ty = "child")]
    // pub name: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct UntersuchterParameter {
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    // #[xml(name = b"name", ty = "child")]
    // pub name: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct BewertungUntersuchungswert {
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    // #[xml(name = b"name", ty = "child")]
    // pub name: String,
}

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// pub struct ErgaenzungZumUntersuchungswertParameter {
//     #[xml(name = b"code", ty = "child")]
//     pub code: Code,
//     #[xml(name = b"name", ty = "child")]
//     pub name: String,
// }

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// pub struct EinheitDesUntersuchungswerts {
//     #[xml(name = b"code", ty = "child")]
//     pub code: Code,
//     #[xml(name = b"name", ty = "child")]
//     pub name: String,
// }

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// pub struct ParameterUnterauswahl {
//     #[xml(name = b"code", ty = "child")]
//     pub code: Code,
//     #[xml(name = b"name", ty = "child")]
//     pub name: String,
// }

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// pub struct Parameterauspraegung {
//     #[xml(name = b"code", ty = "child")]
//     pub code: Code,
//     #[xml(name = b"name", ty = "child")]
//     pub name: String,
// }


#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct Probenbewertung {
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    // #[xml(name = b"name", ty = "child")]
    // pub name: String,
}

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// pub struct KonservierungAufbereitungDesinfektionProbe {
//     #[xml(name = b"code", ty = "child")]
//     pub code: Code,
//     #[xml(name = b"name", ty = "child")]
//     pub name: String,
// }

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// pub struct Probengefaess {
//     #[xml(name = b"code", ty = "child")]
//     pub code: Code,
//     #[xml(name = b"name", ty = "child")]
//     pub name: String,
// }

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// pub struct Probennahmegeraet {
//     #[xml(name = b"code", ty = "child")]
//     pub code: Code,
//     #[xml(name = b"name", ty = "child")]
//     pub name: String,
// }

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeProbennahmeverfahrenType")]
pub struct CodeProbennahmeverfahrenType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}


#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeMediumType")]
pub struct CodeMediumType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}



#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeAnlassUntersuchungType")]
pub struct CodeAnlassUntersuchungType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}




// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// pub struct ZustaendigeBehoerde {
//     #[xml(name = b"id", ty = "child")]
//     pub id: String,
//     #[xml(name = b"zusatz", ty = "child")]
//     pub zusatz: String,
//     #[xml(name = b"anlageNachTrinkwVID", ty = "child")]
//     pub anlageNachTrinkwVID: String,
//     #[xml(name = b"probennehmerID", ty = "child")]
//     pub probennehmerID: String,
//     #[xml(name = b"pruefberichtID", ty = "child")]
//     pub pruefberichtID: String,
//     #[xml(name = b"laenderkuerzel", ty = "child")]
//     pub laenderkuerzel: Laenderkuerzel,
//     #[xml(name = b"kommentar", ty = "child")]
//     pub kommentar: String,
// }

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

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeRechtsformenType")]
pub struct CodeRechtsformenType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}

/// "NameOrganisation" fasst die Angaben zum Namen einer Organisation zusammen.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"NameOrganisationType")]
pub struct NameOrganisationType {
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
    #[xml(ns = b"xwas", name = b"kurzbezeichnung", ty = "child")]
    pub kurzbezeichnung: Vec<String>,
    #[xml(ns = b"xwas", name = b"gueltigkeit", ty = "child")]
    pub gueltigkeit: Vec<Gueltigkeit>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeKommunikationType")]
pub struct CodeKommunikationType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}

/// "Kommunikation" fasst Angaben zur Erreichbarkeit über elektronische
/// Kommunikationskanäle (z.B. Telefon, Fax, E-Mail) zusammen.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"KommunikationType")]
pub struct KommunikationType {
    #[xml(ns = b"xwas", name = b"kanal", ty = "child")]
    pub kanal: Vec<CodeKommunikationType>,
    #[xml(ns = b"xwas", name = b"kennung", ty = "child")]
    pub kennung: Vec<String>,
    #[xml(ns = b"xwas", name = b"istDienstlich", ty = "child")]
    pub ist_dienstlich: Vec<bool>,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    pub zusatz: Vec<String>,
}


#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeRegisterType")]
pub struct CodeRegisterType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeBehoerdeType")]
pub struct CodeBehoerdeType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}

/// Die Organisationseinheit fasst Angaben zur Darstellung der internen hierarchischen
/// Organisationsstruktur einer Institution zusammen, z.B. zur Darstellung von
/// Abteilungen oder Referaten.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"organisationseinheitType")]
pub struct OrganisationseinheitType {
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: String,
    #[xml(ns = b"xwas", name = b"hierarchieebene", ty = "child")]
    pub hierarchieebene: Vec<u8>,
    #[xml(ns = b"xwas", name = b"hierarchiename", ty = "child")]
    pub hierarchiename: Vec<String>,
}

/// Die zu verwendende Schlüsseltabelle ergibt sich aus dem Nachrichtenkontext. Zum
/// Beispiel ist bei der Datenübermittlung an ein Standesamt die Schlüsseltabelle der
/// Standesamtsnummern und somit eine Standesamtsnummer als Behördenkennung zu verwenden.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeBehoerdenkennungType")]
pub struct CodeBehoerdenkennungType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}

/// Die mit der genutzten Codeliste beschriebenen Präfixe werden für die fachliche
/// Adressierung über das DVDV verwendet.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codePraefixType")]
pub struct CodePraefixType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
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
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
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
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"BehoerdeType")]
pub struct BehoerdeType {
    #[xml(ns = b"xwas", name = b"id", ty = "child")]
    pub id: Vec<String>,
    #[xml(ns = b"xwas", name = b"typ", ty = "child")]
    pub typ: Vec<CodeBehoerdeType>,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    pub zusatz: Vec<String>,
    #[xml(ns = b"xwas", name = b"behoerdenkennung", ty = "child")]
    pub behoerdenkennung: Vec<BehoerdenkennungType>,
    #[xml(ns = b"xwas", name = b"kommunikation", ty = "child")]
    pub kommunikation: Vec<KommunikationType>,
    #[xml(ns = b"xwas", name = b"behoerdenidentifikation", ty = "child")]
    pub behoerdenidentifikation: Vec<IdentifikationType>,
    #[xml(ns = b"xwas", name = b"behoerdenname", ty = "child")]
    pub behoerdenname: Vec<NameOrganisationType>,
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
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"RegistrierungType")]
pub struct RegistrierungType {
    #[xml(ns = b"xwas", name = b"id", ty = "child")]
    pub id: Vec<String>,
    #[xml(ns = b"xwas", name = b"registertyp", ty = "child")]
    pub registertyp: Vec<CodeRegisterType>,
    #[xml(ns = b"xwas", name = b"registrierendeBehoerde", ty = "child")]
    pub registrierende_behoerde: Vec<BehoerdeType>,
    #[xml(ns = b"xwas", name = b"gueltigkeit", ty = "child")]
    pub gueltigkeit: Vec<Gueltigkeit>,
}



/// Eine Organisation ist eine Vereinigung mehrerer natürlicher oder juristischer
/// Personen bzw. eine rechtsfähige Personengesellschaft zu einem gemeinsamen Zweck, z.B.
/// im wirtschaftlichen, gemeinnützigen, religiösen, öffentlichen oder politischen
/// Bereich. Behörden werden über eine eigene Kernkomponente "Behoerde" abgebildet.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"OrganisationType")]
pub struct OrganisationType {
    #[xml(ns = b"xwas", name = b"rechtsform", ty = "child")]
    pub rechtsform: Vec<CodeRechtsformenType>,
    #[xml(ns = b"xwas", name = b"branche", ty = "child")]
    pub branche: Vec<CodeBrancheType>,
    #[xml(ns = b"xwas", name = b"zweck", ty = "child")]
    pub zweck: Vec<CodeZweckType>,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<NameOrganisationType>,
    #[xml(ns = b"xwas", name = b"unterorganisation", ty = "child")]
    pub unterorganisation: Vec<OrganisationType>,
    #[xml(ns = b"xwas", name = b"kommunikation", ty = "child")]
    pub kommunikation: Vec<KommunikationType>,
    #[xml(ns = b"xwas", name = b"registrierung", ty = "child")]
    pub registrierung: Vec<RegistrierungType>,
    #[xml(ns = b"xwas", name = b"identifikation", ty = "child")]
    pub identifikation: Vec<IdentifikationType>,
    #[xml(ns = b"xwas", name = b"existenzzeitraum", ty = "child")]
    pub existenzzeitraum: Vec<ZeitraumType>,
    #[xml(ns = b"xwas", name = b"anschrift", ty = "child")]
    pub anschrift: Vec<AnschriftType>,
}


#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct RegistrierendeBehoerde {
    #[xml(name = b"id", ty = "child")]
    pub id: String, 
}
// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// pub struct NachgeordneteBehoerde {
//     #[xml(name = b"id", ty = "child")]
//     pub id: String,
//     #[xml(name = b"zusatz", ty = "child")]
//     pub zusatz: String,
//     #[xml(name = b"verwaltungspolitischeZustaendigkeit", ty = "child")]
//     pub verwaltungspolitische_zustaendigkeit: VerwaltungspolitischeZustaendigkeit,
//     #[xml(name = b"anschrift", ty = "child")]
//     pub anschrift: Anschrift,
//     #[xml(name = b"organisationsstruktur", ty = "child")]
//     pub organisationsstruktur: Organisationsstruktur,
// }

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// pub struct Organisationsstruktur {
//     #[xml(name = b"name", ty = "child")]
//     pub name: String,
//     #[xml(name = b"hierarchieebene", ty = "child")]
//     pub hierarchieebene: String,
//     #[xml(name = b"hierarchiename", ty = "child")]
//     pub hierarchiename: String,
// }

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"Code.AnschrifttypType")]
pub struct CodeAnschrifttypType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"Code.StaatType")]
pub struct CodeStaatType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}


#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"Code.KreisType")]
pub struct CodeKreisType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"Code.BezirkType")]
pub struct CodeBezirkType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"Code.BundeslandType")]
pub struct CodeBundeslandType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"Code.AGSType")]
pub struct CodeAGSType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"Code.RegionalschluesselType")]
pub struct CodeRegionalschluesselType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}


/// Die Komponente "VerwaltungspolitischeKodierung" beinhaltet Information, die eine
/// verwaltungspolitisch eindeutige Zuordnung ermöglichen.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"VerwaltungspolitischeKodierungType")]
pub struct VerwaltungspolitischeKodierungType {
    #[xml(ns = b"xwas", name = b"kreis", ty = "child")]
    pub kreis: Vec<CodeKreisType>,
    #[xml(ns = b"xwas", name = b"bezirk", ty = "child")]
    pub bezirk: Vec<CodeBezirkType>,
    #[xml(ns = b"xwas", name = b"bundesland", ty = "child")]
    pub bundesland: Vec<CodeBundeslandType>,
    #[xml(ns = b"xwas", name = b"gemeindeschluessel", ty = "child")]
    pub gemeindeschluessel: Vec<CodeAGSType>,
    #[xml(ns = b"xwas", name = b"regionalschluessel", ty = "child")]
    pub regionalschluessel: Vec<CodeRegionalschluesselType>,
    #[xml(ns = b"xwas", name = b"nation", ty = "child")]
    pub nation: Vec<CodeStaatType>,
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
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(root = b"AnschriftType")]
pub struct AnschriftType {
    #[xml(name = b"id", ty = "attr")]
    pub id: String,
    #[xml(name = b"strassenschluessel", ty = "child")]
    pub strassenschluessel: Code,
    #[xml(name = b"strasse", ty = "child")]
    pub strasse: String,
    #[xml(name = b"hausnummer", ty = "child")]
    pub hausnummer: String,
    #[xml(name = b"postfach", ty = "child")]
    pub postfach:  String,
    #[xml(name = b"postleitzahl", ty = "child")]
    pub postleitzahl: String,
    #[xml(name = b"ort", ty = "child")]
    pub ort: String,
    #[xml(name = b"ortsteil", ty = "child")]
    pub ortsteil: Vec<String>,
    #[xml(name = b"ortFruehererGemeindename", ty = "child")]
    pub ort_frueherer_gemeindename: Vec<String>,
    #[xml(name = b"wohnungsgeber", ty = "child")]
    pub wohnungsgeber: Vec<String>,
    #[xml(name = b"zusatz", ty = "child")]
    pub zusatz: Vec<String>,
    #[xml(name = b"typ", ty = "child")]
    pub typ: CodeAnschrifttypType,
    #[xml(name = b"staat", ty = "child")]
    pub staat: Vec<CodeStaatType>,
    #[xml(name = b"verwaltungspolitischeKodierung", ty = "child")]
    pub verwaltungspolitische_kodierung: VerwaltungspolitischeKodierungType,
}


// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// pub struct VerwaltungspolitischeZustaendigkeit {
//     #[xml(name = b"kreis", ty = "child")]
//     pub kreis: Kreis,
//     #[xml(name = b"bezirk", ty = "child")]
//     pub bezirk: Bezirk,
//     #[xml(name = b"bundesland", ty = "child")]
//     pub bundesland: Bundesland,
//     #[xml(name = b"gemeindeschluessel", ty = "child")]
//     pub gemeindeschluessel: Gemeindeschluessel,
//     #[xml(name = b"regionalschluessel", ty = "child")]
//     pub regionalschluessel: Regionalschluessel,
//     #[xml(name = b"nation", ty = "child")]
//     pub nation: Nation,
// }




// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// pub struct Name {
//     #[xml(name = b"name", ty = "child")]
//     pub name: String,
//     #[xml(name = b"kurzbeschreibung", ty = "child")]
//     pub kurzbeschreibung: String,
//     #[xml(name = b"gueltigkeit", ty = "child")]
//     pub gueltigkeit: Gueltigkeit,
// }

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct Gueltigkeit {
    #[xml(name = b"beginn", ty = "child")]
    pub beginn: String,
    #[xml(name = b"ende", ty = "child")]
    pub ende: String,
    #[xml(name = b"zusatz", ty = "child")]
    pub zusatz: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct CodeBrancheType {
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    #[xml(name = b"name", ty = "child")]
    pub name: String,
}

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// pub struct Zweck {
//     #[xml(name = b"code", ty = "child")]
//     pub code: Code,
//     #[xml(name = b"name", ty = "child")]
//     pub name: String,
// }

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct CodeRechtsform {
    #[xml(name = b"listVersionID", ty = "attr")]
    // #[serde(skip)]
    pub list_version_id: Option<String>,
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    // #[xml(name = b"name", ty = "child")]
    // pub name: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct ArtProbennahmestelle {
    // #[xml(name = b"listVersionID", ty = "attr")]
    // pub list_version_id: Option<String>,
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    // #[xml(name = b"name", ty = "child")]
    // pub name: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct MediumAnDerProbennahmestelle {
    // #[xml(name = b"listVersionID", ty = "attr")]
    // pub list_version_id: Option<String>,
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    // #[xml(name = b"name", ty = "child")]
    // pub name: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct Gesamtbewertung {
    // #[xml(name = b"listVersionID", ty = "attr")]
    // pub list_version_id: Option<String>,
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    // #[xml(name = b"name", ty = "child")]
    // pub name: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct SprachePruefbericht {
    // #[xml(name = b"listVersionID", ty = "attr")]
    // pub list_version_id: Option<String>,
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    // #[xml(name = b"name", ty = "child")]
    // pub name: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct AuftraggeberArt {
    // #[xml(name = b"listVersionID", ty = "attr")]
    // pub list_version_id: Option<String>,
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    // #[xml(name = b"name", ty = "child")]
    // pub name: String,
}

// #[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(root = b"probennahmestelle")]
// pub struct Probennahmestelle {
//     #[xml(ns = b"xwas", name = b"probennahmestelleID", ty = "child")]
//     pub probennahmestelle_id: String,
//     #[xml(ns = b"xwas", name = b"objektID", ty = "child")]
//     pub objekt_id: String,
//     #[xml(ns = b"xwas", name = b"probe", ty = "child")]
//     pub probe: ProbeType,
//     #[xml(ns = b"xwas", name = b"nameProbennahmestelle", ty = "child")]
//     pub name_probennahmestelle: String,
//     #[xml(ns = b"xwas", name = b"artProbennahmestelle", ty = "child")]
//     pub art_probennahmestelle: ArtProbennahmestelle,
//     #[xml(ns = b"xwas", name = b"mediumAnDerProbennahmestelle", ty = "child")]
//     pub medium_an_der_probennahmestelle: MediumAnDerProbennahmestelle,
// }

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeArtProbennahmestelleType")]
pub struct CodeArtProbennahmestelleType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}


#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeAufbereitungsstoffDesinfektionsverfahrenType")]
pub struct CodeAufbereitungsstoffDesinfektionsverfahrenType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}



/// Klasse für den Transport von Informationen zu einer Probennahmestelle.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"ProbennahmestelleType")]
pub struct ProbennahmestelleType {
    #[xml(ns = b"xwas", name = b"probennahmestelleID", ty = "child")]
    pub probennahmestelle_id: String,
    #[xml(ns = b"xwas", name = b"objektID", ty = "child")]
    pub objekt_id: String,
    #[xml(ns = b"xwas", name = b"probe", ty = "child")]
    pub probe: ProbeType,
    #[xml(ns = b"xwas", name = b"terminplanID", ty = "child")]
    pub terminplan_id: Vec<String>,
    #[xml(ns = b"xwas", name = b"nameProbennahmestelle", ty = "child")]
    pub name_probennahmestelle: String,
    #[xml(ns = b"xwas", name = b"artProbennahmestelle", ty = "child")]
    pub art_probennahmestelle: CodeArtProbennahmestelleType,
    #[xml(ns = b"xwas", name = b"stockwerkProbennahmestelle", ty = "child")]
    pub stockwerk_probennahmestelle: Vec<u8>,
    #[xml(ns = b"xwas", name = b"mediumAnDerProbennahmestelle", ty = "child")]
    pub medium_an_der_probennahmestelle: CodeMediumType,
    #[xml(ns = b"xwas", name = b"desinfektionUndAufbereitungDesWassers", ty = "child")]
    pub desinfektion_und_aufbereitung_des_wassers: Vec<CodeAufbereitungsstoffDesinfektionsverfahrenType>,
    #[xml(ns = b"xwas", name = b"altID", ty = "child")]
    pub alt_id: Vec<String>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Vec<String>,
    #[xml(ns = b"xwas", name = b"probennehmerID", ty = "attr")]
    #[serde(skip)]
    pub _id: ConstStr,

}


#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct CodeZweckType { // xoev-code:Code
    // #[xml(name = b"listVersionID", ty = "attr")]
    // pub list_version_id: Option<String>,
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    // #[xml(name = b"name", ty = "child")]
    // pub name: String,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct Name {
    // #[xml(name = b"listVersionID", ty = "attr")]
    // pub list_version_id: Option<String>,
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    // #[xml(name = b"name", ty = "child")]
    // pub name: String,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct NameZugelasseneUntersuchungsstelle {
    // #[xml(name = b"listVersionID", ty = "attr")]
    // pub list_version_id: Option<String>,
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    // #[xml(name = b"name", ty = "child")]
    // pub name: String,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeUntersuchungsstelleType")]
pub struct CodeUntersuchungsstelleType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}


/// Klasse für den Transport von Informationen zu einer zugelassenen Untersuchungsstelle.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"zugelasseneUntersuchungsstelleType")]
pub struct ZugelasseneUntersuchungsstelleType {
    #[xml(name = b"organisation", ty = "child")]
    pub organisation: OrganisationType,
    #[xml(name = b"zugelasseneUntersuchungsstelleID", ty = "child")]
    pub zugelassene_untersuchungsstelle_id: String, //ConstStr,
    #[xml(name = b"nameZugelasseneUntersuchungsstelle", ty = "child")]
    pub name_zugelassene_untersuchungsstelle: CodeUntersuchungsstelleType,
    #[xml(name = b"pruefgebieteUntersuchungenPhysChem", ty = "child")]
    pub pruefgebiete_untersuchungen_phys_chem: Vec<bool>,
    #[xml(name = b"pruefgebieteUntersuchungenMikrobio", ty = "child")]
    pub pruefgebiete_untersuchungen_mikrobio: Vec<bool>,
    #[xml(name = b"pruefgebieteUntersuchungenRadionuklide", ty = "child")]
    pub pruefgebiete_untersuchungen_radionuklide: Vec<bool>,
    #[xml(name = b"akkreditierungsnummer", ty = "child")]
    pub akkreditierungsnummer: Vec<String>,
    #[xml(name = b"kommentarBeauftragteUntersuchungsstelle", ty = "child")]
    pub kommentar_beauftragte_untersuchungsstelle: Vec<String>,
    #[xml(name = b"kommentarZugelasseneUntersuchungsstelle", ty = "child")]
    pub kommentar_zugelassene_untersuchungsstelle: Vec<String>,
}

/// Klasse für den Transport von ergänzenden Informationen zu den Daten aus dem Register
/// für Zugelassene Untersuchungsstellen im Falle der Beauftragung einer Untersuchung.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"BeauftragteUntersuchungsstelleType")]
pub struct BeauftragteUntersuchungsstelleType {
    #[xml(ns = b"xwas", name = b"zugelasseneUntersuchungsstelle", ty = "child")]
    pub zugelassene_untersuchungsstelle: ZugelasseneUntersuchungsstelleType,
    #[xml(ns = b"xwas", name = b"kommentarBeauftragteUntersuchungsstelle", ty = "child")]
    pub kommentar_beauftragte_untersuchungsstelle: Vec<String>,
}


#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"ZugelasseneUntersuchungsstelleType")]
pub struct OrtDerLabortaetigkeiten {
    #[xml(name = b"id", ty = "attr", value = "H & D GmbH")]
    _id: String,
}

// /// Klasse für den Transport von Informationen zu einer Zuständigen Behörde [Ergänzende
// /// Angaben zu einer im Register Behörden gepflegte Behörde].
// #[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(root = b"beauftragteUntersuchungsstelleRel")]
// pub struct BeauftragteUntersuchungsstelleRel {
//     #[xml(ns = b"xwas", name = b"rechtsform", ty = "child")]
//     pub rechtsform: Rechtsform,
//     #[xml(ns = b"xwas", name = b"branche", ty = "child")]
//     pub branche: Branche,
//     #[xml(ns = b"xwas", name = b"zweck", ty = "child")]
//     pub zweck: Zweck,
//     #[xml(ns = b"xwas", name = b"name", ty = "child")]
//     pub name: String, //Name,
//     #[xml(ns = b"xwas", name = b"unterorganisation", ty = "child")]
//     pub unterorganisation: String,
//     // pub unterorganisation: Unterorganisation,
//     #[xml(ns = b"xwas", name = b"kommunikation", ty = "child")]
//     pub kommunikation: String,
//     #[xml(ns = b"xwas", name = b"registrierung", ty = "child")]
//     pub registrierung: String,
//     #[xml(ns = b"xwas", name = b"identifikation", ty = "child")]
//     pub identifikation: String,
//     #[xml(ns = b"xwas", name = b"existenzzeitraum", ty = "child")]
//     pub existenzzeitraum: String,
//     #[xml(ns = b"xwas", name = b"anschrift", ty = "child")]
//     pub anschrift: AnschriftType,
//     #[xml(ns = b"xwas", name = b"zugelasseneUntersuchungsstelleID", ty = "child")]
//     pub zugelassene_untersuchungsstelle_id: String,
//     #[xml(
//         ns = b"xwas",
//         name = b"nameZugelasseneUntersuchungsstelle",
//         ty = "child"
//     )]
//     pub name_zugelassene_untersuchungsstelle: NameZugelasseneUntersuchungsstelle,
//     #[xml(
//         ns = b"xwas",
//         name = b"pruefgebieteUntersuchungenPhysChem",
//         ty = "child"
//     )]
//     pub pruefgebiete_untersuchungen_phys_chem: bool,
//     #[xml(
//         ns = b"xwas",
//         name = b"pruefgebieteUntersuchungenMikrobio",
//         ty = "child"
//     )]
//     pub pruefgebiete_untersuchungen_mikrobio: bool, //Vec<PruefgebieteUntersuchungenMikrobio>,
//     #[xml(
//         ns = b"xwas",
//         name = b"pruefgebieteUntersuchungenRadionuklide",
//         ty = "child"
//     )]
//     pub pruefgebiete_untersuchungen_radionuklide: bool, //Vec<PruefgebieteUntersuchungenRadionuklide>,
//     #[xml(ns = b"xwas", name = b"akkreditierungsnummer", ty = "child")]
//     pub akkreditierungsnummer: String, //Akkreditierungsnummer,
//     #[xml(
//         ns = b"xwas",
//         name = b"kommentarZugelasseneUntersuchungsstelle",
//         ty = "child"
//     )]
//     pub kommentar_zugelassene_untersuchungsstelle: String, //KommentarZugelasseneUntersuchungsstelle,
//     #[xml(ns = b"xwas", name = b"pruefberichtID", ty = "child")]
//     pub pruefbericht_id: String,
//     #[xml(
//         ns = b"xwas",
//         name = b"kommentarBeauftragteUntersuchungsstelle",
//         ty = "child"
//     )]
//     pub kommentar_beauftragte_untersuchungsstelle: String,
// }

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"zustaemdigeBehoerdeRel")]
pub struct ZustaendigeBehoerdeRel {
    #[xml(ns = b"xwas", name = b"id", ty = "child")]
    pub id: String,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    pub zusatz: String,
    #[xml(ns = b"xwas", name = b"behoerdenKennung", ty = "child")]
    pub behoerden_kennung: String, //BehoerdenKennung,
    #[xml(ns = b"xwas", name = b"kommunikation", ty = "child")]
    pub kommunikation: String,
    #[xml(ns = b"xwas", name = b"behoerdenIdentifikation", ty = "child")]
    pub behoerden_identifikation: String,
    #[xml(ns = b"xwas", name = b"behoerdenname", ty = "child")]
    pub behoerdenname: String,
    #[xml(ns = b"xwas", name = b"nachgeordneteBehoerde", ty = "child")]
    pub nachgeordnete_behoerde: String,
    #[xml(
        ns = b"xwas",
        name = b"verwaltungspolitischeZustaendigkeit",
        ty = "child"
    )]
    pub verwaltungspolitische_zustaendigkeit: String,
    #[xml(ns = b"xwas", name = b"anschrift", ty = "child")]
    pub anschrift: AnschriftType,
    #[xml(ns = b"xwas", name = b"anlageNachTrinkwVID", ty = "child")]
    pub anlage_nach_trinkw_vid: String,
    #[xml(ns = b"xwas", name = b"probennehmerID", ty = "child")]
    pub probennehmer_id: String,
    #[xml(ns = b"xwas", name = b"pruefberichtID", ty = "child")]
    pub pruefbericht_id: String,
    #[xml(ns = b"xwas", name = b"laenderkuerzel", ty = "child")]
    pub laenderkuerzel: Laenderkuerzel,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeGesamtbewertungType")]
pub struct CodeGesamtbewertungType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeAmtsspracheEUType")]
pub struct CodeAmtsspracheEUType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}

/// Erweiterung.XML darf nur dazu genutzt werden, weitere (z. B. fachspezifische)
/// Metadaten zu spezifizieren, deren Übermittlung mit den bereits in XWasser
/// spezifizierten Metadaten nicht möglich ist. Erweiterung.XML bietet über ein
/// xs:any-Element die Möglichkeit, mittels Einbindung externer XML-Schemata diese
/// Metadaten zu spezifizieren. Es können beliebige XML-Schemata mit unterschiedlichen
/// Namensräumen angegeben werden. Die XML-Schema-Validierung der weiterführenden
/// Metadaten erfolgt innerhalb der XWasser-Nachricht selbst (prozessContents = "lax").
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"erweiterungXMLType")]
pub struct ErweiterungXmlType {
    // weiteres xml schema 
    #[xml(ns = b"xwas", name = b"any", ty = "child")]
    pub any: Vec<String>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeDatentypType")]
pub struct CodeDatentypType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}


/// Ein Feld ist ein anwendungsspezifisches Metadatum. Die Konfiguration eines Feldes
/// muss zwischen den Kommunikationspartnern abgesprochen sein.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"erweiterungFeldType")]
pub struct ErweiterungFeldType {
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
    #[xml(ns = b"xwas", name = b"beschreibung", ty = "child")]
    pub beschreibung: Vec<String>,
    #[xml(ns = b"xwas", name = b"datentyp", ty = "child")]
    pub datentyp: CodeDatentypType,
    #[xml(ns = b"xwas", name = b"wert", ty = "child")]
    pub wert: String,
}


/// Eine Erweiterungsgruppe fasst mehrere Felder (Metadaten) zu einem Objekt zusammen .
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
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
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"ErweiterungType")]
pub struct ErweiterungType {
    #[xml(ns = b"xwas", name = b"feld", ty = "child")]
    pub feld: Vec<ErweiterungFeldType>,
    #[xml(ns = b"xwas", name = b"gruppe", ty = "child")]
    pub gruppe: Vec<ErweiterungGruppeType>,
    #[xml(ns = b"xwas", name = b"xml", ty = "child")]
    pub xml: Vec<ErweiterungXmlType>,
}



/// Klasse zur Erfassung bzw. zum Transport der Daten eines Prüfberichts. Prüfberichte
/// werden erstellt, nachdem eine Wasserprobe im Labor analysiert wurde.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"PruefberichtType")]
pub struct PruefberichtType {
    #[xml(ns = b"xwas", name = b"pruefberichtUUID", ty = "child")]
    pub pruefbericht_uuid: String,
    #[xml(ns = b"xwas", name = b"untersuchungsplanID", ty = "child")]
    pub untersuchungsplan_id: Vec<String>,
    #[xml(ns = b"xwas", name = b"probennahmestelle", ty = "child")]
    pub probennahmestelle: ProbennahmestelleType,
    #[xml(ns = b"xwas", name = b"nameBeauftragteUntersuchungsstelle", ty = "child")]
    pub name_beauftragte_untersuchungsstelle: CodeNameBeauftragteUntersuchungsstelle,
    #[xml(ns = b"xwas", name = b"pruefberichtEnthaeltTeilergebnisse", ty = "child")]
    pub pruefbericht_enthaelt_teilergebnisse: Vec<bool>,
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
    pub fuer_validierung_verantwortliche_person: NatuerlichePersonType,
    #[xml(ns = b"xwas", name = b"freigabeUebermittlungBetreiber", ty = "child")]
    pub freigabe_uebermittlung_betreiber: Vec<bool>,
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
    pub kommentar: Vec<String>,
    #[xml(ns = b"xwas", name = b"auftraggeber", ty = "child")]
    pub auftraggeber: AuftraggeberType,
    #[xml(ns = b"xwas", name = b"zustaendigeBehoerde", ty = "child")]
    pub zustaendige_behoerde: ZustaendigeBehoerdeType,
    #[xml(ns = b"xwas", name = b"beauftragteUntersuchungsstelle", ty = "child")]
    pub beauftragte_untersuchungsstelle: BeauftragteUntersuchungsstelleType,
    #[xml(ns = b"xwas", name = b"ortDerLabortaetigkeiten", ty = "child")]
    pub ort_der_labortaetigkeiten: AnschriftType,
    #[xml(ns = b"xwas", name = b"anhang", ty = "child")]
    pub anhang: Vec<String>,
    #[xml(ns = b"xwas", name = b"erweiterung", ty = "child")]
    pub erweiterung: Vec<ErweiterungType>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeAuftraggeberartType")]
pub struct CodeAuftraggeberartType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}

/// Informationen zu einem Auftraggeber [Ergänzende Angaben zu den jeweiligen
/// Informationen aus den Registern von Betreibern/Behörden].
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"auftraggeber")]
pub struct Auftraggeber {
    // #[xml(ns = b"xwas", name = b"organisation", ty = "child")]
    // pub organisation: OrganisationType,
    #[xml(ns = b"xwas", name = b"natuerlichePerson", ty = "child")]
    pub natuerliche_person: NatuerlichePersonType,
    // #[xml(ns = b"xwas", name = b"zustaendigeBehoerde", ty = "child")]
    // pub zustaendige_behoerde: ZustaendigeBehoerdeType,

}


/// Informationen zu einem Auftraggeber [Ergänzende Angaben zu den jeweiligen
/// Informationen aus den Registern von Betreibern/Behörden].
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"AuftraggeberType")]
pub struct AuftraggeberType {
    #[xml(ns = b"xwas", name = b"auftraggeberID", ty = "child")]
    pub auftraggeber_id: String,
    #[xml(ns = b"xwas", name = b"auftraggeberart", ty = "child")]
    pub auftraggeberart: CodeAuftraggeberartType,
    #[xml(ns = b"xwas", name = b"auftraggeber", ty = "child")]
    pub auftraggeber: Auftraggeber,
}


#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct AuftraggeberRel {
    #[xml(ns = b"xwas", name = b"auftraggeberID", ty = "child")]
    pub auftraggeber_id: String,
    // #[xml(name = b"pruefberichtID", ty = "child")]
    // pub pruefberichtID: String,
    // #[xml(name = b"untersuchungsplanID", ty = "child")]
    // pub untersuchungsplanID: String,
    #[xml(ns = b"xwas", name = b"auftraggeberart", ty = "child")]
    pub auftraggeberart: AuftraggeberArt,
    #[xml(ns = b"xwas", name = b"auftraggeber", ty = "child")]
    pub auftraggeber: Auftraggeber,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
pub struct CodeNameBeauftragteUntersuchungsstelle {
    #[xml(name = b"code", ty = "child")]
    pub code: Code,
    #[xml(name = b"name", ty = "child")]
    pub name: Vec<String>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeLaenderkennzeichenType")]
pub struct CodeLaenderkennzeichenType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}


/// Klasse für den Transport von Informationen zu einer Zuständigen Behörde [Ergänzende
/// Angaben zu einer im Register Behörden gepflegte Behörde].
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
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
    pub kommentar: Vec<String>,
}


/// Der AllgemeineName dient der Darstellung von Vor- und Nachnamen und fasst deren
/// gemeinsame Eigenschaften zusammen.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"AllgemeinerNameType")]
pub struct AllgemeinerNameType {
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
    #[xml(ns = b"xwas", name = b"nichtVorhanden", ty = "child")]
    pub nicht_vorhanden: bool,
    #[xml(ns = b"xwas", name = b"namensart", ty = "child")]
    pub namensart: Code,
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
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"AlternativeRepraesentationType")]
pub struct AlternativeRepraesentationType {
    #[xml(ns = b"xwas", name = b"repraesentation", ty = "child")]
    pub repraesentation: String,
    #[xml(ns = b"xwas", name = b"algorithmus", ty = "child")]
    pub algorithmus: Vec<String>,
    #[xml(ns = b"xwas", name = b"hinweis", ty = "child")]
    pub hinweis: Vec<String>,
}

/// Der Name einer Person ist eine Benennung dieser Person, die dazu dient, diese Person
/// von anderen Personen zu unterscheiden.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"nameNatuerlichePersonType")]
pub struct NameNatuerlichePersonType {
    #[xml(name = b"titel", ty = "child")]
    pub titel: Vec<String>,
    #[xml(name = b"anrede", ty = "child")]
    pub anrede: String,
    #[xml(name = b"namenssuffix", ty = "child")]
    pub namenssuffix: Vec<String>,
    #[xml(ns = b"xwas", name = b"familienname", ty = "child")]
    pub familienname: Vec<AllgemeinerNameType>,
    #[xml(name = b"ehename", ty = "child")]
    pub ehename: Vec<AllgemeinerNameType>,
    #[xml(name = b"lebenspartnerschaftsname", ty = "child")]
    pub lebenspartnerschaftsname: Vec<AllgemeinerNameType>,
    #[xml(name = b"geburtsname", ty = "child")]
    pub geburtsname: Vec<AllgemeinerNameType>,
    #[xml(name = b"fruehererFamilienname", ty = "child")]
    pub frueherer_familienname: Vec<AllgemeinerNameType>,
    #[xml(ns = b"xwas", name = b"vorname", ty = "child")]
    pub vorname: Vec<AllgemeinerNameType>,
    #[xml(name = b"rufname", ty = "child")]
    pub rufname: Vec<AllgemeinerNameType>,
    #[xml(name = b"fruehererVorname", ty = "child")]
    pub frueherer_vorname: Vec<AllgemeinerNameType>,
    #[xml(name = b"alternativeRepraesentation", ty = "child")]
    pub alternative_repraesentation: Vec<AlternativeRepraesentationType>,
    #[xml(name = b"ordensname", ty = "child")]
    pub ordensname: Vec<AllgemeinerNameType>,
    #[xml(name = b"kuenstlername", ty = "child")]
    pub kuenstlername: Vec<AllgemeinerNameType>,
    #[xml(name = b"weitererName", ty = "child")]
    pub weiterer_name: Vec<AllgemeinerNameType>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"Auskunftssperre")]
pub struct Auskunftssperre {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    // #[xml(ns = b"xwas", name = b"name", ty = "child")]
    // pub name: Vec<String>,
}

/// Der Zeitraum kennzeichnet einen Abschnitt auf einem Zeitstrahl durch Angabe von
/// Beginn und/oder Ende.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"ZeitraumType")]
pub struct ZeitraumType {
    #[xml(ns = b"xwas", name = b"beginn", ty = "child")]
    pub beginn: Vec<String>, // eigentlich of tyype xs:date
    #[xml(ns = b"xwas", name = b"ende", ty = "child")]
    pub ende: Vec<String>,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    pub zusatz: Vec<String>,
}

/// Die Auskunftssperre beschränkt die Weitergabe von Informationen an Dritte.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"AuskunftssperreType")]
pub struct AuskunftssperreType {
    #[xml(ns = b"xwas", name = b"grund", ty = "child")]
    pub grund: Auskunftssperre,
    #[xml(ns = b"xwas", name = b"gueltigkeit", ty = "child")]
    pub gueltigkeit: Vec<ZeitraumType>,
}


#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"FamilienstandType")]
pub struct Familienstand {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: Code,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}

/// Unter "Geburt" werden geburtsbezogene Informationen zusammengefasst.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"Geburt")]
pub struct Geburt {
    #[xml(ns = b"xwas", name = b"datum", ty = "child")]
    pub datum: Vec<String>,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    pub zusatz: Vec<String>,
    #[xml(ns = b"xwas", name = b"geburtsort", ty = "child")]
    pub geburtsort: Vec<AnschriftType>,
}

/// Dieser Datentyp erlaubt die Angabe von Doktorgraden. Es sind nur diejenigen
/// Doktorgrade anzugeben, die in Pässe eingetragen werden dürfen. Sind mehrere
/// Doktorgrade anzugeben, so sind sie durch ein Leerzeichen zu trennen. Zulässig sind
/// derzeit: „DR.“, „Dr.“, „DR.HC.“, „Dr.hc.“, „Dr.EH.“ und „Dr.eh.“.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"DoktorgradType")]
pub struct DoktorgradType {
    // hier gibt es eine striction mit max length 120, wie umsetzen ?
    #[xml(ns = b"xwas", name = b"bezeichnung", ty = "child")]
    pub bezeichnung: String,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"Code.StaatsangehoerigkeitType")]
pub struct CodeStaatsangehoerigkeitType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"Code.AusweisdokumenteType")]
pub struct CodeAusweisdokumenteType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"Code.GeschlechtType")]
pub struct CodeGeschlechtType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}

/// Unter "Sprache" werden Informationen über Sprachen zusammengefasst.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"SpracheType")]
pub struct SpracheType {
    #[xml(ns = b"xwas", name = b"sprache", ty = "child")]
    pub sprache: String,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    pub zusatz: Vec<String>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"Code.VertretungsartType")]
pub struct CodeVertretungsartType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,

}


/// Mit diesem Datentyp wird ein gesetzlicher Vertreter oder ein Bevollmächtigter einer
/// nichtnatürlichen Person abgebildet.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
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
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"natuerlichePersonType")]
pub struct NatuerlichePersonType {
    #[xml(name = b"auskunftssperre", ty = "child")]
    pub auskunftssperre: AuskunftssperreType,
    #[xml(ns = b"xwas", name = b"nameNatuerlichePerson", ty = "child")]
    pub name_natuerliche_person: NameNatuerlichePersonType,
    #[xml(name = b"familienstand", ty = "child")]
    pub familienstand: Familienstand,
    #[xml(name = b"geburt", ty = "child")]
    pub geburt: Vec<Geburt>,
    #[xml(name = b"doktorgrad", ty = "child")]
    pub doktorgrad: Vec<DoktorgradType>,
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
    pub vertreter_bevollmaechtigter: VertreterBevollmaechtigterType,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(tns(b"xwas", b"xwasser"))]
pub struct ProbennehmerType {
    // #[xml(ns = b"xwas", name = b"organisation", ty = "child")]
    // pub organisation: OrganisationType,
    // #[xml(ns = b"xwas", name = b"natuerlichePerson", ty = "child")]
    // pub natuerliche_person: NatuerlichePersonType,
    #[xml(ns = b"xwas", name = b"zustaendigeBehoerde", ty = "child")]
    pub zustaendige_behoerde: ZustaendigeBehoerdeType,
}

/// Klasse für den Transport von Informationen zu einem Probennehmer [Durch das Labor mit
/// dem Prüfbericht mit zu übermittelnde Informationen].
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"ProbennehmerType")]
pub struct Probennehmer {
    #[xml(ns = b"xwas", name = b"probennehmerID", ty = "child")]
    pub probennehmer_id: String,
    #[xml(ns = b"xwas", name = b"probennehmer", ty = "child")]
    pub probennehmer: ProbennehmerType,
    #[xml(ns = b"xwas", name = b"fremdsystemID_Probennehmer", ty = "child")]
    pub fremdsystem_id_probennehmer: String,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Vec<String>,
    #[xml(ns = b"xwas", name = b"_id", ty = "attr")]
    #[serde(skip)]
    pub _id: ConstStr,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeProbenentnahmegeraetType")]
pub struct CodeProbenentnahmegeraetType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeGefaessType")]
pub struct CodeProbengefaessType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}



#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"codeProbenbewertungType")]
pub struct CodeProbenbewertungType {
    #[xml(ns = b"xwas", name = b"code", ty = "child")]
    pub code: String,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: Vec<String>,
}


/// Klasse zum Transport von Informationen, welche über eine Probe vorliegen sollen, die
/// im Rahmen eines Prüfberichts via SHAPTH übermittelt wird.
#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"Probe")]
pub struct ProbeType {
    #[xml(ns = b"xwas", name = b"probeID", ty = "child")]
    pub probe_id: String,
    #[xml(ns = b"xwas", name = b"analyseergebnisParameter", ty = "child")]
    pub analyseergebnis_parameter: AnalyseergebnisParameter,
    #[xml(ns = b"xwas", name = b"probennehmer", ty = "child")]
    pub probennehmer: Probennehmer,
    #[xml(ns = b"xwas", name = b"anlassDerUntersuchung", ty = "child")]
    pub anlass_der_untersuchung: CodeAnlassUntersuchungType,
    #[xml(ns = b"xwas", name = b"medium", ty = "child")]
    pub medium: CodeMediumType,
    #[xml(name = b"ergaenzungZumMedium", ty = "child")]
    pub ergaenzung_zum_medium: Vec<String>,
    #[xml(ns = b"xwas", name = b"zeitpunktProbennahme", ty = "child")]
    pub zeitpunkt_probennahme: String, //datetime
    #[xml(ns = b"xwas", name = b"probennahmeverfahren", ty = "child")]
    pub probennahmeverfahren: CodeProbennahmeverfahrenType,
    #[xml(name = b"probenentnahmegeraet", ty = "child")]
    pub probenentnahmegeraet: Vec<CodeProbenentnahmegeraetType>,
    #[xml(name = b"probengefaess", ty = "child")]
    pub probengefaess: Vec<CodeProbengefaessType>,
    #[xml(name = b"ergaenzendeInformationenZuProbenentnahmegeraet", ty = "child")]
    pub ergaenzende_informationen_zu_probenentnahmegeraet: Vec<String>,
    #[xml(name = b"desinfektionProbenentnahmegeraetDurchgefuehrt", ty = "child")]
    pub desinfektion_probenentnahmegeraet_durchgefuehrt: Vec<bool>,
    #[xml(name = b"konservierungAufbereitungDesinfektionProbe", ty = "child")]
    pub konservierung_aufbereitung_desinfektion_probe: Vec<CodeAufbereitungsstoffDesinfektionsverfahrenType>,
    #[xml(ns = b"xwas", name = b"kommentarZurProbennahme", ty = "child")]
    pub kommentar_zur_probennahme: String,
    #[xml(name = b"informationenZumProbentransport", ty = "child")]
    pub informationen_zum_probentransport: Vec<String>,
    #[xml(ns = b"xwas", name = b"eingangProbeBeiUntersuchungsstelle", ty = "child")]
    pub eingang_probe_bei_untersuchungsstelle: String, //xs dateTime
    #[xml(ns = b"xwas", name = b"beginnAnalytik", ty = "child")]
    pub beginn_analytik: String, //datetime
    #[xml(ns = b"xwas", name = b"abschlussAnalytik", ty = "child")]
    pub abschluss_analytik: String,
    #[xml(ns = b"xwas", name = b"probenbewertung", ty = "child")]
    pub probenbewertung: CodeProbenbewertungType,
    #[xml(ns = b"xwas", name = b"probennahmestelleID", ty = "child")]
    pub probennahmestelle_id: String,
    #[xml(ns = b"xwas", name = b"analyseergebnisParameterID", ty = "child")]
    pub analyseergebnis_parameter_id: String,
    #[xml(name = b"berichtspflichtig", ty = "child")]
    pub berichtspflichtig: Vec<bool>,
    #[xml(ns = b"xwas", name = b"vonProbennehmerVergebeneProbeID", ty = "child")]
    pub von_probennehmer_vergebene_probe_id: String,
    #[xml(ns = b"xwas", name = b"probeID_ausLabor", ty = "child")]
    pub probe_id_aus_labor: String,
    #[xml(name = b"anhang", ty = "child")]
    pub anhang: Vec<String>,
    #[xml(name = b"kommentar", ty = "child")]
    pub kommentar: Vec<String>,
    #[xml(ns = b"xwas", name = b"probennehmerID", ty = "attr")]
    #[serde(skip)]
    pub _id: ConstStr,
}

/// Klasse für den Transport von Informationen, die für die Erstellung eines
/// Untersuchungsplans für a- und b-Anlagen relevant sind.
#[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
#[xml(root = b"Untersuchungsplan")]
pub struct Untersuchungsplan {
    /// ID zur eindeutigen Identifikation eines Untersuchungsplans. Strukturell
    /// definiertcolon Anlagen ID - Jahr (8-stellig) - Fortlaufende Nr. (5-stellig); Jahr
    /// 8-stellig, da Untersuchungsplan mehrere Jahre gültig sein kann, und Angabe des ersten
    /// und letzten Jahres sipnnvoll ist. ToDo: Die genaue Syntax ist an einem Beispiel zu
    /// konkretisieren!
    #[xml(name = b"untersuchungsplanID", ty = "child")]
    pub untersuchungsplanID: String,
    /// Referenz zur eindeutigen Identifikation einer Anlage nach Trinkwasserverordnung.
    #[xml(name = b"anlageNachTrinkVID", ty = "child")]
    pub anlageNachTrinkVID: String,
    /// Referenz zur eindeutigen Identifikation eines Wasserversorgungsgebiets.
    #[xml(name = b"wasserversorgungsgebiet", ty = "child")]
    pub wasserversorgungsgebiet: String,
    /// Referenz zur eindeutigen Identifikation eines Auftraggebers (und somit einer Behörde
    /// oder eines Betreibers).
    #[xml(name = b"auftraggeberID", ty = "child")]
    pub auftraggeberID: String,
    /// Referenz zur eindeutigen Identifikation einer zuständigen Behörde.
    #[xml(name = b"zustaendigeBehoerdeID", ty = "child")]
    pub zustaendigeBehoerdeID: String,
    /// Das Jahr, oder die Jahre, für die der Untersuchungsplan erstellt wird. Den Plan für
    /// mehrere Jahre anzulegenen bietet bietet sich insbesondere für kleine WVA mit wenigen
    /// Proben an. Es sollten Gültigkeitszeiträume gewählt werden, die ganzzahlige
    /// Probennanzahlen für die Parameter der Gruppe B ergeben.
    #[xml(name = b"jahr", ty = "child")]
    pub jahr: String,
    /// Wasserabgabe (Vorjahr).
    #[xml(name = b"wasserabgabeVorjahr", ty = "child")]
    pub wasserabgabeVorjahr: String,
    /// "Dieser Eintrag hat Einfluss darauf, welche Parameter in den Messprogrammen
    /// automatisch zur Untersuchung markiert werden. Bei Anlagen mit 100 % Fremdbezug werden
    /// z.B. keine unveränderlichen Parameter markiert. 1: Eigenständige WVA
    /// (Versorgungsgebiet der WVA =WVG; Anlage mit eigener Wassergewinnung und Verteilung
    /// oder vollständig untersuchte WVA mit 100% Fremdbezug) 2: Fern-WVA ohne versorgte
    /// Gebiete (Teil des WVG) 3: Fern-WVA mit versorgten Gebieten (gesamtes WVG) 4: WVA mit
    /// 100 % Fremdbezug (Teil des WVG einer anderen WVA oder Fern-WVA)".
    #[xml(name = b"artVonWVAundWVG", ty = "child")]
    pub artVonWVAundWVG: String,
    /// Erläuterungen zur Wasserabgabemenge mit Blick auf die Anrechnung von Untersuchungen
    /// bei Fremdbezug oder Fremdabgabe durch den Lieferanten bzw. Abnehmern.
    #[xml(name = b"erlaeuterungZurWasserabgabemenge", ty = "child")]
    pub erlaeuterungZurWasserabgabemenge: String,
    /// Falls eine Flockung mit Eisen oder Aluminium erfolgt, beeinnflusst das die Zuordnung
    /// dieser Parameter zu Gruppe A oder Gruppe B.
    #[xml(name = b"flockung", ty = "child")]
    pub flockung: String,
    /// Oberflächenwassereinfluss bei dem abgegebenen Trinkwasser.
    #[xml(name = b"oberflaechenwassereinfluss", ty = "child")]
    pub oberflaechenwassereinfluss: String,
    /// "Wenn dauerhaft oder regelmäßig eine Desinfektion mit Chlor, Hypochloriten oder
    /// elektrolytisch erzeugte Chlorlösungen erfolgt, ist auf THM zu untersuchen. Wenn keine
    /// Desinfektion mit den vorgenannten Chlorprodukten erfolgt und die Chlorung nur in
    /// Bereitschaft gehalten wird, oder wenn mit Chlordioxid desinfiziert wird, ist keine
    /// Untersuchung auf THM benötigt."
    #[xml(name = b"desinfektionDurchgefuehrtMit", ty = "child")]
    pub desinfektionDurchgefuehrtMit: String,
    /// Trinkwasserabgabe in verschlossenen Behältnissen liegt in der Regel in Deutschland
    /// nicht als Nutzungszweck ganzer Wasserversorgungsanlagen vor. Wenn
    /// Wasserversorgungsunternehmen neben der leitungsgebundenen Abgabe auch Trinkwasser in
    /// Flaschen abfüllen sollten, ist trotzdem "nein" einzutragen.
    #[xml(
        name = b"abfuellungZurAbgabeInVerschlossenenBehaeltnissen",
        ty = "child"
    )]
    pub abfuellungZurAbgabeInVerschlossenenBehaeltnissen: String,
    /// "Folgende Varianten würden einen rechnerischen Nachweis der Einhaltung des Parameters
    /// Acrylamid ermöglichen: • frühere Untersuchungen lagen unter der Nachweisgrenze und es
    /// gibt keine einschlägigen Änderungen in der Aufbereitung. • kein Einsatz
    /// Polyacrylamid-haltiger Flockungshilfsmittel bei der Wasseraufbereitung •
    /// Polyacrylamid-haltige Flockungshilfsmittel werden verwendet, die Einhaltung der
    /// zulässigen Zugabe und der Reinheitsanforderungen gemäß §11-Liste wird kontrolliert."
    #[xml(name = b"acrylamid", ty = "child")]
    pub acrylamid: String,
    /// "Folgende Varianten würden einen rechnerischen Nachweis der Einhaltung des Parameters
    /// Epichlorhydrin ermöglichen: • frühere Untersuchungen lagen unter der Nachweisgrenze
    /// und es gibt keine einschlägigen Änderungen bei Rohren, Behältern und Beschichtungen •
    /// Es sind keine trinkwasserberührten epoxidharzhaltigen Rohre, Behälter und
    /// Beschichtungen in der öffentlichen WVA vorhanden und dem Gesundheitsamt sind keine
    /// Epoxidharzbeschichtungen in Trinkwasser-Installationen bekannt. • wasserberührte
    /// epoxidharzbasierte Bauteile oder Beschichtungen sind in der öffentlichen WVA
    /// vorhanden. Es wurden ausschließlich zertifizierte Produkte und Verfahren eingesetzt."
    #[xml(name = b"epichlorhydrin", ty = "child")]
    pub epichlorhydrin: String,
    /// "Folgende Varianten würden einen rechnerischen Nachweis der Einhaltung des Parameters
    /// Vinylchlorid ermöglichen: • Frühere Untersuchungen lagen unter der Nachweisgrenze und
    /// es gibt keine einschlägigen Änderungen bei Rohren, Behältern und Beschichtungen sowie
    /// im Einzugsgebiet. • Es sind keine trinkwasserberührten Rohre, Behälter und
    /// Beschichtungen aus PVC in der öffentlichen WVA vorhanden. • Wasserberührte Bauteile
    /// oder Beschichtungen aus PVC sind in der öffentlichen WVA vorhanden. Es wurden
    /// ausschließlich zertifizierte Produkte eingesetzt. • Belastungen mit chlorierten
    /// Lösungsmitteln im Wassereinzugsgebiet können jeweils ausgeschlossen werden."
    #[xml(name = b"vinylchlorid", ty = "child")]
    pub vinylchlorid: String,
    /// Wenn der pH-Wert am Wassewerksausgang i.d.R. gt 7,7 ist, braucht der Parameter
    /// Calcitlösekapazität nicht untersucht werden.
    #[xml(name = b"phWertWasserwerksausgang", ty = "child")]
    pub phWertWasserwerksausgang: String,
    /// Automatische Berechnung anhand der gesamten Wasserabgabe des Vorjahres:
    /// [=Wasserabgabe (Vorjahr)/365].
    #[xml(name = b"wasserabgabeVorjahrProTag", ty = "child")]
    pub wasserabgabeVorjahrProTag: String,
    /// Automatische Berechnung der Untersuchungsanzahl nach Anlage 4 Buchstabe c TrinkwV:
    /// lbrack=IF("Wasserabgabe (Vorjahr) pro Tag"=0;0;IF("Wasserabgabe (Vorjahr) pro
    /// Tag"lt10;1;IF("Wasserabgabe (Vorjahr) pro Tag"lt=1000;4;4+(ROUNDUP(("Wasserabgabe
    /// (Vorjahr) pro Tag"-1000);-3)/1000)*3)))rbrack.
    #[xml(name = b"anzahlUntersuchungenproJahrGruppeA", ty = "child")]
    pub anzahlUntersuchungenproJahrGruppeA: String,
    /// Wenn das Gesundheitsamt eigene Untersuchungen nach §19 (1) und (7) durchführt, können
    /// diese auf den Umfang der Überwachung angerechnet werden. Hinweis an Implementierende
    /// / Nachrichtenerzeuger: Bei diesem Element handet es sich um ein abhängiges/
    /// dynamisches Pflichtfeld, dessen Befüllung von einem anderen Feldinhalt abhängt.
    #[xml(name = b"abzudeckenDurchBetreiberGruppeA", ty = "child")]
    pub abzudeckenDurchBetreiberGruppeA: String,
    /// Berechnung der Untersuchungsanzahl nach Anlage 4 Buchstabe c TrinkwV:
    /// lbrack=IF("Wasserabgabe (Vorjahr) pro Tag"=0;0;IF("Wasserabgabe (Vorjahr) pro
    /// Tag"lt10;1/3;IF("Wasserabgabe (Vorjahr) pro Tag"lt=1000;1;IF("Wasserabgabe (Vorjahr)
    /// pro Tag"lt=5500;2;IF("Wasserabgabe (Vorjahr) pro Tag"lt=10000;3;IF("Wasserabgabe
    /// (Vorjahr) pro Tag"lt=100000;3+ROUNDUP(("Wasserabgabe (Vorjahr) pro
    /// Tag"-10000);-4)/10000;12+(ROUNDUP(("Wasserabgabe (Vorjahr) pro
    /// Tag"-100000)/25000;0))*1))))))rbrack.
    #[xml(name = b"anzahlUntersuchungenproJahrGruppeB", ty = "child")]
    pub anzahlUntersuchungenproJahrGruppeB: String,
    /// Wenn das Gesundheitsamt eigene Untersuchungen nach §19 (1) und (7) durchführt, können
    /// diese auf den Umfang der Überwachung angerechnet werden.
    #[xml(name = b"abzudeckenDurchBetreiberGruppeB", ty = "child")]
    pub abzudeckenDurchBetreiberGruppeB: String,
    /// Wenn eine RAP durchgeführt wurde, bitte prüfen, dass die Messprogramme und Termine
    /// entsprechend der RAP angelegt wurden. In diesem Fall erfolgt nur bei den nicht
    /// reduzierbaren Parametern eine Fehlermeldung, wenn die Untersuchungsanzahlen nach
    /// Wassermenge nicht erreicht werden.
    #[xml(name = b"rapDurchgefuehrt", ty = "child")]
    pub rapDurchgefuehrt: String,
    /// Klasse für den Transport von Informationen, die für die Erstellung von Terminplänen
    /// als Teil des Untersuchungsplans für a- und b-Anlagen relevant sind.
    #[xml(name = b"terminplan", ty = "child")]
    pub terminplan: String,
    /// Technischer Status des Untersuchungsplans auf SHAPTH und bildet nicht den fachlichen
    /// Status der Untersuchungen ab.
    #[xml(name = b"statusUntersuchungsplan", ty = "child")]
    pub statusUntersuchungsplan: String,
    /// Kommentar zu dem Untersuchungsplan.
    #[xml(name = b"kommentar", ty = "child")]
    pub kommentar: String,
    /// Mit dieser Relation lässt sich zu einem Untersuchungsplan ein Terminplan erfassen.
    #[xml(name = b"terminplan_Rel", ty = "child")]
    pub terminplan_Rel: String,
    #[xml(name = b"anlageNachTrinkwV_Rel", ty = "child")]
    pub anlageNachTrinkwV_Rel: String,
    #[xml(name = b"auftraggeber_Rel", ty = "child")]
    pub auftraggeber_Rel: String,
    #[xml(name = b"zustaendigeBehoerde_Rel", ty = "child")]
    pub zustaendigeBehoerde_Rel: String,
    #[xml(name = b"probe_Rel", ty = "child")]
    pub probe_Rel: String,
    #[xml(name = b"erweiterung", ty = "child")]
    pub erweiterung: String,
}

#[derive(Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(tns(b"xwas", b"xwasser"))]
pub struct VorgangType {
    #[xml(ns = b"xwas", name = b"pruefbericht", ty = "child")]
    pub pruefbericht: PruefberichtType,
    // Pruefbericht(Pruefbericht),
    // #[xml(ns = b"xwas", name = b"untersuchungsplan", ty = "child")]
    // Untersuchungsplan(Untersuchungsplan),
    // #[xml(ns = b"xwas", name = b"olgBericht", ty = "child")]
    // OlgBericht(String),
}

extern crate raxb as _raxb;

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(root = b"administration.quittung.0020")]
// pub struct AdministrationQuittung {
//     #[xml(name = b"nachrichtenkopf.g2g", ty = "child")]
//     pub nachrichtenkopf_g2g: NachrichtenkopfG2g,

//     #[xml(ns = b"xwas", name = b"identifikationVorgang", ty = "child")]
//     pub identifikation_vorgang: IdentifikationVorgang,

//     #[xml(ns = b"xwas", name = b"quittung", ty = "child")]
//     pub quittung: Quittung,
// }

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(root = b"quittung")]
// pub struct Quittung {
//     #[xml(ns = b"xwas", name = b"aktuellerStatusTechnisch", ty = "child")]
//     pub aktueller_status_technisch: AktuellerStatusTechnisch,
// }

// #[derive(Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize, Tsify)]
// #[xml(root = b"aktueller_status_technisch")]
// pub struct AktuellerStatusTechnisch {
//     #[xml(name = b"code", ty = "child")]
//     pub code: Code,
// }

#[wasm_bindgen]
pub fn create_quality_report_xml(data: QualityReport) -> Result<String, JsValue> {
    Ok(raxb::ser::to_string_pretty_with_decl(&data)
        .map_err(|err| JsValue::from_str(&err.to_string()))?)
}

#[wasm_bindgen]
pub fn parse_quality_report_xml(xml: String) -> Result<QualityReport, JsValue> {
    Ok(raxb::de::from_str(&xml).map_err(|err| JsValue::from_str(&err.to_string()))?)
}

// #[wasm_bindgen]
// pub fn create_vorgang_xml(data: Vorgang) -> Result<String, JsValue> {
//     Ok(raxb::ser::to_string_pretty_with_decl(&data)
//         .map_err(|err| JsValue::from_str(&err.to_string()))?)
// }

// #[wasm_bindgen]
// pub fn parse_vorgang_xml(xml: String) -> Result<Vorgang, JsValue> {
//     Ok(raxb::de::from_str(&xml).map_err(|err| JsValue::from_str(&err.to_string()))?)
// }
