#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

use xoev_xwasser_derive::XWasserValidate;

#[cfg(feature = "wasm")]
use tsify::Tsify;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

use crate::{TNS, model::codes::CodeWaehrungType};

use super::{misc::ErweiterungType, xoev::XWasserXoevCode, zeitraum::ZeitraumType};

/// Diese Klasse enthält alle Informationen für den Transport einer freien Nachricht.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct FreieNachrichtType {
    #[xml(ns = b"xwas", name = b"rcvrPartnerID", ty = "child")]
    pub rcvr_partner_id: String,
    #[xml(ns = b"xwas", name = b"sendPartnerID", ty = "child")]
    pub send_partner_id: Option<String>,
    #[xml(ns = b"xwas", name = b"inhalt", ty = "child")]
    pub inhalt: Option<String>,
    #[xml(ns = b"xwas", name = b"erweiterung", ty = "child")]
    pub erweiterung: Option<ErweiterungType>,
}

/// Mit diesem Datentyp wird ein Betrag in numerischer Form abgebildet.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct BetragType {
    #[xml(ns = b"xwas", name = b"betrag", ty = "child")]
    pub betrag: f64,
    #[xml(ns = b"xwas", name = b"waehrung", ty = "child")]
    pub waehrung: Option<CodeWaehrungType>,
}

/// Diese Klasse dient zur Übertragung eines Kurzsteckbriefs für Nachrichten.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct IndexType {
    #[xml(ns = b"xwas", name = b"nachrichtUUID", ty = "child")]
    pub nachricht_uuid: String,
    #[xml(ns = b"xwas", name = b"sender", ty = "child")]
    pub sender: Option<String>,
    #[xml(ns = b"xwas", name = b"empfaenger", ty = "child")]
    pub empfaenger: Option<String>,
    #[xml(ns = b"xwas", name = b"autor", ty = "child")]
    pub autor: Option<String>,
}

/// Diese Klasse enthält die Suchkriterien zur Abfrage auf vorhandene Vorgänge/Nachrichten.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct IndexanfrageType {
    #[xml(ns = b"xwas", name = b"kriterium", ty = "child")]
    pub kriterium: String,
    #[xml(ns = b"xwas", name = b"bereich", ty = "child")]
    #[serde(default)]
    pub bereich: Vec<XWasserXoevCode>,
    #[xml(ns = b"xwas", name = b"zeitraum", ty = "child")]
    pub zeitraum: Option<ZeitraumType>,
}
