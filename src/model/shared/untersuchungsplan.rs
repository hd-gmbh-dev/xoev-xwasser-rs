
// /// Klasse für den Transport von Informationen, die für die Erstellung eines
// /// Untersuchungsplans für a- und b-Anlagen relevant sind.
// #[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
// #[cfg_attr(feature = "wasm", derive(Tsify))]
// #[xml(tns(
//     b"xwas",
//     b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
// ))]
// pub struct UntersuchungsplanType {
//     #[xml(ns = b"xwas", name = b"untersuchungsplanID", ty = "child")]
//     pub untersuchungsplan_id: String,
//     #[xml(ns = b"xwas", name = b"wasserversorgungsgebiet", ty = "child")]
//     #[serde(default)]
//     pub wasserversorgungsgebiet: Vec<String>,
//     #[xml(ns = b"xwas", name = b"jahr", ty = "child")]
//     #[serde(default)]
//     pub jahr: Vec<String>, // JahrType
//     #[xml(ns = b"xwas", name = b"wasserabgabeVorjahr", ty = "child")]
//     pub wasserabgabe_vorjahr: f64,
//     #[xml(ns = b"xwas", name = b"artVonWVAundWVG", ty = "child")]
//     pub art_von_wva_und_wvg: CodeWVAType,
//     #[xml(ns = b"xwas", name = b"erlaeuterungZurWasserabgabemenge", ty = "child")]
//     pub erlaeuterung_zur_wasserabgabemenge: CodeErlaeuterungWasserabgabemengeType,
//     #[xml(ns = b"xwas", name = b"flockung", ty = "child")]
//     pub flockung: CodeFlockungType,
//     #[xml(ns = b"xwas", name = b"oberflaechenwassereinfluss", ty = "child")]
//     pub oberflaechenwassereinfluss: bool,
//     #[xml(ns = b"xwas", name = b"desinfektionDurchgefuehrtMit", ty = "child")]
//     pub desinfektion_durchgefuehrt_mit: CodeDesinfektionsartType,
//     #[xml(
//         ns = b"xwas",
//         name = b"abfuellungZurAbgabeInVerschlossenenBehaeltnissen",
//         ty = "child"
//     )]
//     pub abfuellung_zur_abgabe_in_verschlossenen_behaeltnissen: bool,
//     #[xml(ns = b"xwas", name = b"acrylamid", ty = "child")]
//     pub acrylamid: CodeNachweisartType,
//     #[xml(ns = b"xwas", name = b"epichlorhydrin", ty = "child")]
//     pub epichlorhydrin: CodeNachweisartType,
//     #[xml(ns = b"xwas", name = b"vinylchlorid", ty = "child")]
//     pub vinylchlorid: CodeNachweisartType,
//     #[xml(ns = b"xwas", name = b"phWertWasserwerksausgang", ty = "child")]
//     pub ph_wert_wasserwerksausgang: bool,
//     #[xml(ns = b"xwas", name = b"wasserabgabeVorjahrProTag", ty = "child")]
//     pub wasserabgabe_vorjahr_pro_tag: f64,
//     #[xml(
//         ns = b"xwas",
//         name = b"anzahlUntersuchungenproJahrGruppeA",
//         ty = "child"
//     )]
//     pub anzahl_untersuchungenpro_jahr_gruppe_a: u32,
//     #[xml(ns = b"xwas", name = b"abzudeckenDurchBetreiberGruppeA", ty = "child")]
//     pub abzudecken_durch_betreiber_gruppe_a: Option<u32>,
//     #[xml(
//         ns = b"xwas",
//         name = b"anzahlUntersuchungenproJahrGruppeB",
//         ty = "child"
//     )]
//     pub anzahl_untersuchungenpro_jahr_gruppe_b: u32,
//     #[xml(ns = b"xwas", name = b"abzudeckenDurchBetreiberGruppeB", ty = "child")]
//     pub abzudecken_durch_betreiber_gruppe_b: Option<u32>,
//     #[xml(ns = b"xwas", name = b"rapDurchgefuehrt", ty = "child")]
//     pub rap_durchgefuehrt: bool,
//     #[xml(ns = b"xwas", name = b"statusUntersuchungsplan", ty = "child")]
//     pub status_untersuchungsplan: CodeStatusUntersuchungsplanType,
//     #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
//     pub kommentar: Option<String>,
//     #[xml(ns = b"xwas", name = b"terminplan", ty = "child")]
//     #[serde(default)]
//     pub terminplan: Vec<TerminplanType>,
//     #[xml(ns = b"xwas", name = b"anlageNachTrinkwV", ty = "child")]
//     pub anlage_nach_trinkw_v: AnlageNachTrinkwVType,
//     #[xml(ns = b"xwas", name = b"auftraggeber", ty = "child")]
//     pub auftraggeber: Auftraggeber,
//     #[xml(ns = b"xwas", name = b"zustaendigeBehoerde", ty = "child")]
//     pub zustaendige_behoerde: ZustaendigeBehoerdeType,
//     #[xml(ns = b"xwas", name = b"erweiterung", ty = "child")]
//     pub erweiterung: Option<ErweiterungType>,
//     #[xml(ns = b"xwas", name = b"id", ty = "attr")]
//     pub id: String,
// }



