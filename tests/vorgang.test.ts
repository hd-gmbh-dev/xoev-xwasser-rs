import { describe, it, expect } from "vitest";
import {
  JuristischePersonType,
  NachrichtenTyp,
  GeburtType,
  AlternativeRepraesentationType,
  AllgemeinerNameType,
  VerwaltungspolitischeKodierungType,
  AnschriftType,
  IdentifikationType,
  KommunikationType,
  SpracheType,
  VertreterBevollmaechtigterType,
  BehoerdeType,
  RegistrierungType,
  AnalyseergebnisParameter,
  Probennehmer,
  ProbennahmestelleType,
  ZugelasseneUntersuchungsstelleType,
  BeauftragteUntersuchungsstelleType,
  AusweisdokumentType,
  StaatType,
  StaatsangehoerigkeitType,
  FamilienstandType,
  CodeDokumenttypType,
  SignaturenType,
  DokumentRepraesentationType,
  DokumentType,
  BetreiberType,
  ObjektType,
  IncidentCauseAndRemedialActionType,
  IncidentType,
  ExceedanceCauseAndRemedialActionType,
  ExceedanceType,
  QualityAndMonitoringType,
  DerogationRemedialActionType,
  DerogationType,
  GeokoordinatenShapthType,
  WasserversorgungsgebietType,
  AnlageNachTrinkwVType,
  TerminplanType,
  UntersuchungsplanType,
  VorgangType,
  NameOrganisationType,
  OrganisationType,
  PruefberichtType,
  NameNatuerlichePersonType,
  ZeitraumType,
  AuskunftssperreType,
  NatuerlichePersonType,
  ProbeType,
  ZustaendigeBehoerdeType,
  create_quality_report_xml,
  parse_quality_report_xml,
  AuftraggeberType,
  Auftraggeber,
  QualityReport,
  ArtDerPerson 
} from "../pkg/xoev_xwasser";

function createHeadInfo(kennung: string, name: string): any {
  return {
    kennung: kennung,
    name: name,
    verzeichnisdienst: {
        code: {
            list_uri: "",
            list_version_id: "",
            code: "",
        },
    },
  }
}

function nachrichtenTyp(): NachrichtenTyp {
  return {
    code: {
        list_uri: "",
        list_version_id: "",
        code: "",
    },
  }
}

function code(name: string, code: string): any {
  return {
    code: code,
    name: name,
  }
}

function geburtType():GeburtType {
  return {
    datum: "datum",
    zusatz: "zusatz",
    geburtsort: anschriftType(),
  }
}

function alternativeRepraesentationType():AlternativeRepraesentationType {
  return {
    repraesentation: "r1",
    algorithmus: "algorithm",
    hinweis: "hinweis",
  }
}

function allgemeinerNameType(name: string): AllgemeinerNameType {
  return {
    name: name,
    nicht_vorhanden: false,
    namensart: {
        code: "1010"
    },
    alternative_repraesentation: [alternativeRepraesentationType()],
  }
}

function verwaltungspolitischeKodierungType():VerwaltungspolitischeKodierungType {
  return {
    kreis: code("name","1111"),
    bezirk: code("name","1111"),
    bundesland: code("name","1111"),
    gemeindeschluessel: code("name","1111"),
    regionalschluessel: code("name","1111"),
    nation: code("name","1111"),
  }
}


function anschriftType(): AnschriftType {
  return  {
    id: "IDfcfd2538-f074-4848-b443-d15997e42c9e",
    strassenschluessel: code("name","1234"),
    strasse: "Musterstr.",
    hausnummer: "1",
    postfach: "1234",
    postleitzahl: "123456",
    ort: "Musterort",
    ortsteil: "Ortsteil",
    ort_frueherer_gemeindename: "frueherer_gemeindename",
    wohnungsgeber: "",
    zusatz: "",
    typ: [code("anschriftType","1234")],
    staat: code("staat","1111"),
    verwaltungspolitische_kodierung: verwaltungspolitischeKodierungType(),
  }
}

function zeitraumType(beginn: string, ende: string, zusatz: string): ZeitraumType {
  return {                                                    
    beginn: beginn,
    ende: ende,
    zusatz: zusatz,
  }
}

