use crate::model::shared::misc::IdentifikationType;

// use serde::{Deserialize, Serialize};

// use typed_builder::TypedBuilder;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::zeitraum::zeitraum_type;

// #[cfg(feature = "wasm")]
// use tsify::Tsify;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn identifikation_type() -> IdentifikationType {
    IdentifikationType::builder()
        .id(Default::default())
        .beschreibung(Default::default())
        .gueltigkeit(Some(zeitraum_type()))
        .build()
}
