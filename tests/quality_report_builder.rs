#[cfg(feature = "builder")]
#[test]
fn test_quality_report_builder() -> anyhow::Result<()> {
    use xoev_xwasser::{
        builder::{
            shared::anschrift::anschrift_type,
            transport::NachrichtenTypEnum,
            utils::{new_uuid, now},
        },
        model::{
            pruefbericht::PruefberichtType,
            shared::{
                auftraggeber::{Auftraggeber, AuftraggeberType},
                behoerde::ZustaendigeBehoerdeType,
                organisation::OrganisationType,
                person::NatuerlichePersonType,
                probe::{
                    AnalyseergebnisParameterType, ProbeType, ProbennahmestelleType, Probennehmer,
                    ProbennehmerType,
                },
                unterssuchungsstelle::BeauftragteUntersuchungsstelleType,
            },
            transport::{NachrichtenkopfG2g, VorgangTransportieren2010},
            vorgang::{IdentifikationVorgang, Vorgang},
        },
    };
    let identifikation_nachricht = xoev_xwasser::builder::transport::identifikation_nachricht(
        NachrichtenTypEnum::VorgangTransportieren2010,
    );

    // whp -> Wasserhygieneportal
    // ghb -> Gesundheitsbehörde
    // bwv -> Betreiber Wasserversorgungsanlage
    // wus -> Wasseruntersuchungsstelle
    let autor = xoev_xwasser::builder::transport::behoerde_g2g_type("lab 1", "wus:1");
    let leser = xoev_xwasser::builder::transport::behoerde_g2g_type("wsp 1", "bwv:1");

    let fuer_validierung_verantwortliche_person = NatuerlichePersonType::builder()
        .auskunftssperre(Default::default())
        .name_natuerliche_person(Default::default())
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
        .id(Some("bevollmaechtigter-1".into()))
        .build();

    let probennehmer = Probennehmer::NatuerlichePerson(
        NatuerlichePersonType::builder()
            .auskunftssperre(Default::default())
            .name_natuerliche_person(Default::default())
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
            .id(Default::default())
            .build(),
    );

    let probennehmer = ProbennehmerType::builder()
        .probennehmer_id(Some(new_uuid()))
        .probennehmer(probennehmer)
        .fremdsystem_id_probennehmer(Default::default())
        .kommentar(Default::default())
        .id("probennehmer-1".into())
        .build();

    let nachrichtenkopf_g2g = NachrichtenkopfG2g::builder()
        .autor(autor)
        .leser(leser)
        .dvdv_dienstkennung("1".into())
        .referenz_uuid(None)
        .identifikation_nachricht(identifikation_nachricht)
        .zustaendige_behoerde_id(Default::default())
        .build();

    let identifikation_vorgang = IdentifikationVorgang::builder()
        .vorgangs_id(new_uuid())
        .build();

    let ort_der_labortaetigkeiten = anschrift_type(
        "strasse".into(),
        "hausnummer".into(),
        "postleitzahl".into(),
        "ort".into(),
    );

    let param = AnalyseergebnisParameterType::builder()
        .analyseergebnis_parameter_id(new_uuid())
        .anschrift_id(Some(ort_der_labortaetigkeiten.id.clone()))
        .zugelassene_untersuchungsstelle("wsu-1".into())
        .akkreditierte_durchfuehrung_analyse(Default::default())
        .zugelassene_durchfuehrung_analyse(true)
        .untersuchungsverfahren(vec!["1010".into()])
        .ergaenzung_zum_untersuchungsverfahren(Default::default())
        .untersuchter_parameter("1021".into())
        .parameterauspraegung(Some("10001-1".into()))
        .parameter_durch_betreiber_untersucht(Default::default())
        .wurde_der_parameter_korrigiert(Default::default())
        .untersuchungswert_parameter(Some(0.0))
        .einheit_des_untersuchungswerts(Some("1400".into()))
        .ergaenzung_zum_untersuchungswert_parameter(Default::default())
        .parameterwert_ergaenzung(Default::default())
        .ausgewertetes_ansatzvolumen(Some(0.0))
        .shapth_parameter_nummer(Default::default())
        .bewertung_untersuchungswert("1010".into())
        .parameterauffaelligkeit(Default::default())
        .messunsicherheit_untersuchungswert_absolut(Default::default())
        .messunsicherheit_untersuchungswert_relativ(Default::default())
        .bestimmungsgrenze_lo_q(Some(0.0))
        .kommentar(Default::default())
        .id("param-1".into())
        .build();

    let probenahmestelle_id = format!("id-{}", new_uuid());

    let probe = ProbeType::builder()
        .probe_id(new_uuid())
        .probennahmestelle(probenahmestelle_id.clone())
        .untersuchungsplan_id(Default::default())
        .probennehmer(Default::default())
        .titel_probe(Default::default())
        .analyseergebnis_parameter(vec![param])
        .anlass_der_untersuchung(vec!["1010".into()])
        .medium(Some("1010".into()))
        .akkreditierte_durchfuehrung_der_probennahme(Default::default())
        .ergaenzung_zum_medium(Default::default())
        .zeitpunkt_probennahme(now())
        .probennahmeverfahren(vec!["1010".into()])
        .probenentnahmegeraet(Some("9020".into()))
        .probengefaess(Default::default())
        .ergaenzende_informationen_zu_probenentnahmegeraet(Default::default())
        .desinfektion_probenentnahmegeraet_durchgefuehrt(Default::default())
        .konservierung_der_probe(vec!["1020".into()])
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
        .id("probe-1".into())
        .build();

    let probennahmestelle = ProbennahmestelleType::builder()
        .probennahmestelle_id(probenahmestelle_id.clone())
        .wasserversorgungsgebiet_id(None)
        .objekt_id(Some("none".into()))
        .probe(Default::default())
        .terminplan_id(Default::default())
        .name_probennahmestelle(Default::default())
        .kategorie_probennahmestelle("L".into())
        .unterkategorie_probennahmestelle(Some("1030".into()))
        .art_der_entnahmearmatur(Some("1010".into()))
        .stockwerk_probennahmestelle(0.into())
        .medium_an_der_probennahmestelle(vec!["1010".into()])
        .desinfektion_und_aufbereitung_des_wassers(Default::default())
        .angaben_alternative_id(None)
        .berichtspflichtig(Default::default())
        .kommentar(Default::default())
        .id(probenahmestelle_id)
        .build();

    let auftraggeber = Auftraggeber::Organisation(
        OrganisationType::builder()
            .id(Default::default())
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
            .build(),
    );

    let auftraggeber = AuftraggeberType::builder()
        .auftraggeber_id(Default::default())
        .auftraggeberart("1010".into())
        .auftraggeber(auftraggeber)
        .build();

    let zustaendige_behoerde = ZustaendigeBehoerdeType::builder()
        .id(Default::default())
        .typ(Default::default())
        .zusatz(Default::default())
        .behoerdenkennung(Default::default())
        .kommunikation(Default::default())
        .behoerdenidentifikation(Default::default())
        .behoerdenname(Default::default())
        .nachgeordnete_behoerde(Default::default())
        .verwaltungspolitische_zustaendigkeit(Default::default())
        .anschrift(Default::default())
        .organisationsstruktur(Default::default())
        .anlage_nach_trinkw_vid(Default::default())
        .laenderkuerzel("DEBY".into())
        .kommentar(Default::default())
        .build();

    let beauftragte_untersuchungsstelle = BeauftragteUntersuchungsstelleType::builder()
        .id(Some("wsu-1".into()))
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
        .zugelassene_untersuchungsstelle_id(Default::default())
        .pruefgebiete_untersuchungen_phys_chem(Default::default())
        .pruefgebiete_untersuchungen_mikrobio(Default::default())
        .pruefgebiete_untersuchungen_radionuklide(Default::default())
        .pruefgebiete_nur_vor_ort_parameter(Default::default())
        .akkreditierungsnummer(Default::default())
        .kommentar_zugelassene_untersuchungsstelle(Default::default())
        .kommentar_beauftragte_untersuchungsstelle(Default::default())
        .build();

    let pruefbericht = PruefberichtType::builder()
        .pruefbericht_uuid(new_uuid())
        .versionsnummer(Some(1))
        .auftragsnummer(new_uuid())
        .probennahmestelle(vec![probennahmestelle])
        .probe(vec![probe])
        .probennehmer(vec![probennehmer])
        .pruefbericht_enthaelt_teilergebnisse(Default::default())
        .korrekturvermerk(Default::default())
        .pruefbericht_gem_vorgaben_akkredition(Default::default())
        .titel("Prüfbericht 1".into())
        .gesamtbewertung("1010".into())
        .auffaelligkeiten(Default::default())
        .zeitpunkt_validierung_pruefbericht(now())
        .fuer_validierung_verantwortliche_person(vec![fuer_validierung_verantwortliche_person])
        .freigabe_uebermittlung_betreiber(Default::default())
        .pruefbericht_id_labor(Default::default())
        .sw_version(Default::default())
        .sprache_pruefbericht(Default::default())
        .rechtlicher_disclaimer(Default::default())
        .zeitpunkt_uebermittlung_an_shapth(Default::default())
        .kommentar(Default::default())
        .auftraggeber(auftraggeber)
        .zustaendige_behoerde(vec![zustaendige_behoerde])
        .beauftragte_untersuchungsstelle(beauftragte_untersuchungsstelle)
        .zugelassene_untersuchungsstelle(Default::default())
        .ort_der_labortaetigkeiten(vec![ort_der_labortaetigkeiten])
        .anhang(Default::default())
        .erweiterung(Default::default())
        .gefahr_in_verzug(false)
        .anlage_nach_trinkw_v(Default::default())
        .objekt(Default::default())
        .id("pruefbericht-1".into())
        .build();

    let e = VorgangTransportieren2010::builder()
        .produkt("XWasser Test".into())
        .produkthersteller("H&D GmbH".into())
        .produktversion("0.800.0".into())
        .test(Some(true))
        .nachrichtenkopf_g2g(nachrichtenkopf_g2g)
        .vorgang(
            Vorgang::builder()
                .anlage(Default::default())
                .bemerkung(None)
                .identifikation_vorgang(identifikation_vorgang)
                .vorgang_type(xoev_xwasser::model::vorgang::VorgangType::Pruefbericht(
                    pruefbericht,
                ))
                .build(),
        )
        .signature(None)
        .build();

    let json = serde_json::to_string_pretty(&e).unwrap();
    std::fs::write("tests/quality_report_builder.json", json)?;
    let xml = raxb::ser::to_string_pretty_with_decl(&e)?;
    std::fs::write("tests/quality_report_builder_test_result.xml", xml.replace(r#"xsi:schemaLocation="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_9_2/ xwasser.xsd""#,
    r#"xsi:schemaLocation="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_9_2/ ../schemas/V0_9_2/xwasser.xsd""#))?;
    Ok(())
}

#[cfg(feature = "schema")]
fn quality_report_builder_test_result() -> String {
    std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join("tests/quality_report_builder_test_result.xml"),
    )
    .unwrap()
}

#[cfg(feature = "schema")]
#[test]
fn test_quality_report_builder_test_result_against_schema() -> anyhow::Result<()> {
    let s = quality_report_builder_test_result();
    let validation = xoev_xwasser::schemas::XmlValidation::new()?;
    validation.validate(s.as_bytes())?;
    Ok(())
}
