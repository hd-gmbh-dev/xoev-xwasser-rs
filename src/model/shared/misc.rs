#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

use xoev_xwasser_derive::XWasserValidate;

#[cfg(feature = "wasm")]
use tsify::Tsify;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

use crate::{
    TNS,
    model::codes::{
        CodeDatentypType, CodeFormatAlternativeIdGesundheitType, CodeFormatAlternativeIdUmweltType,
    },
};

use super::{xoev::XWasserXoevCode, zeitraum::ZeitraumType};

/// Unter "Identifikation" werden die Informationen zusammengefasst, die die eindeutige
/// Identifikation von Objekten in einem fachlichen Kontext erlauben.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct IdentifikationType {
    #[xml(ns = b"xwas", name = b"id", ty = "child")]
    pub id: Option<String>,
    #[xml(ns = b"xwas", name = b"beschreibung", ty = "child")]
    pub beschreibung: Option<String>,
    #[xml(ns = b"xwas", name = b"gueltigkeit", ty = "child")]
    pub gueltigkeit: Option<ZeitraumType>,
}

/// Erweiterung.XML darf nur dazu genutzt werden, weitere (z. B. fachspezifische)
/// Metadaten zu spezifizieren, deren Übermittlung mit den bereits in XWasser
/// spezifizierten Metadaten nicht möglich ist. Erweiterung.XML bietet über ein
/// xs:any-Element die Möglichkeit, mittels Einbindung externer XML-Schemata diese
/// Metadaten zu spezifizieren. Es können beliebige XML-Schemata mit unterschiedlichen
/// Namensräumen angegeben werden. Die XML-Schema-Validierung der weiterführenden
/// Metadaten erfolgt innerhalb der XWasser-Nachricht selbst (prozessContents = "lax").
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct ErweiterungXmlType {
    // weiteres xml schema
    #[xml(ns = b"xwas", name = b"any", ty = "child")]
    #[serde(default)]
    pub any: Vec<String>,
}

/// Ein Feld ist ein anwendungsspezifisches Metadatum. Die Konfiguration eines Feldes
/// muss zwischen den Kommunikationspartnern abgesprochen sein.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
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
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct ErweiterungGruppeType {
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: String,
    #[xml(ns = b"xwas", name = b"beschreibung", ty = "child")]
    pub beschreibung: String,
    #[xml(ns = b"xwas", name = b"untergruppe", ty = "child")]
    #[serde(default)]
    pub untergruppe: Vec<ErweiterungGruppeType>,
    #[xml(ns = b"xwas", name = b"feld", ty = "child")]
    #[serde(default)]
    pub feld: Vec<ErweiterungFeldType>,
}

/// Die Klasse Erweiterung dient zur Übertragung generischer Informationen. Die
/// Verwendung ist für folgende Fälle vorgesehen: Es müssen zeitnah Informationen
/// übertragen werden (zum Beispiel aufgrund einer Gesetzesänderung), für die es in der
/// aktuellen Version des Standards noch keine geeigneten Klassen und Elemente gibt.
/// Zwischen Sender und Empfänger wird bilateral die Übermittlung einer strukturierten
/// Information vereinbart, die sich mit den vorhandenen Klassen und Elemente nicht
/// abbilden lässt.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct ErweiterungType {
    #[xml(ns = b"xwas", name = b"feld", ty = "child")]
    #[serde(default)]
    pub feld: Vec<ErweiterungFeldType>,
    #[xml(ns = b"xwas", name = b"gruppe", ty = "child")]
    #[serde(default)]
    pub gruppe: Vec<ErweiterungGruppeType>,
    #[xml(ns = b"xwas", name = b"xml", ty = "child")]
    pub xml: Option<ErweiterungXmlType>,
}

/// Klasse zur Abbildung von SHAPTH-spezifischen Geokoordinaten.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct GeografischeAngabenType {
    #[xml(
        ns = b"xwas",
        name = b"geografischePositionUndAusdehnung",
        ty = "child"
    )]
    pub geografische_position_und_ausdehnung: Option<String>,
    #[xml(ns = b"xwas", name = b"nameShapefile", ty = "child")]
    pub name_shapefile: Option<String>,
    #[xml(ns = b"xwas", name = b"geokoordinatenZone", ty = "child")]
    pub geokoordinaten_zone: Option<String>,
    #[xml(ns = b"xwas", name = b"geokoordinatenOstwert", ty = "child")]
    pub geokoordinaten_ostwert: Option<f64>,
    #[xml(ns = b"xwas", name = b"geokoordinatenNordwert", ty = "child")]
    pub geokoordinaten_nordwert: Option<f64>,
    #[xml(ns = b"xwas", name = b"geokoordinatenBreitengrad", ty = "child")]
    pub geokoordinaten_breitengrad: Option<f64>,
    #[xml(ns = b"xwas", name = b"geokoordinatenLaengengrad", ty = "child")]
    pub geokoordinaten_laengengrad: Option<f64>,
}

