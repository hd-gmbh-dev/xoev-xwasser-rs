#![allow(non_snake_case, dead_code)]

use raxb::{value::ConstStr, XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

use xoev_xwasser_derive::XWasserValidate;

#[cfg(feature = "wasm")]
use tsify::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

use crate::TNS;

use super::{
    codes::CodeStatusTechnischType, transport::NachrichtenkopfG2g, vorgang::IdentifikationVorgang,
};

/// Mit dieser Nachricht wird eine Quittung transportiert. Diese wird durch jede Nachricht ausgelöst. Die ID der Quittungsnachricht wird im Feld nachrichtenkopf/identifikation.nachricht/nachrichtenUUID und die ID der Nachricht, die quittiert wird, wird im Feld nachrichtenkopf/referenzUUID abgelegt. Auf eine Nachricht vom Typ administration.quittung.0020 wird keine Antwort mehr übermittelt, auch keine weitere Quittungsnachricht.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(root = b"administration.quittung.0020")]
#[xml(tns(b"xwas", TNS))]
pub struct AdministrationQuittung0020 {
    #[xml(
        ns = b"xmlns",
        name = b"xsi",
        ty = "attr",
        value = "http://www.w3.org/2001/XMLSchema-instance"
    )]
    #[serde(skip)]
    #[cfg_attr(feature = "builder", builder(default))]
    _xmlns_xsi: ConstStr,
    #[xml(
        ns = b"xsi",
        name = b"schemaLocation",
        ty = "attr",
        value = "https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_9_5/ xwasser.xsd"
    )]
    #[serde(skip)]
    #[cfg_attr(feature = "builder", builder(default))]
    schema_location: ConstStr,
    #[xml(
        ns = b"xmlns",
        name = b"xwas",
        ty = "attr",
        value = "https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_9_5/"
    )]
    #[serde(skip)]
    _xmlns: ConstStr,
    #[xml(name = b"produkt", ty = "attr")]
    pub produkt: String,
    #[xml(name = b"produkthersteller", ty = "attr")]
    pub produkthersteller: String,
    #[xml(name = b"produktversion", ty = "attr")]
    pub produktversion: String,
    #[xml(name = b"standard", ty = "attr", value = "XWasser")]
    #[serde(skip)]
    #[cfg_attr(feature = "builder", builder(default))]
    _standard: ConstStr,
    #[xml(name = b"test", ty = "attr")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub test: Option<bool>,
    #[xml(name = b"version", ty = "attr", value = "0.9.5")]
    #[serde(skip)]
    #[cfg_attr(feature = "builder", builder(default))]
    _version: ConstStr,

    #[xml(name = b"nachrichtenkopf.g2g", ty = "child")]
    pub nachrichtenkopf_g2g: NachrichtenkopfG2g,

    /// Eindeutige Identifizierung des Vorgangs.
    #[xml(ns = b"xwas", name = b"identifikationVorgang", ty = "child")]
    pub identifikation_vorgang: IdentifikationVorgang,
    /// Mit diesem Element wird eine Nachricht in Form eines technischen Status quittiert.
    #[xml(ns = b"xwas", name = b"quittung", ty = "child")]
    pub quittung: QuittungType,
}

/// Dieses Objekt enthält Informationen zum technischen Prozessstatus, die zur Quittierung eingegangene Nachrichten dienen.
#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct QuittungType {
    /// Mit dem Code-Datentyp Code.StatusTechnisch wird der aktuelle technische Prozess-Status übermittelt.
    #[xml(ns = b"xwas", name = b"aktuellerStatusTechnisch", ty = "child")]
    pub aktueller_status_technisch: CodeStatusTechnischType,
    /// Alter (vorhergehender) technischer Prozess-Status, nur informativ.
    #[xml(ns = b"xwas", name = b"vorherigerStatusTechnisch", ty = "child")]
    pub vorheriger_status_technisch: Option<CodeStatusTechnischType>,
    /// Grund des Status. Freie Textbeschreibung. Wird nicht maschinell ausgewertet.
    #[xml(ns = b"xwas", name = b"grund", ty = "child")]
    pub grund: Option<String>,
}
