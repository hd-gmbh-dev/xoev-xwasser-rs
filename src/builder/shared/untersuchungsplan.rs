#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{
    builder::utils::{new_id, new_uuid, now, today},
    model::shared::{auftraggeber::AuftraggeberType, untersuchungsplan::*},
};

use super::{
    behoerde::{behoerde_type, zustaendige_behoerde_type},
    betreiber::objekt_type,
    organisation::organisation_type,
    probe::probennahmestelle_type,
};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn untersuchungsplan_type() -> UntersuchungsplanType {
    let zustaendige_behoerde = zustaendige_behoerde_type("DEBY".to_string());
    let mut result = UntersuchungsplanType::builder()
        .untersuchungsplan_id(new_uuid())
        .wasserversorgungsgebiet(Default::default())
        .titel_untersuchungsplan(Default::default())
        .jahr(vec![2024, 2025])
        .wasserabgabe_vorjahr(Default::default())
        .art_von_wva_und_wvg("1010".into())
        .erlaeuterung_zur_wasserabgabemenge("1010".into())
        .flockung("1010".into())
        .oberflaechenwassereinfluss(Default::default())
        .desinfektion_durchgefuehrt_mit("1010".into())
        .abfuellung_zur_abgabe_in_verschlossenen_behaeltnissen(Default::default())
        .acrylamid("1010".into())
        .epichlorhydrin("1010".into())
        .vinylchlorid("1010".into())
        .ph_wert_wasserwerksausgang(Default::default())
        .wasserabgabe_vorjahr_pro_tag(Default::default())
        .anzahl_untersuchungen_pro_jahr_gruppe_a(Default::default())
        .abzudecken_durch_betreiber_gruppe_a(Default::default())
        .anzahl_untersuchungenpro_jahr_gruppe_b(Default::default())
        .abzudecken_durch_betreiber_gruppe_b(Default::default())
        .rap_durchgefuehrt(Default::default())
        .status_untersuchungsplan("1010".into())
        .kommentar(Default::default())
        .terminplan(vec![terminplan_type()])
        .anlage_nach_trinkw_v(anlage_nach_trinkw_v_type(
            zustaendige_behoerde.id.as_ref().cloned(),
        ))
        .auftraggeber(
            AuftraggeberType::builder()
                .auftraggeber_id(Default::default())
                .auftraggeberart("1010".into())
                .auftraggeber(
                    crate::model::shared::auftraggeber::Auftraggeber::Organisation(
                        organisation_type(),
                    ),
                )
                .build(),
        )
        .zustaendige_behoerde(zustaendige_behoerde)
        .beauftragte_untersuchungsstelle_id(Default::default())
        .objekt(vec![objekt_type()])
        .probennahmestelle(vec![probennahmestelle_type(
            "Probennahmestelle 1".to_string(),
        )])
        .aenderungshistorie(Default::default())
        .erweiterung(Default::default())
        .id(format!("untersuchungsplan-{}", new_id()))
        .build();
    let wvg = result
        .anlage_nach_trinkw_v
        .wasserversorgungsgebiet
        .first()
        .unwrap()
        .id
        .clone();
    result.wasserversorgungsgebiet = vec![wvg];
    result
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn terminplan_type() -> TerminplanType {
    TerminplanType::builder()
        .terminplan_id(new_uuid())
        .probennahmestelle(Default::default())
        .datum_zeitraum(vec![now()])
        .probennahmestelle_kategorie("L".into())
        .weitere_beschreibung_der_probennahmestelle(Default::default())
        .untersuchung_durch(vec!["1010".into()])
        .untersuchung_durch_erlaeuterung(Default::default())
        .anlass_der_untersuchung(Default::default())
        .zu_untersuchender_parameter(vec![parameterangaben_type()])
        .probennahmeverfahren(Default::default())
        .ersatz_fuer_terminplan_mit_der_id(Default::default())
        .kommentar(Default::default())
        .id(format!("terminplan-{}", new_id()))
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn kommentar_type() -> KommentarType {
    KommentarType::builder()
        .autor(Default::default())
        .erstellungszeitpunkt(Default::default())
        .inhalt(Default::default())
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn aenderungshistorie_type() -> AenderungshistorieType {
    AenderungshistorieType::builder()
        .eintrag(Default::default())
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn incident_type() -> IncidentType {
    IncidentType::builder()
        .incident_identifier(Default::default())
        .exceedance(Default::default())
        .incident_start_date(Default::default())
        .incident_end_date(Default::default())
        .incident_category(Default::default())
        .incident_affected_population(Default::default())
        .remarks(Default::default())
        .incident_cause_and_remedial_action(Default::default())
        .id(format!("incident-{}", new_id()))
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn parameterangaben_type() -> ParameterangabenType {
    ParameterangabenType::builder()
        .parameter(Default::default())
        .messprogramm(Default::default())
        .parameter_gruppe(Default::default())
        .veraenderlich(Default::default())
        .probennahmeverfahren_uba(Default::default())
        .reduzierbar_durch_rap_rau(Default::default())
        .reduzierbar_ohne_rau(Default::default())
        .bemerkung(Default::default())
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn quality_and_monitoring_type() -> QualityAndMonitoringType {
    QualityAndMonitoringType::builder()
        .quality_and_monitoring_requirement_identifier(Default::default())
        .parameter_code(Default::default())
        .parameter_threshold_value(Default::default())
        .parameter_threshold_value_unit(Default::default())
        .sampling_frequency(Default::default())
        .sampling_period(Default::default())
        .sampling_location_type(Default::default())
        .remarks(Default::default())
        .id(format!("qandm-{}", new_id()))
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn derogation_remedial_action_type() -> DerogationRemedialActionType {
    DerogationRemedialActionType::builder()
        .derogation_remedial_action_identifier(Default::default())
        .derogation_remedial_action(Default::default())
        .derogation_remedial_action_start_date(Default::default())
        .derogation_remedial_action_end_date(Default::default())
        .derogation_remedial_action_cost(Default::default())
        .remarks(Default::default())
        .id(format!("dra-{}", new_id()))
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn derogation_type() -> DerogationType {
    DerogationType::builder()
        .derogation_identifier(Default::default())
        .trivial_derogation(Default::default())
        .trivial_derogation_justification(Default::default())
        .derogation_start_date(Default::default())
        .derogation_end_date(Default::default())
        .volume_of_water_supplied(Default::default())
        .derogation_affected_population(Default::default())
        .food_production_affected(Default::default())
        .derogation_under_recast_dwd(Default::default())
        .derogation_grounds(Default::default())
        .previous_derogation_identifier(Default::default())
        .previous_derogation_conclusions(Default::default())
        .previous_derogation_start_date(Default::default())
        .previous_derogation_end_date(Default::default())
        .previous_derogation_grounds(Default::default())
        .remarks(Default::default())
        .derogation_remedial_action(Default::default())
        .quality_and_monitoring(Default::default())
        .id(format!("derogation-{}", new_id()))
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn exceedance_cause_and_remedial_action_type() -> ExceedanceCauseAndRemedialActionType {
    ExceedanceCauseAndRemedialActionType::builder()
        .exceedance_cause_and_remedial_action_identifier(Default::default())
        .exceedance_cause(Default::default())
        .exceedance_remedial_action(Default::default())
        .exceedance_remedial_action_start_date(Default::default())
        .exceedance_remedial_action_end_date(Default::default())
        .remarks(Default::default())
        .id(format!("ecara-{}", new_id()))
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn exceedance_type() -> ExceedanceType {
    ExceedanceType::builder()
        .exceedance_identifier(Default::default())
        .trivial_exceedance(Default::default())
        .trivial_exceedance_justification(Default::default())
        .parameter_code(Default::default())
        .exceedance_start_date(Default::default())
        .exceedance_end_date(Default::default())
        .exceedance_affected_population(Default::default())
        .point_of_compliance_type(Default::default())
        .number_of_samples_per_year(Default::default())
        .incident_identifier(Default::default())
        .derogation_identifier(Default::default())
        .remarks(Default::default())
        .exceedance_cause_and_remedial_action(Default::default())
        .id(format!("exceedance-{}", new_id()))
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn incident_cause_and_remedial_action_type() -> IncidentCauseAndRemedialActionType {
    IncidentCauseAndRemedialActionType::builder()
        .incident_cause_and_remidial_action_identifier(Default::default())
        .incident_cause(Default::default())
        .incident_remedial_action(Default::default())
        .incident_remedial_action_start_date(Default::default())
        .incident_remedial_action_end_date(Default::default())
        .remarks(Default::default())
        .id(format!("icara-{}", new_id()))
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn wasserversorgungsgebiet_type() -> WasserversorgungsgebietType {
    let zustaendige_behoerde = behoerde_type();
    WasserversorgungsgebietType::builder()
        .wasserversorgungsgebiet_id(new_uuid())
        .name_wasserversorgungsgebiet(Default::default())
        .lau2_code(Default::default())
        .zustaendige_behoerde(vec![zustaendige_behoerde])
        .geokoordinaten_shapth(Default::default())
        .datum_der_einrichtung(today())
        .datum_der_schliessung(Default::default())
        .grund_der_schliessung(Default::default())
        .nachfolger_wvg_bei_schliessung(Default::default())
        .wvg_fremdbezogen(Default::default())
        .abgegebene_wassermenge(Default::default())
        .anzahl_versorgte_personen_wvg(Default::default())
        .referenzjahr_angaben_wvg(Default::default())
        .art_der_wasserressource(vec!["1010".into()])
        .anteil_der_wasserressource(vec![0])
        .vorgeschriebene_untersuchungshaeufigkeit_parameter_a(Default::default())
        .vorgeschriebene_untersuchungshaeufigkeit_parameter_b(Default::default())
        .kommentar(Default::default())
        .derogation(Default::default())
        .exceedance(Default::default())
        .incident(Default::default())
        .id(format!("wvg-{}", new_id()))
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn anlage_nach_trinkw_v_type(zustaendige_behoerde_id: Option<String>) -> AnlageNachTrinkwVType {
    AnlageNachTrinkwVType::builder()
        .anlage_nach_trinkw_v_id(Default::default())
        .zustaendige_behoerde_id(zustaendige_behoerde_id.clone())
        .untersuchungsplan_id(Default::default())
        .art_anlage("1010".into())
        .name_der_anlage(Default::default())
        .abgegebene_wassermenge_der_anlage_pro_tag(Default::default())
        .anzahl_durch_anlage_versorgte_personen(Default::default())
        .kommentar(Default::default())
        .wasserversorgungsgebiet(vec![wasserversorgungsgebiet_type()])
        .angaben_alternative_id(Default::default())
        .id(format!("antv-{}", new_id()))
        .build()
}
