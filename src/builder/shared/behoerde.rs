use crate::{
    builder::utils::new_id,
    model::shared::{
        behoerde::{BehoerdeType, BehoerdenkennungType, ZustaendigeBehoerdeType},
        xoev::XWasserXoevCode,
    },
};

// use serde::{Deserialize, Serialize};

// use typed_builder::TypedBuilder;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::{misc::identifikation_type, organisation::name_organisation_type};

// #[cfg(feature = "wasm")]
// use tsify_next::Tsify;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn behoerde_type() -> BehoerdeType {
    BehoerdeType::builder()
        .id(format!("behoerde-{}", new_id()).into())
        .typ(Some(XWasserXoevCode::builder().code("".into()).build()))
        .zusatz(Default::default())
        .behoerdenkennung(
            BehoerdenkennungType::builder()
                .kennung(Default::default())
                .praefix(Default::default())
                .build()
                .into(),
        )
        .kommunikation(Default::default())
        .behoerdenidentifikation(Some(identifikation_type()))
        .behoerdenname(Some(name_organisation_type(None, None)))
        .nachgeordnete_behoerde(Default::default())
        .verwaltungspolitische_zustaendigkeit(Default::default())
        .anschrift(Default::default())
        .organisationsstruktur(Default::default())
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn zustaendige_behoerde_type(laenderkuerzel: String) -> ZustaendigeBehoerdeType {
    ZustaendigeBehoerdeType::builder()
        .id(format!("behoerde-{}", new_id()).into())
        .typ(Some(XWasserXoevCode::builder().code("".into()).build()))
        .zusatz(Default::default())
        .behoerdenkennung(
            BehoerdenkennungType::builder()
                .kennung(Default::default())
                .praefix(Default::default())
                .build()
                .into(),
        )
        .kommunikation(Default::default())
        .behoerdenidentifikation(Some(identifikation_type()))
        .behoerdenname(Some(name_organisation_type(None, None)))
        .nachgeordnete_behoerde(Default::default())
        .verwaltungspolitische_zustaendigkeit(Default::default())
        .anschrift(Default::default())
        .organisationsstruktur(Default::default())
        .anlage_nach_trinkw_vid(Default::default())
        .laenderkuerzel(laenderkuerzel.into())
        .kommentar(Default::default())
        .build()
}
