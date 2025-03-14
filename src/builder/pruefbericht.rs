use crate::{
    builder::{
        shared::untersuchungsstelle::UntersuchungsstelleDetails,
        utils::{new_id, new_uuid, now},
    },
    model::{
        pruefbericht::PruefberichtType,
        shared::auftraggeber::{Auftraggeber, AuftraggeberType},
        signature::*,
    },
};

// use serde::{Deserialize, Serialize};

// use typed_builder::TypedBuilder;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::shared::{
    organisation::organisation_type, person::natuerliche_person_type, probe::probennehmer_type,
    untersuchungsstelle::beauftragte_untersuchungsstelle_type,
};

// #[cfg(feature = "wasm")]
// use tsify_next::Tsify;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn pruefbericht_type(
    sw_version: String,
    id: Option<String>,
    context: String,
    details: UntersuchungsstelleDetails,
) -> PruefberichtType {
    PruefberichtType::builder()
        .pruefbericht_uuid(id.unwrap_or_else(new_uuid))
        .versionsnummer(Some(1))
        .auftragsnummer(new_uuid())
        .probennahmestelle(Default::default())
        .probe(Default::default())
        .probennehmer(vec![probennehmer_type()])
        .pruefbericht_enthaelt_teilergebnisse(Default::default())
        .korrekturvermerk(Default::default())
        .pruefbericht_gem_vorgaben_akkredition(Default::default())
        .titel("Prüfbericht Titel".into())
        .gesamtbewertung("1010".into())
        .auffaelligkeiten(Default::default())
        .zeitpunkt_validierung_pruefbericht(now())
        .fuer_validierung_verantwortliche_person(vec![natuerliche_person_type(
            "".into(),
            "".into(),
        )])
        .freigabe_uebermittlung_betreiber(true)
        .pruefbericht_id_labor(Default::default())
        .sw_version(sw_version)
        .sprache_pruefbericht("de".into())
        .rechtlicher_disclaimer(Default::default())
        .zeitpunkt_uebermittlung_an_shapth(Some(now()))
        .kommentar(Default::default())
        .auftraggeber(
            AuftraggeberType::builder()
                .auftraggeber_id(context)
                .auftraggeberart("1020".into())
                .auftraggeber(Auftraggeber::Organisation(organisation_type()))
                .build(),
        )
        .zustaendige_behoerde(Default::default())
        .beauftragte_untersuchungsstelle(beauftragte_untersuchungsstelle_type(details))
        .zugelassene_untersuchungsstelle(Default::default())
        .ort_der_labortaetigkeiten(Default::default())
        .anhang(Default::default())
        .erweiterung(Default::default())
        .id(format!("pruefbericht-{}", new_id()))
        .anlage_nach_trinkw_v(Default::default())
        .gefahr_in_verzug(false)
        .objekt(Default::default())
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn pruefbericht_signature_template() -> Signature {
    Signature { exists: true }
}