function identifikationType(): IdentifikationType {
  return {
    id: "238b7cc7-6d64-4db8-9c69-779bb65d60b1",
    beschreibung: "beschreibung",
    gueltigkeit: zeitraumType("9","19","zusatz"),

  }
}

function kommunikationType(kennung: string, ist_dienstlich: boolean, zusatz: string): KommunikationType {
  return {
    kanal: code("kanal","1"),
    kennung: kennung,
    ist_dienstlich: ist_dienstlich,
    zusatz:zusatz,
  }
}

function spracheType(sprache: string, zusatz: string):SpracheType {
  return {
    sprache: sprache, 
    zusatz: zusatz
  }
}

function  vertreterBevollmaechtigterType():VertreterBevollmaechtigterType {
  return {
    vertreter_bevollmaechtigter_id: "1234",
    art_vertreter: code("vertretungsart","1233")
  }
}

function auskunftssperreType(): AuskunftssperreType {
  return {
    grund: code("auskunftsperretype","1"),
    gueltigkeit: zeitraumType("9","19","zusatz"),
  }
}


function familienstandType():FamilienstandType {
    return {
      familienstand: code("CodeFamilienstandType","1234"),
      zusatz: "zusatz",
      grund: code("CodeFamilienstandBeendigungsgrundType","1234"),
      gueltigkeit: zeitraumType("9","19","zusatz"),
      behoerde: behoerdeType(),
    }
}

function staatsangehoerigkeitType():StaatsangehoerigkeitType {
    return {
      staatsangehoerigkeit: code("CodeStaatsangehoerigkeitType","1234"),
    }
}

function staatType():StaatType {
    return {
      staat: code("CodeStaatType","1234"),
    }
}


function ausweisdokumentType():AusweisdokumentType {
    return {
      ausweisart: code("CodeAusweisdokumenteType","1234"),
      gueltigkeit: zeitraumType("9","19","zusatz"),
      ausweis_id: identifikationType(),
      ausstellende_behoerde: behoerdeType(),
      ausstellender_staat: staatType(),
    }
}

function nameNatuerlichePersonType(): NameNatuerlichePersonType {
  return {
    titel: "Prof",
    anrede: ["Herr", "von und zu"],
    namenssuffix: [
      "Sir",
      "Jr"
    ],

    familienname: allgemeinerNameType("familienname"),
    ehename: allgemeinerNameType("ehename"),
    lebenspartnerschaftsname: allgemeinerNameType("lebenspartnerschaftsname"),
    geburtsname: allgemeinerNameType("geburtsname"),
    frueherer_familienname: [allgemeinerNameType("frueherer_familienname")],
    vorname: allgemeinerNameType("vorname"),
    rufname: allgemeinerNameType("rufname"),
    frueherer_vorname: allgemeinerNameType("frueherer_vorname"),
    alternative_repraesentation: alternativeRepraesentationType(),
    ordensname: allgemeinerNameType("Ordensname"),
    kuenstlername: [allgemeinerNameType("kuenstlername")],
    weiterer_name: [allgemeinerNameType("weiterer_name")],
  } 
}

function natuerlichePerson(): NatuerlichePersonType {
  return {
    auskunftssperre: [auskunftssperreType()],
    name_natuerliche_person: nameNatuerlichePersonType(),
    familienstand: [code("familienstand","1111")],
    geburt: geburtType(),
    doktorgrad: {
        bezeichnung: "DR."
      },
    staatsangehoerigkeit: [code("staatsangehoerigkeit","1111")],
    ausweisdokument: [code("ausweisdokument","1234")],
    anschrift: [anschriftType()],
    geschlecht: [code("geschlecht","1111")],
    identifikationsnummer: [identifikationType()],
    kommunikation: [kommunikationType("text", true, "zusatz")],
    muttersprache: [spracheType("Deutsch", "zusatz")],
    fremdsprache: [spracheType("Englisch", "zusatz")],
    vertreter_bevollmaechtigter: [vertreterBevollmaechtigterType()]
  }
}

