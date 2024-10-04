use super::utils::new_uuid;
use crate::model::vorgang::IdentifikationVorgang;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn identifikation_vorgang(id: Option<String>) -> IdentifikationVorgang {
    IdentifikationVorgang::builder()
        .vorgangs_id(id.unwrap_or_else(new_uuid))
        .build()
}