// /// Klasse für den Transport von Informationen, die für die Erstellung von Terminplänen
// /// als Teil des Untersuchungsplans für a- und b-Anlagen relevant sind.
// #[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
// #[cfg_attr(feature = "wasm", derive(Tsify))]
// #[xml(tns(
//     b"xwas",
//     b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
// ))]
// pub struct TerminplanType {
//     #[xml(ns = b"xwas", name = b"terminplanID", ty = "child")]
//     pub terminplan_id: String,
//     #[xml(ns = b"xwas", name = b"probennahmestelleID", ty = "child")]
//     pub probennahmestelle_id: Option<String>,
//     #[xml(ns = b"xwas", name = b"datumZeitraum", ty = "child")]
//     #[serde(default)]
//     pub datum_zeitraum: Vec<String>,
//     #[xml(ns = b"xwas", name = b"probennahmestelleKategorie", ty = "child")]
//     pub probennahmestelle_kategorie: CodeKategorieProbennahmestelleType,
//     #[xml(
//         ns = b"xwas",
//         name = b"weitereBeschreibungDerProbennahmestelle",
//         ty = "child"
//     )]
//     pub weitere_beschreibung_der_probennahmestelle: Option<String>,
//     #[xml(ns = b"xwas", name = b"untersuchungDurch", ty = "child")]
//     #[serde(default)]
//     pub untersuchung_durch: Vec<CodeUeberwachungAufbereitungType>,
//     #[xml(ns = b"xwas", name = b"untersuchungDurchErlaeuterung", ty = "child")]
//     pub untersuchung_durch_erlaeuterung: Option<String>,
//     #[xml(ns = b"xwas", name = b"anlassDerUntersuchung", ty = "child")]
//     pub anlass_der_untersuchung: Vec<CodeAnlassUntersuchungType>,
//     #[xml(ns = b"xwas", name = b"zuUntersuchendeParameter", ty = "child")]
//     #[serde(default)]
//     pub zu_untersuchende_parameter: Vec<CodeShapthParameterType>,
//     #[xml(ns = b"xwas", name = b"probennahmeverfahren", ty = "child")]
//     #[serde(default)]
//     pub probennahmeverfahren: Vec<CodeProbennahmeverfahrenType>,
//     #[xml(ns = b"xwas", name = b"ersatzFuerTerminplanMitDerID", ty = "child")]
//     pub ersatz_fuer_terminplan_mit_der_id: Option<String>,
//     #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
//     pub kommentar: Option<String>,
//     #[xml(ns = b"xwas", name = b"id", ty = "attr")]
//     pub _id: String,
// }