function organisationseinheitType():any {
  return {
    name: "",
    hierarchieebene: 1,
    hierarchiename: "String",
  }
}

function nameOrganisationType(name: string, kurzbezeichnung: string, gueltigkeit: ZeitraumType): NameOrganisationType {
  return {
    name: name, // wenn es nicht optional type ist wird gemeldet dass hier name fehlt .. why ?? 
    kurzbezeichnung: kurzbezeichnung,
    gueltigkeit: gueltigkeit,
  }
}

function  behoerdeType():BehoerdeType {
  return {
    id: "ID753d97bc-4262-45e8-8c1f-cb7b6ab7864a",
    typ: code("behoerde","1234"),
    zusatz: "zusatz",
    behoerdenkennung: {
      kennung: [code("behoerdenkennung","1234")],
      praefix: [code("praefixtype","1234")]
    },
    kommunikation: [kommunikationType("text", true, "zusatz")],
    behoerdenidentifikation: identifikationType(),
    behoerdenname: [nameOrganisationType("behorde 1", "", zeitraumType("9","19","zusatz"))],
    nachgeordnete_behoerde: [],
    verwaltungspolitische_zustaendigkeit: [verwaltungspolitischeKodierungType()],
    anschrift: [anschriftType()],
    organisationsstruktur: [organisationseinheitType()]
  }
}

function  registrierungType():RegistrierungType {
  return {
    id: "ID753d97bc-4262-45e8-8c1f-cb7b6ab7864a",
    registertyp: code("registertyp","1234"),
    registrierende_behoerde: behoerdeType(),
    gueltigkeit: zeitraumType("9","19","zusatz"),
  }
}

function organisationType(): OrganisationType  {
  return {
    rechtsform: code("rechtsform", "0815"),
    branche: [code("branche", "666")],
    zweck: [code("zweck", "666")],
    name: nameOrganisationType("abc gmbh", "produces stuff",zeitraumType("9","19","zusatz")),
    unterorganisation: [],
    kommunikation: [kommunikationType("text", true, "zusatz")],
    registrierung: [registrierungType()],
    identifikation: [identifikationType()],
    existenzzeitraum: zeitraumType("9","19","zusatz"),
    anschrift: [anschriftType()]
  }
}

function  analyseergebnisParameter():AnalyseergebnisParameter {
  return {
    analyseergebnis_parameter_id: "ID753d97bc-4262-45e8-8c1f-cb7b6ab7864a",
    probe_id: "ID3cd3c929-ee22-4056-a2a0-6c2d1c295c5c",
    zugelassene_untersuchungsstelle_id: "ID14aeb6cd-bc5e-443f-890c-cbdfe6f50c86",
    anschrift_id: "IDfcfd2538-f074-4848-b443-d15997e42c9e",
    analyse_im_rahmen_der_akkreditierung: true,
    untersuchungsverfahren: [code("untersuchungsverfahren","9919")],
    ergaenzung_zum_untersuchungsverfahren: "ergaenzung_zum_untersuchungsverfahren",
    untersuchter_parameter: code("untersuchter_parameter","1210"),
    parameterauspraegung: code("parameterauspraegung","1210"),
    parameter_unterauswahl: code("parameter_unterauswahl","1210"),
    sensorischer_parameter_ist_annehmbar: true,
    untersuchungswert_parameter: "1000 gramm",
    einheit_des_untersuchungswerts: code("einheit_des_untersuchungswerts","1210"),
    ergaenzung_zum_untersuchungswert_parameter: code("ergaenzung_zum_untersuchungswert_parameter","1210"),
    parameterwert_ergaenzung: "parameterwert_ergaenzung",
    ausgewertetes_ansatzvolumen: "ausgewertetes_ansatzvolumen",
    shapth_parameter_nummer: ["shapth_parameter_nummer"],
    bewertung_untersuchungswert: code("bewertung_untersuchungswert","9999"),
    parameterauffaelligkeit: "parameterauffaelligkeit",
    messunsicherheit_untersuchungswert: "messunsicherheit_untersuchungswert",
    bestimmungsgrenze_lo_q: "bestimmungsgrenze_lo_q",
    kommentar: "kommentar",
  }
}


