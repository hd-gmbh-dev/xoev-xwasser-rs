#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::model::codes::CodeDokumenttypType;

/// Sofern das Dokument signiert wurde findet sich hier eine Referenzliste auf die
/// Signaturen. Diese sind ihrerseits wieder Dokumente, die auch als Dokumentelemente in
/// der Nachricht zu finden sind.
#[derive(Clone, Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_6_0/"
))]
pub struct SignaturenType {
    #[xml(ns = b"xwas", name = b"signaturDokumentID", ty = "child")]
    #[serde(default)]
    pub signatur_dokument_id: Vec<String>,
}

/// Dokument-Repräsentation, eine Darstellungsform des Dokumentes. Es muss das Element
/// content oder die Elemente externalReferenceIndex und externalReferenceType oder alle
/// 3 vorhanden sein.
#[derive(Clone, Debug, Default, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_6_0/"
))]
pub struct DokumentRepraesentationType {
    #[xml(ns = b"xwas", name = b"referenz", ty = "child")]
    pub referenz: Option<String>,
    #[xml(ns = b"xwas", name = b"mimeType", ty = "child")]
    pub mime_type: String,
    #[xml(ns = b"xwas", name = b"inhalt", ty = "child")]
    pub inhalt: Option<String>,
    #[xml(ns = b"xwas", name = b"externerReferenzTyp", ty = "child")]
    pub externer_referenz_typ: Option<String>,
    #[xml(ns = b"xwas", name = b"externerReferenzIndex", ty = "child")]
    pub externer_referenz_index: Option<String>,
    #[xml(ns = b"xwas", name = b"dateiname", ty = "child")]
    pub dateiname: Option<String>,
    #[xml(ns = b"xwas", name = b"inhaltTyp", ty = "child")]
    pub inhalt_typ: Option<String>,
    #[xml(ns = b"xwas", name = b"signaturen", ty = "child")]
    pub signaturen: Option<SignaturenType>,
    #[xml(name = b"dokumentRepraesentationID", ty = "attr")]
    pub dokument_repraesentation_id: String,
}

/// Eine zum Antrag gehörige Unterlage in verschiedenen Dokumentendarstellungen, z. B.
/// PDF oder eine Datendarstellung. Es muss immer das komplette Dokument mit allen
/// Darstellungen übertragen werden.
#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_6_0/"
))]
pub struct DokumentType {
    #[xml(ns = b"xwas", name = b"dokumentTyp", ty = "child")]
    pub dokument_typ: CodeDokumenttypType,
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    pub name: String,
    #[xml(ns = b"xwas", name = b"aktuelleVersion", ty = "child")]
    pub aktuelle_version: Option<String>,
    #[xml(ns = b"xwas", name = b"letzteVersion", ty = "child")]
    pub letzte_version: Option<String>,
    #[xml(ns = b"xwas", name = b"dokumentRepraesentation", ty = "child")]
    #[serde(default)]
    pub dokument_repraesentation: Vec<DokumentRepraesentationType>,
    #[xml(ns = b"xwas", name = b"personReferenzID", ty = "child")]
    #[serde(default)]
    pub person_referenz_id: Vec<String>,
    #[xml(ns = b"xwas", name = b"dokumentID", ty = "attr")]
    pub dokument_id: String,
}
