use crate::model::shared::person::{
    AllgemeinerNameType, NameNatuerlichePersonType, NatuerlichePersonType,
};

// use serde::{Deserialize, Serialize};

// use typed_builder::TypedBuilder;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

// #[cfg(feature = "wasm")]
// use tsify_next::Tsify;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn allgemeiner_name_type(name: String) -> AllgemeinerNameType {
    AllgemeinerNameType::builder()
        .name(Some(name))
        .nicht_vorhanden(Default::default())
        .namensart(Default::default())
        .alternative_repraesentation(Default::default())
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn natuerliche_person_type(vorname: String, familienname: String) -> NatuerlichePersonType {
    let vorname = allgemeiner_name_type(vorname);
    let familienname = allgemeiner_name_type(familienname);
    let name_natuerliche_person = NameNatuerlichePersonType::builder()
        .titel(Default::default())
        .anrede(Default::default())
        .namenssuffix(Default::default())
        .familienname(Some(familienname))
        .ehename(Default::default())
        .lebenspartnerschaftsname(Default::default())
        .geburtsname(Default::default())
        .frueherer_familienname(Default::default())
        .vorname(Some(vorname))
        .rufname(Default::default())
        .frueherer_vorname(Default::default())
        .alternative_repraesentation(Default::default())
        .ordensname(Default::default())
        .kuenstlername(Default::default())
        .weiterer_name(Default::default())
        .build();
    NatuerlichePersonType::builder()
        .auskunftssperre(Default::default())
        .name_natuerliche_person(Some(name_natuerliche_person))
        .familienstand(Default::default())
        .geburt(Default::default())
        .doktorgrad(Default::default())
        .staatsangehoerigkeit(Default::default())
        .ausweisdokument(Default::default())
        .anschrift(Default::default())
        .geschlecht(Default::default())
        .identifikationsnummer(Default::default())
        .kommunikation(Default::default())
        .muttersprache(Default::default())
        .fremdsprache(Default::default())
        .vertreter_bevollmaechtigter(Default::default())
        .id(format!("person-{}", crate::builder::utils::new_id()).into())
        .build()
}