function zustaendigeBehoerde():ZustaendigeBehoerdeType {
  return {
    behoerde: behoerdeType(),
    anlage_nach_trinkw_vid: ["IDfcfd2538-f074-4848-b443-d15997e42c9e"],
    probennehmer_id: ["1234"],
    laenderkuerzel: code("laenderkuerzel","DEAA"),
    kommentar: "kommentar"
  }
}

function  probennehmer():Probennehmer {
  return {
    probennehmer_id: "IDb05bfc54-c2a9-4ff1-92c8-47b2c4fd9804",
    probennehmer: {
      // alle möglichen Typen
      natuerliche_person: natuerlichePerson(),
      // organisation: organisationType(),
      // zustaendige_behoerde: zustaendigeBehoerde(),
    },
    fremdsystem_id_probennehmer: "fremdsystemid",
    kommentar: "kommentar",
  }
}

function probe(): ProbeType {
  return {
    probe_id: "ID3cd3c929-ee22-4056-a2a0-6c2d1c295c5c",
    analyseergebnis_parameter: [analyseergebnisParameter()],
    probennehmer: probennehmer(),
    anlass_der_untersuchung: [code("name","1010")],
    medium: code("medium","1010"),
    ergaenzung_zum_medium: "Wasser",
    zeitpunkt_probennahme: "2024-05-27T09:00:00",
    probennahmeverfahren: [code("probennahmeverfahren","1010")],
    probenentnahmegeraet: code("probenentnahmegeraet","1234"),
    analyseergebnis_parameter_id: "ID753d97bc-4262-45e8-8c1f-cb7b6ab7864a",
    probengefaess: code("probengefaess","1234"),
    ergaenzende_informationen_zu_probenentnahmegeraet: "",
    desinfektion_probenentnahmegeraet_durchgefuehrt: true,
    konservierung_aufbereitung_desinfektion_probe: [code("konservierung_aufbereitung_desinfektion_probe","1234")],
    kommentar_zur_probennahme: "",
    informationen_zum_probentransport: "Lieferwagen",
    eingang_probe_bei_untersuchungsstelle: "2024-05-27T10:00:00",
    beginn_analytik: "2024-05-27T10:01:00",
    abschluss_analytik: "2024-05-27T10:02:00",
    probenbewertung: code("probenbewertung","1010"),
    berichtspflichtig: true,
    von_probennehmer_vergebene_probe_id: "123456789",
    probe_id_aus_labor: "1233",
    anhang: ["Anhang"],
    kommentar: "Kommentar",
  }
}

function  probennahmestelleType():ProbennahmestelleType {
  return {
    probennahmestelle_id: "ID14aeb6cd-bc5e-443f-890c-cbdfe6f50c86",
    objekt_id: "",
    probe: [probe()],
    terminplan_id: ["terminplan id"],
    name_probennahmestelle: "wasserloch um die ecke",
    art_probennahmestelle: code("wasserloch","123455"),
    stockwerk_probennahmestelle: 1,
    medium_an_der_probennahmestelle: [code("medium probennahmestelle","1235")],
    desinfektion_und_aufbereitung_des_wassers: [code("desinfektionAufbereitungDesWassers","1234")],
    alt_id: "altid2",
    kommentar: "kommentar zur proebennahmestelle",
  }
}

function auftraggeber(): Auftraggeber {
  return {
      t: "NatuerlichePerson",
      c: natuerlichePerson(),
  }
}


function auftraggeberType():AuftraggeberType {
  return {
    auftraggeber_id: "ID4510d774-13a7-414f-82b0-60e8176e5e19",
    auftraggeberart: code("name","1010"),
    auftraggeber: auftraggeber(),
  }
}

function  zugelasseneUntersuchungsstelleType():ZugelasseneUntersuchungsstelleType {
  return {
    organisation: organisationType(),
    zugelassene_untersuchungsstelle_id: "ID14aeb6dd-bc5e-443f-890c-cbdfe6f50c86",
    name_zugelassene_untersuchungsstelle: code("name","09010"),
    pruefgebiete_untersuchungen_phys_chem: true,
    pruefgebiete_untersuchungen_mikrobio: true,
    pruefgebiete_untersuchungen_radionuklide: true,
    akkreditierungsnummer: "",
    kommentar_beauftragte_untersuchungsstelle: "",
    kommentar_zugelassene_untersuchungsstelle: "",
  }
}