// /// Klasse für den Transport von Informationen, die für Vorfälle gem. den neuen Vorgaben
// /// für das EU-Berichtsformat benötigt werden.
// #[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
// #[cfg_attr(feature = "wasm", derive(Tsify))]
// #[xml(tns(
//     b"xwas",
//     b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
// ))]
// pub struct IncidentType {
//     #[xml(ns = b"xwas", name = b"incidentIdentifier", ty = "child")]
//     pub incident_identifier: String, // bn-uq-g2g:UUID
//     #[xml(ns = b"xwas", name = b"exceedance", ty = "child")]
//     #[serde(default)]
//     pub exceedance: Vec<String>, // xs:IDREF
//     #[xml(ns = b"xwas", name = b"incidentStartDate", ty = "child")]
//     pub incident_start_date: String, // xs:date
//     #[xml(ns = b"xwas", name = b"incidentEndDate", ty = "child")]
//     pub incident_end_date: String, // xs:date
//     #[xml(ns = b"xwas", name = b"incidentCategory", ty = "child")]
//     #[serde(default)]
//     pub incident_category: Vec<CodeIncidentCategoryType>,
//     #[xml(ns = b"xwas", name = b"incidentAffectedPopulation", ty = "child")]
//     pub incident_affected_population: u32,
//     #[xml(ns = b"xwas", name = b"remarks", ty = "child")]
//     pub remarks: Option<String>,
//     #[xml(ns = b"xwas", name = b"incidentCauseAndRemedialAction", ty = "child")]
//     #[serde(default)]
//     pub incident_cause_and_remedial_action: Vec<IncidentCauseAndRemedialActionType>,
//     #[xml(ns = b"xwas", name = b"_id", ty = "attr")]
//     #[serde(skip)]
//     pub _id: ConstStr,
// }


// /// Transport der Informationen, die für Qualität und Überwachung gem. dem neuen Vorgaben
// /// für das EU-Berichtsformat benötigt werden.
// #[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
// #[cfg_attr(feature = "wasm", derive(Tsify))]
// #[xml(tns(
//     b"xwas",
//     b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
// ))]
// pub struct QualityAndMonitoringType {
//     #[xml(
//         ns = b"xwas",
//         name = b"qualityAndMonitoringRequirementIdentifier",
//         ty = "child"
//     )]
//     pub quality_and_monitoring_requirement_identifier: String, // bn-uq-g2g:UUID
//     #[xml(ns = b"xwas", name = b"parameterCode", ty = "child")]
//     pub parameter_code: CodeShapthParameterType,
//     #[xml(ns = b"xwas", name = b"parameterThresholdValue", ty = "child")]
//     pub parameter_threshold_value: f64,
//     #[xml(ns = b"xwas", name = b"parameterThresholdValueUnit", ty = "child")]
//     pub parameter_threshold_value_unit: CodeShapthParameterEinheitType,
//     #[xml(ns = b"xwas", name = b"samplingFrequency", ty = "child")]
//     pub sampling_frequency: u32,
//     #[xml(ns = b"xwas", name = b"samplingPeriod", ty = "child")]
//     pub sampling_period: CodeProbennahmezeitraumType,
//     #[xml(ns = b"xwas", name = b"samplingLocationType", ty = "child")]
//     #[serde(default)]
//     pub sampling_location_type: Vec<CodeArtProbennahmestelleEUType>,
//     #[xml(ns = b"xwas", name = b"remarks", ty = "child")]
//     pub remarks: Option<String>,
//     #[xml(ns = b"xwas", name = b"_id", ty = "attr")]
//     #[serde(skip)]
//     pub _id: ConstStr,
// }

