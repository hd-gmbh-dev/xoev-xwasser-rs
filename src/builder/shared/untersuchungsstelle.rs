use serde::{Deserialize, Serialize};

// use typed_builder::TypedBuilder;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use tsify_next::Tsify;

use crate::{
    builder::utils::new_id,
    model::shared::{
        organisation::OrganisationType,
        unterssuchungsstelle::{
            BeauftragteUntersuchungsstelleType, ZugelasseneUntersuchungsstelleType,
        },
    },
};

use super::{organisation::name_organisation_type, zeitraum::zeitraum_type};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct UnterorganisationDetails {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct UntersuchungsstelleDetails {
    pub id: String,
    pub name: String,
    pub zugelassene_untersuchungsstelle_id: String,
    pub pruefgebiete_untersuchungen_phys_chem: bool,
    pub pruefgebiete_untersuchungen_mikrobio: bool,
    pub pruefgebiete_untersuchungen_radionuklide: bool,
    pub pruefgebiete_nur_vor_ort_parameter: bool,
    pub akkreditierungsnummer: String,
    pub unterorganisation: Option<UnterorganisationDetails>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn zugelassene_untersuchungsstelle_type(
    details: UntersuchungsstelleDetails,
) -> ZugelasseneUntersuchungsstelleType {
    let id = format!("untersuchungsstelle-{}", new_id());
    ZugelasseneUntersuchungsstelleType::builder()
        .id(id.clone().into())
        .rechtsform(Default::default())
        .branche(Default::default())
        .zweck(Default::default())
        .name(Some(name_organisation_type(
            Some(details.name),
            Some(details.id),
        )))
        .unterorganisation(
            details
                .unterorganisation
                .map(|val| {
                    vec![OrganisationType::builder()
                        .id(format!("{id}-unterorganisation-{}", new_id()).into())
                        .rechtsform(Default::default())
                        .branche(Default::default())
                        .zweck(Default::default())
                        .name(Some(name_organisation_type(Some(val.name), Some(val.id))))
                        .unterorganisation(Default::default())
                        .kommunikation(Default::default())
                        .registrierung(Default::default())
                        .identifikation(Default::default())
                        .existenzzeitraum(Some(zeitraum_type()))
                        .anschrift(Default::default())
                        .build()]
                })
                .unwrap_or_default(),
        )
        .kommunikation(Default::default())
        .registrierung(Default::default())
        .identifikation(Default::default())
        .existenzzeitraum(Some(zeitraum_type()))
        .anschrift(Default::default())
        .zugelassene_untersuchungsstelle_id(details.zugelassene_untersuchungsstelle_id)
        .pruefgebiete_untersuchungen_phys_chem(Some(details.pruefgebiete_untersuchungen_phys_chem))
        .pruefgebiete_untersuchungen_mikrobio(Some(details.pruefgebiete_untersuchungen_mikrobio))
        .pruefgebiete_untersuchungen_radionuklide(Some(
            details.pruefgebiete_untersuchungen_radionuklide,
        ))
        .pruefgebiete_nur_vor_ort_parameter(Some(details.pruefgebiete_nur_vor_ort_parameter))
        .akkreditierungsnummer(Some(details.akkreditierungsnummer))
        .kommentar_zugelassene_untersuchungsstelle(Default::default())
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn beauftragte_untersuchungsstelle_type(
    details: UntersuchungsstelleDetails,
) -> BeauftragteUntersuchungsstelleType {
    let id = format!("untersuchungsstelle-{}", new_id());
    BeauftragteUntersuchungsstelleType::builder()
        .id(id.clone().into())
        .rechtsform(Default::default())
        .branche(Default::default())
        .zweck(Default::default())
        .name(Some(name_organisation_type(
            Some(details.name),
            Some(details.id),
        )))
        .unterorganisation(
            details
                .unterorganisation
                .map(|val| {
                    vec![OrganisationType::builder()
                        .id(format!("{id}-unterorganisation-{}", new_id()).into())
                        .rechtsform(Default::default())
                        .branche(Default::default())
                        .zweck(Default::default())
                        .name(Some(name_organisation_type(Some(val.name), Some(val.id))))
                        .unterorganisation(Default::default())
                        .kommunikation(Default::default())
                        .registrierung(Default::default())
                        .identifikation(Default::default())
                        .existenzzeitraum(Some(zeitraum_type()))
                        .anschrift(Default::default())
                        .build()]
                })
                .unwrap_or_default(),
        )
        .kommunikation(Default::default())
        .registrierung(Default::default())
        .identifikation(Default::default())
        .existenzzeitraum(Some(zeitraum_type()))
        .anschrift(Default::default())
        .zugelassene_untersuchungsstelle_id(details.zugelassene_untersuchungsstelle_id)
        .pruefgebiete_untersuchungen_phys_chem(Some(details.pruefgebiete_untersuchungen_phys_chem))
        .pruefgebiete_untersuchungen_mikrobio(Some(details.pruefgebiete_untersuchungen_mikrobio))
        .pruefgebiete_untersuchungen_radionuklide(Some(
            details.pruefgebiete_untersuchungen_radionuklide,
        ))
        .pruefgebiete_nur_vor_ort_parameter(Default::default())
        .akkreditierungsnummer(Some(details.akkreditierungsnummer))
        .kommentar_beauftragte_untersuchungsstelle(Default::default())
        .kommentar_zugelassene_untersuchungsstelle(Default::default())
        .build()
}