function  beauftragteUntersuchungsstelleType():BeauftragteUntersuchungsstelleType {
  return {
    zugelassene_untersuchungsstelle: zugelasseneUntersuchungsstelleType(),
    kommentar_beauftragte_untersuchungsstelle: "",
  }
}

function erweiterung():any {
  return {
    feld: [
      {
        name: "feldname",
        beschreibung: "feldbeschreibung",
        datentyp: code("datentyp","1234"),
        wert: "feldwert"
      }
    ],
    gruppe: [
      {
        name: "gruppe",
        beschreibung: "gruppenbeschreibung",
        untergruppe: [],
        feld: [],
      }
    ],
    xml: {
      any: []
    },

  }

}

function pruefberichtType():PruefberichtType {
  return {
    pruefbericht_uuid: "IDcd7df392-08ca-4915-bbd9-c14be7b69d02",
    untersuchungsplan_id: "IDcd7df392-08ca-4915-bbd9-c14be7b69d02",
    probennahmestelle: [probennahmestelleType()],
    name_beauftragte_untersuchungsstelle: code("name","1010"),
    pruefbericht_enthaelt_teilergebnisse: true,
    pruefgericht_gem_vorgaben_akkredition: true,
    titel: "",
    gesamtbewertung: code("name","1010"),
    auffaelligkeiten: ["ids are duplicated, makes no sense"],
    zeitpunkt_validierung_pruefbericht: "2024-05-28T09:10:00",
    fuer_validierung_verantwortliche_person: [natuerlichePerson()],
    freigabe_uebermittlung_betreiber: true,
    pruefbericht_id_labor: "aaa",
    sw_version: code("sw_version", "1234"),
    sprache_pruefbericht: code("name","DE"),
    rechtlicher_disclaimer: "",
    zeitpunkt_uebermittlung_an_shapth: "2024-05-28T09:11:00",
    kommentar: "kommentar",
    auftraggeber: auftraggeberType(),
    zustaendige_behoerde: [zustaendigeBehoerde()],
    beauftragte_untersuchungsstelle: beauftragteUntersuchungsstelleType(),
    ort_der_labortaetigkeiten: [anschriftType()],
    anhang: ["anhang"],
    erweiterung: erweiterung(),
  }
}

function terminplanType(): TerminplanType {
    return { 
      terminplan_id: "terminplan_id",
      probennahmestelle_id: "probennahmestelle_id",
      datum_zeitraum: ["datum_zeitraum"],
      probennahmestelle_kategorie: code("CodeKategorieProbennahmestelleType","1234"),
      weitere_beschreibung_der_probennahmestelle: "weitere_beschreibung_der_probennahmestelle",
      untersuchung_durch: [code("CodeUeberwachungAufbereitungType","1234")],
      zu_untersuchende_parameter: [code("CodeShapthParameterType","1234")],
      probennahmeverfahren: [code("CodeProbennahmeverfahrenType","1234")],
      kommentar: "kommentar", 
    }
}

function geokoordinatenShapthType():GeokoordinatenShapthType {
  return {
    geografische_position_und_ausdehnung: "geografische_position_und_ausdehnung",
    name_shapefile: "name_shapefile",
    geokoordinaten_breitengrad: 1.11,
    geokoordinaten_laengengrad: 1.11,
    geokoordinaten_rechtswert: 1,
    geokoordinaten_hochwert: 1,
  }
}

function derogationRemedialActionType():DerogationRemedialActionType {
    return {
      derogation_remedial_action_identifier: "derogation_remedial_action_identifier",
      derogation_remedial_action: code("CodeAbhilfemassnahmeType","1234"),
      derogation_remedial_action_start_date: "derogation_remedial_action_start_date",
      derogation_remedial_action_end_date: "derogation_remedial_action_end_date",
      derogation_remedial_action_cost: 1.11,
      remarks: "remarks",
    }
}