// /// Transport solcher Informationen, die für die Ausnahmeregelungen gem. den neuen
// /// Vorgaben für das EU-Berichtsformat benötigt werden.
// #[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
// #[cfg_attr(feature = "wasm", derive(Tsify))]
// #[xml(tns(
//     b"xwas",
//     b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
// ))]
// pub struct DerogationType {
//     #[xml(ns = b"xwas", name = b"derogationIdentifier", ty = "child")]
//     pub derogation_identifier: String, // bn-uq-g2g:UUID
//     #[xml(ns = b"xwas", name = b"trivialDerogation", ty = "child")]
//     pub trivial_derogation: bool,
//     #[xml(ns = b"xwas", name = b"trivialDerogationJustification", ty = "child")]
//     pub trivial_derogation_justification: Option<String>,
//     #[xml(ns = b"xwas", name = b"derogationStartDate", ty = "child")]
//     pub derogation_start_date: Option<String>, // xs:date
//     #[xml(ns = b"xwas", name = b"derogationEndDate", ty = "child")]
//     pub derogation_end_date: Option<String>, // xs:date
//     #[xml(ns = b"xwas", name = b"volumeOfWaterSupplied", ty = "child")]
//     pub volume_of_water_supplied: Option<f64>,
//     #[xml(ns = b"xwas", name = b"derogationAffectedPopulation", ty = "child")]
//     pub derogation_affected_population: Option<u32>,
//     #[xml(ns = b"xwas", name = b"foodProductionAffected", ty = "child")]
//     pub food_production_affected: Option<bool>,
//     #[xml(ns = b"xwas", name = b"derogationUnderRecastDWD", ty = "child")]
//     pub derogation_under_recast_dwd: bool,
//     #[xml(ns = b"xwas", name = b"derogationGrounds", ty = "child")]
//     pub derogation_grounds: Option<CodeGrundAusnahmeregelungType>,
//     #[xml(ns = b"xwas", name = b"previousDerogationIdentifier", ty = "child")]
//     pub previous_derogation_identifier: Option<String>, // xs:IDREF
//     #[xml(ns = b"xwas", name = b"previousDerogationConclusions", ty = "child")]
//     pub previous_derogation_conclusions: Option<String>,
//     #[xml(ns = b"xwas", name = b"previousDerogationStartDate", ty = "child")]
//     pub previous_derogation_start_date: Option<String>, // xs:date
//     #[xml(ns = b"xwas", name = b"previousDerogationEndDate", ty = "child")]
//     pub previous_derogation_end_date: Option<String>, // xs:date
//     #[xml(ns = b"xwas", name = b"previousDerogationGrounds", ty = "child")]
//     pub previous_derogation_grounds: Option<CodeGrundAusnahmeregelungType>,
//     #[xml(ns = b"xwas", name = b"remarks", ty = "child")]
//     pub remarks: Option<String>,
//     #[xml(ns = b"xwas", name = b"derogationRemedialAction", ty = "child")]
//     #[serde(default)]
//     pub derogation_remedial_action: Vec<DerogationRemedialActionType>,
//     #[xml(ns = b"xwas", name = b"qualityAndMonitoring", ty = "child")]
//     #[serde(default)]
//     pub quality_and_monitoring: Vec<QualityAndMonitoringType>,
//     #[xml(ns = b"xwas", name = b"_id", ty = "attr")]
//     #[serde(skip)]
//     pub _id: ConstStr,
// }

