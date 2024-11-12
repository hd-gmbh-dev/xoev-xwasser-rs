#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

use xoev_xwasser_derive::XWasserValidate;

#[cfg(feature = "wasm")]
use tsify_next::Tsify;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

use crate::model::codes::CodeDatentypType;

use super::zeitraum::ZeitraumType;

/// Unter "Identifikation" werden die Informationen zusammengefasst, die die eindeutige
/// Identifikation von Objekten in einem fachlichen Kontext erlauben.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_7_0/"
))]
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
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_7_0/"
))]
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
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_7_0/"
))]
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
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_7_0/"
))]
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
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_7_0/"
))]
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
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_7_0/"
))]
pub struct GeokoordinatenShapthType {
    #[xml(
        ns = b"xwas",
        name = b"geografischePositionUndAusdehnung",
        ty = "child"
    )]
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