/// Die Komponente "Geokodierung" beinhaltet Informationen zur geografischen Bestimmung von Dingen.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct GeokodierungType {
    #[xml(ns = b"xwas", name = b"koordinatensystem", ty = "child")]
    pub koordinatensystem: Option<XWasserXoevCode>,
    #[xml(ns = b"xwas", name = b"koordinate1", ty = "child")]
    pub koordinate1: Option<f64>,
    #[xml(ns = b"xwas", name = b"koordinate2", ty = "child")]
    pub koordinate2: Option<f64>,
    #[xml(ns = b"xwas", name = b"flurnummer", ty = "child")]
    pub flurnummer: Option<String>,
    #[xml(ns = b"xwas", name = b"flurstueck", ty = "child")]
    pub flurstueck: Option<String>,
    #[xml(ns = b"xwas", name = b"gemarkung", ty = "child")]
    pub gemarkung: Option<String>,
}

/// Mit dieser Klasse wird die Alternative ID_Gesundheit und das Format, bzw. der Typ, dem die Alternative ID_Gesundheit folgt, übertragen.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct AngabenAlternativeIdGesundheitType {
    #[xml(ns = b"xwas", name = b"alternativeIDGesundheit", ty = "child")]
    pub alternative_id_gesundheit: Option<String>,
    #[xml(ns = b"xwas", name = b"formatAlternativeIDGesundheit", ty = "child")]
    pub format_alternative_id_gesundheit: Option<CodeFormatAlternativeIdGesundheitType>,
}

/// Mit dieser Klasse wird die Alternative ID_Umwelt und die Herkunft des dazugehörigen Schemas übertragen.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct AngabenAlternativeIdUmweltType {
    #[xml(ns = b"xwas", name = b"alternativeIDUmwelt", ty = "child")]
    pub alternative_id_umwelt: String,
    #[xml(ns = b"xwas", name = b"formatAlternativeIDUmwelt", ty = "child")]
    pub format_alternative_id_umwelt: CodeFormatAlternativeIdUmweltType,
}

/// Mit dieser Klasse wird eine für die Überwachung von Sollproben zuständige Behörde spezifiziert.
/// Bei grenzübergreifenden Wasserversorgungsgebieten bzw. Wasserversorgungsanlagen kann es sein,
/// dass mehrere Behörden sich die Solluntersuchungen nach Berichtsplan untereinander aufteilen müssen.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct UeberwachendeBehoerdeType {
    #[xml(ns = b"xwas", name = b"zustaendigeBehoerde", ty = "child")]
    pub zustaendige_behoerde: String,
    #[xml(ns = b"xwas", name = b"untersuchungenGruppeA", ty = "child")]
    pub untersuchungen_gruppe_a: Option<i32>,
    #[xml(ns = b"xwas", name = b"untersuchungenGruppeB", ty = "child")]
    pub untersuchungen_gruppe_b: Option<i32>,
}

/// Mit dieser Klasse wird ein Ortsteil spezifiziert, der im Wasserversorgungsgebiet liegt bzw.
/// von der Wasserversorgungsanlage mit Wasser versorgt wird.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct VersorgterOrtsteilType {
    #[xml(ns = b"xwas", name = b"nameOrtsteil", ty = "child")]
    pub name_ortsteil: String,
    #[xml(ns = b"xwas", name = b"ortsteilID", ty = "child")]
    pub ortsteil_id: String,
    #[xml(ns = b"xwas", name = b"anzahlVersorgterPersonen", ty = "child")]
    pub anzahl_versorgter_personen: i32,
}

/// Mit dieser Klasse können zusätzliche fachliche Informationen transportiert werden.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct ZusatzinformationenType {
    #[xml(ns = b"xwas", name = b"zustaendigeBehoerdeID", ty = "child")]
    #[serde(default)]
    pub zustaendige_behoerde_id: Vec<String>,
    #[xml(ns = b"xwas", name = b"wasserversorgungsgebietID", ty = "child")]
    pub wasserversorgungsgebiet_id: Option<String>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
}
