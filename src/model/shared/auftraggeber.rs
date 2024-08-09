#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::model::codes::CodeAuftraggeberartType;

use super::{
    behoerde::ZustaendigeBehoerdeType, organisation::OrganisationType,
    person::NatuerlichePersonType,
};

/// Informationen zu einem Auftraggeber [Ergänzende Angaben zu den jeweiligen
/// Informationen aus den Registern von Betreibern/Behörden].
#[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
))]
pub struct AuftraggeberType {
    #[xml(ns = b"xwas", name = b"auftraggeberID", ty = "child")]
    pub auftraggeber_id: String,
    #[xml(ns = b"xwas", name = b"auftraggeberart", ty = "child")]
    pub auftraggeberart: CodeAuftraggeberartType,
    #[xml(ns = b"xwas", name = b"auftraggeber", ty = "child")]
    pub auftraggeber: Auftraggeber,
}

// TODO: implement Box<T>, Arc<T>, Rc<T> for raxb
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[serde(tag = "t", content = "c")]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
))]
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