// /// Klasse für den Transport von Informationen, die für Ursachen und Maßnahmen von
// /// Überschreitungen gem. den neuen Vorgaben für das EU-Berichtsformat benötigt werden.
// #[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
// #[cfg_attr(feature = "wasm", derive(Tsify))]
// #[xml(tns(
//     b"xwas",
//     b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
// ))]
// pub struct ExceedanceCauseAndRemedialActionType {
//     #[xml(
//         ns = b"xwas",
//         name = b"exceedanceCauseAndRemedialActionIdentifier",
//         ty = "child"
//     )]
//     pub exceedance_cause_and_remedial_action_identifier: String, // bn-uq-g2g:UUID
//     #[xml(ns = b"xwas", name = b"exceedanceCause", ty = "child")]
//     pub exceedance_cause: CodeIncidentExceedanceCauseType,
//     #[xml(ns = b"xwas", name = b"exceedanceRemedialAction", ty = "child")]
//     pub exceedance_remedial_action: CodeMassnahmeType,
//     #[xml(
//         ns = b"xwas",
//         name = b"exceedanceRemedialActionStartDate",
//         ty = "child"
//     )]
//     pub exceedance_remedial_action_start_date: String, // xs:date
//     #[xml(ns = b"xwas", name = b"exceedanceRemedialActionEndDate", ty = "child")]
//     pub exceedance_remedial_action_end_date: String, // xs:date
//     #[xml(ns = b"xwas", name = b"remarks", ty = "child")]
//     pub remarks: Option<String>,
//     #[xml(ns = b"xwas", name = b"_id", ty = "attr")]
//     #[serde(skip)]
//     pub _id: ConstStr,
// }

// /// Klasse für den Transport von Informationen, die für Überschreitungen gem. den neuen
// /// Vorgaben für das EU-Berichtsformat benötigt werden.
// #[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
// #[cfg_attr(feature = "wasm", derive(Tsify))]
// #[xml(tns(
//     b"xwas",
//     b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
// ))]
// pub struct ExceedanceType {
//     #[xml(ns = b"xwas", name = b"exceedanceIdentifier", ty = "child")]
//     pub exceedance_identifier: String, // bn-uq-g2g:UUID
//     #[xml(ns = b"xwas", name = b"trivialExceedance", ty = "child")]
//     pub trivial_exceedance: bool,
//     #[xml(ns = b"xwas", name = b"trivialExceedanceJustification", ty = "child")]
//     pub trivial_exceedance_justification: Option<String>,
//     #[xml(ns = b"xwas", name = b"parameterCode", ty = "child")]
//     pub parameter_code: CodeShapthParameterType,
//     #[xml(ns = b"xwas", name = b"exceedanceStartDate", ty = "child")]
//     pub exceedance_start_date: String, // xs:date
//     #[xml(ns = b"xwas", name = b"exceedanceEndDate", ty = "child")]
//     pub exceedance_end_date: String, // xs:date
//     #[xml(ns = b"xwas", name = b"exceedanceAffectedPopulation", ty = "child")]
//     pub exceedance_affected_population: u32,
//     #[xml(ns = b"xwas", name = b"pointOfComplianceType", ty = "child")]
//     #[serde(default)]
//     pub point_of_compliance_type: Vec<CodeArtProbennahmestelleEUType>,
//     #[xml(ns = b"xwas", name = b"numberOfSamplesPerYear", ty = "child")]
//     pub number_of_samples_per_year: u32,
//     #[xml(ns = b"xwas", name = b"incidentIdentifier", ty = "child")]
//     pub incident_identifier: Option<String>, // xs:IDREF
//     #[xml(ns = b"xwas", name = b"derogationIdentifier", ty = "child")]
//     pub derogation_identifier: Option<String>, //xs:IDREF
//     #[xml(ns = b"xwas", name = b"remarks", ty = "child")]
//     pub remarks: Option<String>,
//     #[xml(ns = b"xwas", name = b"exceedanceCauseAndRemedialAction", ty = "child")]
//     #[serde(default)]
//     pub exceedance_cause_and_remedial_action: Vec<ExceedanceCauseAndRemedialActionType>,
//     #[xml(ns = b"xwas", name = b"_id", ty = "attr")]
//     #[serde(skip)]
//     pub _id: ConstStr,
// }

