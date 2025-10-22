use crate::model::shared::zeitraum::ZeitraumType;

// use serde::{Deserialize, Serialize};

// use typed_builder::TypedBuilder;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

// #[cfg(feature = "wasm")]
// use tsify::Tsify;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn zeitraum_type() -> ZeitraumType {
    ZeitraumType::builder()
        .beginn(Some("2024-01-01".into()))
        .ende(Some("2099-01-01".into()))
        .zusatz(Default::default())
        .build()
}
