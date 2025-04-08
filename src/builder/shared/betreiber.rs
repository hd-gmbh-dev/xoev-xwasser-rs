use crate::{
    builder::utils::{new_id, new_uuid},
    model::shared::betreiber::{BetreiberType, ObjektType},
};

// use serde::{Deserialize, Serialize};

// use typed_builder::TypedBuilder;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::organisation::organisation_type;

// #[cfg(feature = "wasm")]
// use tsify_next::Tsify;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn objekt_type() -> ObjektType {
    let id = format!("obj-{}", new_id());
    let mut betreiber = betreiber_type();
    betreiber.objekt_id = vec![id.clone()];
    ObjektType::builder()
        .objekt_id(new_uuid())
        .anlage_nach_trinkw_vid(Default::default())
        .anschrift_objekt(Default::default())
        .art_objekt("1010".into())
        .name_objekt(Default::default())
        .betriebszustand_des_objekts(Default::default())
        .datum_in_betriebnahme(Default::default())
        .datum_ausser_betriebnahme(Default::default())
        .rahmen_der_trinkwasserbereitstellung(Default::default())
        .geokoordinaten_objekt(Default::default())
        .angaben_alternative_id(Default::default())
        .kommentar(Default::default())
        .betreiber(vec![betreiber])
        .wasserversorgungsgebiet_id(Default::default())
        .id(id)
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn betreiber_type() -> BetreiberType {
    BetreiberType::builder()
        .betreiber_id(new_uuid())
        .art_der_person(crate::model::shared::betreiber::ArtDerPerson::Organisation(
            organisation_type(),
        ))
        .objekt_id(Default::default())
        .kommentar(Default::default())
        .id(format!("betreiber-{}", new_id()))
        .build()
}