// /// Klasse für den Transport von Informationen, die zur Beschreibung von Ursache und
// /// Maßnahmen von Vorfällen gem. den neuen Vorgaben für das EU-Berichtsformat benötigt
// /// werden.
// #[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
// #[cfg_attr(feature = "wasm", derive(Tsify))]
// #[xml(tns(
//     b"xwas",
//     b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
// ))]
// pub struct IncidentCauseAndRemedialActionType {
//     #[xml(ns = b"xwas", name = b"incidentCauseAndActionIdentifier", ty = "child")]
//     pub incident_cause_and_action_identifier: String, // bn-uq-g2g:UUID
//     #[xml(ns = b"xwas", name = b"incidentCause", ty = "child")]
//     pub incident_cause: CodeIncidentExceedanceCauseType,
//     #[xml(ns = b"xwas", name = b"incidentRemedialAction", ty = "child")]
//     pub incident_remedial_action: CodeMassnahmeType,
//     #[xml(ns = b"xwas", name = b"incidentRemedialActionStartDate", ty = "child")]
//     pub incident_remedial_action_start_date: String, // xs:date
//     #[xml(ns = b"xwas", name = b"incidentRemedialActionEndDate", ty = "child")]
//     pub incident_remedial_action_end_date: String, // xs:date
//     #[xml(ns = b"xwas", name = b"remarks", ty = "child")]
//     pub remarks: Option<u32>,
//     #[xml(ns = b"xwas", name = b"_id", ty = "attr")]
//     #[serde(skip)]
//     pub _id: ConstStr,
// }


// /// Klasse für den Transport von Informationen zu einem Wasserversorgungsgebiet [Soweit
// /// möglich in Register zu pflegen].
// #[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
// #[cfg_attr(feature = "wasm", derive(Tsify))]
// #[xml(tns(
//     b"xwas",
//     b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
// ))]
// pub struct WasserversorgungsgebietType {
//     #[xml(ns = b"xwas", name = b"wasserversorgungsgebietID", ty = "child")]
//     pub wasserversorgungsgebiet_id: String,
//     #[xml(ns = b"xwas", name = b"nameWasserversorgungsgebiet", ty = "child")]
//     pub name_wasserversorgungsgebiet: CodeWasserversorgungsgebietType,
//     #[xml(ns = b"xwas", name = b"lau2Code", ty = "child")]
//     pub lau2_code: Option<String>,
//     #[xml(ns = b"xwas", name = b"zustaendigeBehoerde", ty = "child")]
//     #[serde(default)]
//     pub zustaendige_behoerde: Vec<ZustaendigeBehoerdeType>,
//     #[xml(ns = b"xwas", name = b"geokoordinatenSHAPTH", ty = "child")]
//     pub geokoordinaten_shapth: Option<GeokoordinatenShapthType>,
//     #[xml(ns = b"xwas", name = b"datumDerEinrichtung", ty = "child")]
//     pub datum_der_einrichtung: String, // xs:date
//     #[xml(ns = b"xwas", name = b"datumDerSchliessung", ty = "child")]
//     pub datum_der_schliessung: Option<String>, //xs:date
//     #[xml(ns = b"xwas", name = b"grundDerSchliessung", ty = "child")]
//     pub grund_der_schliessung: Option<CodeGrundSchliessungWasserversorgungsgebietType>,
//     #[xml(ns = b"xwas", name = b"nachfolgerWVGbeiSchliessung", ty = "child")]
//     #[serde(default)]
//     pub nachfolger_wvg_bei_schliessung: Vec<String>, // xs:IDREF
//     #[xml(ns = b"xwas", name = b"wvgFremdbezogen", ty = "child")]
//     #[serde(default)]
//     pub wvg_fremdbezogen: Vec<String>, // xs:IDREF
//     #[xml(ns = b"xwas", name = b"abgegebeneWassermenge", ty = "child")]
//     pub abgegebene_wassermenge: f64,
//     #[xml(ns = b"xwas", name = b"anzahlVersorgtePersonenWVG", ty = "child")]
//     pub anzahl_versorgte_personen_wvg: u32,
//     #[xml(ns = b"xwas", name = b"referenzjahrAngabenWVG", ty = "child")]
//     pub referenzjahr_angaben_wvg: u32,
//     #[xml(ns = b"xwas", name = b"artDerWasserressource", ty = "child")]
//     #[serde(default)]
//     pub art_der_wasserressource: Vec<CodeArtWasserressourceType>,
//     #[xml(ns = b"xwas", name = b"anteilDerWasserressource", ty = "child")]
//     #[serde(default)]
//     pub anteil_der_wasserressource: Vec<u32>,
//     #[xml(
//         ns = b"xwas",
//         name = b"vorgeschriebeneUntersuchungshaeufigkeitParameterA",
//         ty = "child"
//     )]
//     pub vorgeschriebene_untersuchungshaeufigkeit_parameter_a: u32,
//     #[xml(
//         ns = b"xwas",
//         name = b"vorgeschriebeneUntersuchungshaeufigkeitParameterB",
//         ty = "child"
//     )]
//     pub vorgeschriebene_untersuchungshaeufigkeit_parameter_b: u32,
//     #[xml(ns = b"xwas", name = b"altID", ty = "child")]
//     pub alt_id: Option<String>,
//     #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
//     pub kommentar: Option<String>,
//     #[xml(ns = b"xwas", name = b"derogation", ty = "child")]
//     #[serde(default)]
//     pub derogation: Vec<DerogationType>,
//     #[xml(ns = b"xwas", name = b"exceedance", ty = "child")]
//     #[serde(default)]
//     pub exceedance: Vec<ExceedanceType>,
//     #[xml(ns = b"xwas", name = b"incident", ty = "child")]
//     #[serde(default)]
//     pub incident: Vec<IncidentType>,
//     #[xml(ns = b"xwas", name = b"_id", ty = "attr")]
//     #[serde(skip)]
//     pub _id: ConstStr,
// }