function qualityAndMonitoringType():QualityAndMonitoringType {
  return {
    quality_and_monitoring_requirement_identifier: "quality_and_monitoring_requirement_identifier",
    parameter_code: code("CodeShapthParameterType","1234"),
    parameter_threshold_value: 1.11,
    parameter_threshold_value_unit: code("CodeShapthParameterEinheitType","1234"),
    sampling_frequency: 1,
    sampling_period: code("CodeProbennahmezeitraumType","1234"),
    sampling_location_type: [code("CodeArtProbennahmestelleEUType","1234")],
    remarks: "remarks",
  }
}

function derogationType():DerogationType {
  return {
    derogation_identifier: "derogation_identifier",
    trivial_derogation: true,
    trivial_derogation_justification: "trivial_derogation_justification",
    derogation_start_date: "derogation_start_date",
    derogation_end_date: "derogation_end_date",
    volume_of_water_supplied: 1.11,
    derogation_affected_population: 1,
    food_production_affected: true,
    derogation_under_recast_dwd: true,
    derogation_grounds: code("CodeGrundAusnahmeregelungType","1234"),
    previous_derogation_identifier: "previous_derogation_identifier",
    previous_derogation_conclusions: "previous_derogation_conclusions",
    previous_derogation_start_date: "previous_derogation_start_date",
    previous_derogation_end_date: "previous_derogation_end_date",
    previous_derogation_grounds: code("CodeGrundAusnahmeregelungType","1234"),
    remarks: "remarks",
    derogation_remedial_action: [derogationRemedialActionType()],
    quality_and_monitoring: [qualityAndMonitoringType()],
  }
}

function exceedanceCauseAndRemedialActionType():ExceedanceCauseAndRemedialActionType {
  return {
    exceedance_cause_and_remedial_action_identifier: "exceedance_cause_and_remedial_action_identifier",
    exceedance_cause: code("CodeIncidentExceedanceCauseType","1234"),
    exceedance_remedial_action: code("CodeMassnahmeType","1234"),
    exceedance_remedial_action_start_date: "exceedance_remedial_action_start_date",
    exceedance_remedial_action_end_date: "exceedance_remedial_action_end_date",
    remarks: "remarks",
  }
}

function exceedanceType():ExceedanceType {
  return {
    exceedance_identifier: "exceedance_identifier",
    trivial_exceedance: true,
    trivial_exceedance_justification: "trivial_exceedance_justification",
    parameter_code: code("CodeShapthParameterType","1234"),
    exceedance_start_date: "exceedance_start_date",
    exceedance_end_date: "exceedance_end_date",
    exceedance_affected_population: 1,
    point_of_compliance_type: [code("CodeArtProbennahmestelleEUType","1234")],
    number_of_samples_per_year: 1,
    incident_identifier: "incident_identifier",
    derogation_identifier: "derogation_identifier",
    remarks: "remarks",
    exceedance_cause_and_remedial_action: [exceedanceCauseAndRemedialActionType()],
  }
}

function incidentCauseAndRemedialActionType():IncidentCauseAndRemedialActionType {
  return {
    incident_cause_and_action_identifier: "incident_cause_and_action_identifier",
    incident_cause: code("CodeIncidentExceedanceCauseType","1234"),
    incident_remedial_action: code("CodeMassnahmeType","1234"),
    incident_remedial_action_start_date: "incident_remedial_action_start_date",
    incident_remedial_action_end_date: "incident_remedial_action_end_date",
    remarks: "remarks",
  }
}

function incidentType():IncidentType {
  return {
    incident_identifier: "incident_identifier",
    exceedance: ["exceedanceString"],
    incident_start_date: "incident_start_date",
    incident_end_date: "incident_end_date",
    incident_category: [code("CodeIncidentCategoryType","1234")],
    incident_affected_population: 1,
    remarks: "remarks",
    incident_cause_and_remedial_action: [incidentCauseAndRemedialActionType()],
  }
}

