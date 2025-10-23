use crate::{
    builder::utils::{new_id, new_uuid, now},
    model::shared::probe::{
        AnalyseergebnisParameterType, ProbeType, ProbennahmestelleType, Probennehmer,
        ProbennehmerType,
    },
};

// use serde::{Deserialize, Serialize};

// use typed_builder::TypedBuilder;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::person::natuerliche_person_type;

// #[cfg(feature = "wasm")]
// use tsify::Tsify;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn probennehmer_type() -> ProbennehmerType {
    ProbennehmerType::builder()
        .id(format!("probennehmer-{}", new_id()))
        .probennehmer_id(Some(new_uuid()))
        .probennehmer(Probennehmer::NatuerlichePerson(natuerliche_person_type(
            "".into(),
            "".into(),
        )))
        .fremdsystem_id_probennehmer(Default::default())
        .kommentar(Default::default())
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn probennahmestelle_type(name: String) -> ProbennahmestelleType {
    ProbennahmestelleType::builder()
        .id(format!("probennahmestelle-{}", new_id()))
        .probennahmestelle_id(new_uuid())
        .objekt_id(Default::default())
        .probe(Default::default())
        .terminplan_id(Default::default())
        .name_probennahmestelle(name)
        .kategorie_probennahmestelle("1000".into())
        .unterkategorie_probennahmestelle(Some("1030".into()))
        .art_der_entnahmearmatur(Some("1010".into()))
        .stockwerk_probennahmestelle(Some(0))
        .medium_an_der_probennahmestelle(vec!["1010".into()])
        .desinfektion_und_aufbereitung_des_wassers(Default::default())
        .angaben_alternative_id(Default::default())
        .kommentar(Default::default())
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn probe_type() -> ProbeType {
    let id = new_uuid();
    ProbeType::builder()
        .id(format!("probe-{id}"))
        .probe_id(new_uuid())
        .probennahmestelle(new_uuid())
        .terminplan_id(Default::default())
        .probennehmer(Default::default())
        .titel_probe(format!("probe-titel-{id}"))
        .analyseergebnis_parameter(Default::default())
        .anlass_der_untersuchung(vec!["1010".into()])
        .medium(Some("1010".into()))
        .akkreditierte_durchfuehrung_der_probennahme(Default::default())
        .ergaenzung_zum_medium(Default::default())
        .zeitpunkt_probennahme(now())
        .probennahmeverfahren(vec!["1010".into()])
        .probenentnahmegeraet(Some("1010".into()))
        .probengefaess(Some("1010".into()))
        .ergaenzende_informationen_zu_probenentnahmegeraet(Some("".into()))
        .desinfektion_probenentnahmegeraet_durchgefuehrt(Some(true))
        .konservierung_der_probe(Default::default())
        .kommentar_zur_probennahme(Default::default())
        .informationen_zum_probentransport(Default::default())
        .eingang_probe_bei_untersuchungsstelle(now())
        .beginn_labortaetigkeit_analytik(now())
        .abschluss_labortaetigkeit_analytik(now())
        .konformitaetsbewertung_der_probe("1010".into())
        .berichtspflichtig(Default::default())
        .von_probennehmer_vergebene_probe_id(Default::default())
        .angelieferte_probe(Default::default())
        .informationen_zur_angelieferten_probe(Default::default())
        .probe_id_aus_labor(Default::default())
        .anhang(Default::default())
        .kommentar(Default::default())
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn analyseergebnis_parameter_type(
    anschrift_id: String,
    zugelassene_untersuchungsstelle: String,
) -> AnalyseergebnisParameterType {
    AnalyseergebnisParameterType::builder()
        .id(format!("parameter-{}", new_id()))
        .analyseergebnis_parameter_id(new_uuid())
        .anschrift_id(Some(anschrift_id))
        .zugelassene_untersuchungsstelle(zugelassene_untersuchungsstelle)
        .akkreditierte_durchfuehrung_analyse(Default::default())
        .zugelassene_durchfuehrung_analyse(Default::default())
        .untersuchungsverfahren(vec!["1010".into()])
        .ergaenzung_zum_untersuchungsverfahren(Default::default())
        .untersuchter_parameter("1021".into())
        .parameterauspraegung(Default::default())
        .parameter_durch_betreiber_untersucht(Default::default())
        .wurde_der_parameter_korrigiert(Default::default())
        .untersuchungswert_parameter(Some(0.0))
        .einheit_des_untersuchungswerts(Some("1400".into()))
        .ergaenzung_zum_untersuchungswert_parameter(Default::default())
        .parameterwert_ergaenzung(Default::default())
        .ausgewertetes_ansatzvolumen(Some(0.0))
        .verknuepfte_parameter(Default::default())
        .bewertung_untersuchungswert("1010".into())
        .parameterauffaelligkeit(Default::default())
        .messunsicherheit_untersuchungswert_absolut(Default::default())
        .messunsicherheit_untersuchungswert_relativ(Default::default())
        .bestimmungsgrenze_lo_q(Default::default())
        .kommentar(Default::default())
        .build()
}
