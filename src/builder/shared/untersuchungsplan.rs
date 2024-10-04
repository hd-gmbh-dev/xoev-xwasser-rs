#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{
    builder::utils::{new_id, new_uuid},
    model::shared::untersuchungsplan::*,
};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn untersuchungsplan_type() -> UntersuchungsplanType {
    UntersuchungsplanType::builder()
        .untersuchungsplan_id(new_uuid())
        .wasserversorgungsgebiet(Default::default())
        .jahr(Default::default())
        .wasserabgabe_vorjahr(Default::default())
        .art_von_wva_und_wvg(Default::default())
        .erlaeuterung_zur_wasserabgabemenge(Default::default())
        .flockung(Default::default())
        .oberflaechenwassereinfluss(Default::default())
        .desinfektion_durchgefuehrt_mit(Default::default())
        .abfuellung_zur_abgabe_in_verschlossenen_behaeltnissen(Default::default())
        .acrylamid(Default::default())
        .epichlorhydrin(Default::default())
        .vinylchlorid(Default::default())
        .ph_wert_wasserwerksausgang(Default::default())
        .wasserabgabe_vorjahr_pro_tag(Default::default())
        .anzahl_untersuchungen_pro_jahr_gruppe_a(Default::default())
        .abzudecken_durch_betreiber_gruppe_a(Default::default())
        .anzahl_untersuchungenpro_jahr_gruppe_b(Default::default())
        .abzudecken_durch_betreiber_gruppe_b(Default::default())
        .rap_durchgefuehrt(Default::default())
        .status_untersuchungsplan(Default::default())
        .kommentar(Default::default())
        .terminplan(Default::default())
        .anlage_nach_trinkw_v(Default::default())
        .auftraggeber(Default::default())
        .zustaendige_behoerde(Default::default())
        .aenderungshistorie(Default::default())
        .erweiterung(Default::default())
        .id(format!("untersuchungsplan-{}", new_id()))
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn terminplan_type() -> TerminplanType {
    TerminplanType::builder()
        .terminplan_id(new_uuid())
        .probennahmestelle(Default::default())
        .datum_zeitraum(Default::default())
        .probennahmestelle_kategorie(Default::default())
        .weitere_beschreibung_der_probennahmestelle(Default::default())
        .untersuchung_durch(Default::default())
        .untersuchung_durch_erlaeuterung(Default::default())
        .anlass_der_untersuchung(Default::default())
        .zu_untersuchender_parameter(Default::default())
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
    WasserversorgungsgebietType::builder()
        .wasserversorgungsgebiet_id(new_uuid())
        .name_wasserversorgungsgebiet(Default::default())
        .lau2_code(Default::default())
        .zustaendige_behoerde(Default::default())
        .geokoordinaten_shapth(Default::default())
        .datum_der_einrichtung(Default::default())
        .datum_der_schliessung(Default::default())
        .grund_der_schliessung(Default::default())
        .nachfolger_wvg_bei_schliessung(Default::default())
        .wvg_fremdbezogen(Default::default())
        .abgegebene_wassermenge(Default::default())
        .anzahl_versorgte_personen_wvg(Default::default())
        .referenzjahr_angaben_wvg(Default::default())
        .art_der_wasserressource(Default::default())
        .anteil_der_wasserressource(Default::default())
        .vorgeschriebene_untersuchungshaeufigkeit_parameter_a(Default::default())
        .vorgeschriebene_untersuchungshaeufigkeit_parameter_b(Default::default())
        .alt_id(Default::default())
        .kommentar(Default::default())
        .derogation(Default::default())
        .exceedance(Default::default())
        .incident(Default::default())
        .id(format!("wvg-{}", new_id()))
        .build()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn anlage_nach_trinkw_v_type() -> AnlageNachTrinkwVType {
    AnlageNachTrinkwVType::builder()
        .anlage_nach_trinkw_v_id(Default::default())
        .zustaendige_behoerde_id(Default::default())
        .untersuchungsplan_id(Default::default())
        .art_anlage(Default::default())
        .name_der_anlage(Default::default())
        .abgegebene_wassermenge_der_anlage_pro_tag(Default::default())
        .anzahl_durch_anlage_versorgte_personen(Default::default())
        .alt_id(Default::default())
        .kommentar(Default::default())
        .wasserversorgungsgebiet(Default::default())
        .anlage_nach_trinkw_v_objekt(Default::default())
        .id(format!("antv-{}", new_id()))
        .build()
}