function wasserversorgungsgebietType(): WasserversorgungsgebietType {
  return {
    wasserversorgungsgebiet_id: "wasserversorgungsgebiet_id",
    name_wasserversorgungsgebiet: code("CodeWasserversorgungsgebietType","1234"),
    lau2_code: "lau2_code",
    zustaendige_behoerde: [zustaendigeBehoerde()],
    geokoordinaten_shapth: geokoordinatenShapthType(),
    datum_der_einrichtung: "datum_der_einrichtung",
    datum_der_schliessung: "datum_der_schliessung",
    grund_der_schliessung: code("CodeGrundSchliessungWasserversorgungsgebietType","11234"),
    nachfolger_wvg_bei_schliessung: ["nachfolger_wvgbei_schliessung"],
    wvg_fremdbezogen: ["wvg_fremdbezogenString"],
    abgegebene_wassermenge: 1.11,
    anzahl_versorgte_personen_wvg: 1,
    referenzjahr_angaben_wvg: 1,
    art_der_wasserressource: [code("CodeArtWasserressourceType","1234")],
    anteil_der_wasserressource: [1],
    vorgeschriebene_untersuchungshaeufigkeit_parameter_a: 1,
    vorgeschriebene_untersuchungshaeufigkeit_parameter_b: 1,
    alt_id: "alt_id",
    kommentar: "kommentar",
    derogation: [derogationType()],
    exceedance: [exceedanceType()],
    incident: [incidentType()],
  }
}

function artDerPerson(): ArtDerPerson {
  return {
    t: "NatuerlichePerson",
    c: natuerlichePerson(),
  }
}

function betreiberType():BetreiberType {
  return {
    betreiber_id: "betreiber_id",
    art_der_person: artDerPerson(),
    kommentar: "kommentar",
  }
}

function objektType():ObjektType {
  return {
    objekt_id: "objekt_id",
    wasserversorgungsgebiet: "wasserversorgungsgebiet",
    anschrift_objekt: [anschriftType()],
    art_objekt: code("CodeArtObjektType","1234"),
    name_objekt: "name_objekt",
    betriebszustand_des_objekts: code("CodeBetriebszustandType","1234"),
    datum_in_betriebnahme: "datum_in_betriebnahme",
    datum_ausser_betriebnahme: "datum_ausser_betriebnahme",
    rahmen_der_trinkwasserbereitstellung: [code("CodeRahmenTrinkwasserbereitstellungType","1234")],
    geokoordinaten_objekt: geokoordinatenShapthType(),
    alt_id: "alt_id",
    kommentar: "kommentar",
    betreiber: [betreiberType()],
    objekt_probennahmestelle: [probennahmestelleType()],
  }
}

function anlageNachTrinkwVType():AnlageNachTrinkwVType {
  return {
    anlage_nach_trinkw_vid: "anlage_nach_trinkw_vid",
    zustaendige_behoerde_id: "zustaendige_behoerde_id",
    untersuchungsplan_id: ["untersuchungsplan_id"],
    art_anlage: code("CodeArtTrinkwasseranlageType","1234"),
    name_der_anlage: "name_der_anlage",
    abgegebene_wassermenge_der_anlage_pro_tag: 5.55,
    anzahl_durch_anlage_versorgte_personen: 10000,
    alt_id: "alt_id",
    kommentar: "kommentar",
    wasserversorgungsgebiet: [wasserversorgungsgebietType()],
    anlage_nach_trinw_v_objekt: [objektType()],
  }
}

