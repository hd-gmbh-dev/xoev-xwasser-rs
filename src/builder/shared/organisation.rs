use crate::{
    builder::utils::new_id,
    model::shared::organisation::{Name, NameOrganisationType, OrganisationType},
};

// use serde::{Deserialize, Serialize};

// use typed_builder::TypedBuilder;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::zeitraum::zeitraum_type;

// #[cfg(feature = "wasm")]
// use tsify_next::Tsify;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn name_organisation_type(
    name: Option<String>,
    kurzbezeichnung: Option<String>,
) -> NameOrganisationType {
    NameOrganisationType::builder()
        .name(Some(Name::builder().text(name).build()))
        .kurzbezeichnung(Some(kurzbezeichnung.unwrap_or_default()))
        .gueltigkeit(Some(zeitraum_type()))
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn organisation_type() -> OrganisationType {
    OrganisationType::builder()
        .id(format!("organisation-{}", new_id()).into())
        .rechtsform(Default::default())
        .branche(Default::default())
        .zweck(Default::default())
        .name(Default::default())
        .unterorganisation(Default::default())
        .kommunikation(Default::default())
        .registrierung(Default::default())
        .identifikation(Default::default())
        .existenzzeitraum(Default::default())
        .anschrift(Default::default())
        .build()
}