// /// Klasse für den Transport von Informationen zu einer Trinkwasserversorgungsanlage.
// #[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
// #[cfg_attr(feature = "wasm", derive(Tsify))]
// #[xml(tns(
//     b"xwas",
//     b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
// ))]
// pub struct AnlageNachTrinkwVType {
//     #[xml(ns = b"xwas", name = b"anlageNachTrinkwVID", ty = "child")]
//     pub anlage_nach_trinkw_vid: String,
//     #[xml(ns = b"xwas", name = b"zustaendigeBehoerdeID", ty = "child")]
//     pub zustaendige_behoerde_id: String,
//     #[xml(ns = b"xwas", name = b"untersuchungsplanID", ty = "child")]
//     #[serde(default)]
//     pub untersuchungsplan_id: Vec<String>,
//     #[xml(ns = b"xwas", name = b"artAnlage", ty = "child")]
//     pub art_anlage: CodeArtTrinkwasseranlageType,
//     #[xml(ns = b"xwas", name = b"nameDerAnlage", ty = "child")]
//     pub name_der_anlage: String,
//     #[xml(
//         ns = b"xwas",
//         name = b"abgegebeneWassermengeDerAnlageProTag",
//         ty = "child"
//     )]
//     pub abgegebene_wassermenge_der_anlage_pro_tag: Option<f64>,
//     #[xml(
//         ns = b"xwas",
//         name = b"anzahlDurchAnlageVersorgtePersonen",
//         ty = "child"
//     )]
//     pub anzahl_durch_anlage_versorgte_personen: Option<u32>,
//     #[xml(ns = b"xwas", name = b"altID", ty = "child")]
//     pub alt_id: Option<String>,
//     #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
//     pub kommentar: Option<String>,
//     #[xml(ns = b"xwas", name = b"wasserversorgungsgebiet", ty = "child")]
//     #[serde(default)]
//     pub wasserversorgungsgebiet: Vec<WasserversorgungsgebietType>,
//     #[xml(ns = b"xwas", name = b"anlageNachTrinwV_Objekt", ty = "child")]
//     #[serde(default)]
//     pub anlage_nach_trinw_v_objekt: Vec<ObjektType>,
//     #[xml(ns = b"xwas", name = b"id", ty = "attr")]
//     pub id: String,
// }