function untersuchungsplanType(): UntersuchungsplanType { 
  return {
      untersuchungsplan_id: "untersuchungsplan_id",
      wasserversorgungsgebiet: ["wasserversorgungsgebiet"],
      jahr: ["jahr"],
      wasserabgabe_vorjahr: 5.01,
      art_von_wva_und_wvg: code("CodeWVAType","1234"),
      erlaeuterung_zur_wasserabgabemenge: code("CodeErlaeuterungWasserabgabemengeType","1234"),
      flockung: code("CodeFlockungType","1234"),
      oberflaechenwassereinfluss: true,
      desinfektion_durchgefuehrt_mit: code("CodeDesinfektionsartType","1234"),
      abfuellung_zur_abgabe_in_verschlossenen_behaeltnissen: true,
      acrylamid: code("CodeNachweisartType","1234"),
      epichlorhydrin: code("CodeNachweisartType","1234"),
      vinylchlorid: code("CodeNachweisartType","1234"),
      ph_wert_wasserwerksausgang: true,
      wasserabgabe_vorjahr_pro_tag: 1.11,
      anzahl_untersuchungenpro_jahr_gruppe_a: 1,
      abzudecken_durch_betreiber_gruppe_a: 1,
      anzahl_untersuchungenpro_jahr_gruppe_b: 1,
      abzudecken_durch_betreiber_gruppe_b: 1,
      rap_durchgefuehrt: true,
      status_untersuchungsplan: code("CodeStatusUntersuchungsplanType","1234"),
      kommentar: "Kommentar",
      terminplan: [terminplanType()],
      anlage_nach_trinkw_v: anlageNachTrinkwVType(),
      auftraggeber: auftraggeber(),
      zustaendige_behoerde: zustaendigeBehoerde(),
      probe_rel: [probe()],
      erweiterung: erweiterung(),
  }
}

function signaturenType():SignaturenType {
    return {
      signatur_dokument_id: ["signatur_dokument_id"],
    }
}


function dokumentRepraesentationType():DokumentRepraesentationType {
    return {
      referenz: "referenz",
      mime_type: "mime_type",
      inhalt: "inhalt",
      externer_referenz_typ: "externer_referenz_typ",
      externer_referenz_index: "externer_referenz_index",
      inhalt_typ: "inhalt_typ",
      signaturen: signaturenType(),
    }
}

function juristischePersonType(): JuristischePersonType {
  return {
    bundeseinheitliche_wirtschaftsnummer: "bundeseinheitliche_wirtschaftsnummer",
    rechtsform: code("CodeRechtsformenType","1234"),
    eingetragener_name: "eingetragener_name",
    eintragung: "eintragung",
    geschaeftsbezeichnung: "geschaeftsbezeichnung",
    anschrift: [anschriftType()],
    sitz: "sitz",
    effektiver_verwaltungssitz: "effektiver_verwaltungssitz",
    kommunikation: [kommunikationType("text", true, "zusatz")],
    vertreter_bevollmaechtigter: [vertreterBevollmaechtigterType()],
  }
}

function dokumentType():DokumentType {
    return {
      dokument_typ: code("CodeDokumenttypType","1234"),
      name: "name",
      aktuelle_version: "aktuelle_version",
      letzte_version: "letzte_version",
      dokument_repraesentation: [dokumentRepraesentationType()],
      person_referenz_id: ["person_referenz_id"],
    }
}

function vorgangType(): VorgangType {
  return {
    t: "Pruefbericht",
    c: pruefberichtType(),
    // t: "Untersuchungsplan",
    // c: untersuchungsplanType(),
    // t: "OlgBericht",
    // c: dokumentType(),
  }
}

function vorgang():any {
  return {
    identifikation_vorgang: {
        vorgangs_id: "5e08e073-4e06-438d-9444-1275f6cbf061",
    },
    vorgang_type: vorgangType(),
    bemerkung: "Bemerkung",
  }
}

function nachrichtenkopf():any {
  return {
    leser: createHeadInfo("psw:11113110", "Reader"),
    autor: createHeadInfo("psw:01003110", "Author"),
    identifikation_nachricht: {
        nachrichten_uuid: "693c64d6-456f-4d14-abe7-fe9681c74aae",
        nachrichten_typ: nachrichtenTyp(), 
        erstellungszeitpunkt: "2024-05-28T09:00:00",
    },
    referenz_uuid: "238b7cc7-6d64-4db8-9c69-779bb65d60b1",
    dvdv_dienstkennung: "s",
  }
}

function qualityReport():QualityReport {
  return {
    produkt: "SHAPTH CLI",
    test: true,
    nachrichtenkopf_g2g: nachrichtenkopf(),
    vorgang: vorgang(),
  }
}

describe("simple xml generation via wasm", async () => {
  it("should be able to create and parse quality report xml", async () => {
    console.log(JSON.stringify(qualityReport(), null, 2));
    
    // const xml = create_quality_report_xml(qualityReport());
    // console.log(xml);
  });
});
