use crate::model::transport::{BehoerdeG2GType, Code, IdentifikationNachricht, NachrichtenTyp, Verzeichnisdienst};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, Display};
use super::utils::{new_uuid, now};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use tsify_next::Tsify;

pub fn behoerde_g2g_type<S1: Into<String>, S2: Into<String>>(name: S1, kennung: S2) -> BehoerdeG2GType {
    BehoerdeG2GType::builder()
        .name(name.into())
        .kennung(kennung.into())
        .verzeichnisdienst(Verzeichnisdienst::builder().code("".into()).build())
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn identifikation_nachricht(typ: NachrichtenTypEnum) -> IdentifikationNachricht {
    let nachrichten_typ = NachrichtenTyp::builder().code(Code {
        code: typ.to_string(),
    }).build();
    IdentifikationNachricht::builder()
        .nachrichten_typ(Some(nachrichten_typ))
        .erstellungszeitpunkt(Some(now()))
        .nachrichten_uuid(Some(new_uuid()))
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn autor(name: String, kennung: String) -> BehoerdeG2GType {
    behoerde_g2g_type(name, kennung)
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn leser(name: String, kennung: String) -> BehoerdeG2GType {
    behoerde_g2g_type(name, kennung)
}

#[derive(Debug, PartialEq, EnumString, Display, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub enum NachrichtenTypEnum {
    #[strum(serialize = "0010")]
    AdministrationRueckweisung0010,
    #[strum(serialize = "0020")]
    AdministrationQuittung0020,
    #[strum(serialize = "1010")]
    WeiterleitungWeiterleitung1010,
    #[strum(serialize = "1020")]
    WeiterleitungAbgabe1020,
    #[strum(serialize = "1030")]
    WeiterleitungNichtzustaendigkeit1030,
    #[strum(serialize = "2010")]
    VorgangTransportieren2010,
    #[strum(serialize = "2020")]
    VorgangNachricht2020,
    #[strum(serialize = "2030")]
    VorgangStatusanfrage2030,
    #[strum(serialize = "2040")]
    VorgangStatusantwort2040,
}
