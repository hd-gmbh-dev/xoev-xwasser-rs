use crate::{builder::utils::new_id, model::shared::anschrift::AnschriftType};

// use serde::{Deserialize, Serialize};

// use typed_builder::TypedBuilder;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

// #[cfg(feature = "wasm")]
// use tsify::Tsify;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn anschrift_type(
    strasse: String,
    hausnummer: String,
    postleitzahl: String,
    ort: String,
) -> AnschriftType {
    AnschriftType::builder()
        .strasse(Some(strasse))
        .hausnummer(Some(hausnummer))
        .postleitzahl(Some(postleitzahl))
        .ort(Some(ort))
        .strassenschluessel(Default::default())
        .postfach(Default::default())
        .ortsteil(Default::default())
        .ort_frueherer_gemeindename(Default::default())
        .wohnungsgeber(Default::default())
        .zusatz(Default::default())
        .typ(Default::default())
        .staat(Default::default())
        .verwaltungspolitische_kodierung(Default::default())
        .id(format!("anschrift-{}", new_id()))
        .build()
}
