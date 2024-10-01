use crate::model::transport::{BehoerdeG2GType, Code, IdentifikationNachricht, NachrichtenTyp, Verzeichnisdienst};

use super::utils::{new_uuid, now};

pub fn behoerde_g2g_type<S1: Into<String>, S2: Into<String>>(name: S1, kennung: S2) -> BehoerdeG2GType {
    BehoerdeG2GType::builder()
        .name(name.into())
        .kennung(kennung.into())
        .verzeichnisdienst(Verzeichnisdienst::builder().code("".into()).build())
        .build()
}

pub use behoerde_g2g_type as autor;
pub use behoerde_g2g_type as leser;

pub fn identifikation_nachricht<S: Into<String>>(typ: S) -> IdentifikationNachricht {
    let nachrichten_typ = NachrichtenTyp::builder().code(Code {
        code: typ.into(),
    }).build();
    IdentifikationNachricht::builder()
        .nachrichten_typ(Some(nachrichten_typ))
        .erstellungszeitpunkt(Some(now()))
        .nachrichten_uuid(Some(new_uuid()))
        .build()
}

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn identifikation_nachricht(typ: String) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&super::identifikation_nachricht(typ))?)
    }

    #[wasm_bindgen]
    pub fn autor(name: String, kennung: String) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&super::autor(name, kennung))?)
    }

    #[wasm_bindgen]
    pub fn leser(name: String, kennung: String) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&super::leser(name, kennung))?)
    }
}
