import { describe, it, expect } from "vitest";
import { create_quality_report_xml, parse_quality_report_xml } from "../pkg/xwasser_rs";

const GH_XML = `<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<vorgang.transportieren.2010 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="xwasser ../schemas/V0_2_0/xwasser.xsd" xmlns:xwas="xwasser" produkt="SHAPTH CLI" produkthersteller="H &amp; D GmbH" produktversion="H &amp; D GmbH" standard="XWasser" test="true" version="0.2.0">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>693c64d6-456f-4d14-abe7-fe9681c74aae</nachrichtenUUID>
      <nachrichtentyp listURI="urn:xoev-de:xwasser:codeliste:nachrichtentyp" listVersionID="1">
        <code>2010</code>
      </nachrichtentyp>
      <erstellungszeitpunkt>2024-05-28T09:00:00</erstellungszeitpunkt>
    </identifikation.nachricht>
    <leser>
      <verzeichnisdienst listVersionID="">
        <code></code>
      </verzeichnisdienst>
      <kennung>psw:01003110</kennung>
      <name>Reader</name>
    </leser>
    <autor>
      <verzeichnisdienst listVersionID="">
        <code></code>
      </verzeichnisdienst>
      <kennung>psw:01003110</kennung>
      <name>Author</name>
    </autor>
    <referenzUUID>238b7cc7-6d64-4db8-9c69-779bb65d60b1</referenzUUID>
    <dvdvDienstkennung>s</dvdvDienstkennung>
  </nachrichtenkopf.g2g>
  <xwas:vorgang>
    <xwas:identifikationVorgang>
      <xwas:vorgangsID>5e08e073-4e06-438d-9444-1275f6cbf061</xwas:vorgangsID>
    </xwas:identifikationVorgang>
    <xwas:vorgangType>
      <xwas:pruefbericht>
        <xwas:pruefberichtID>IDcd7df392-08ca-4915-bbd9-c14be7b69d02</xwas:pruefberichtID>
        <xwas:probennahmestelle>
          <xwas:probennahmestelleID>ID14aeb6cd-bc5e-443f-890c-cbdfe6f50c86</xwas:probennahmestelleID>
          <xwas:objektID></xwas:objektID>
          <xwas:probe>
            <xwas:probeID>ID3cd3c929-ee22-4056-a2a0-6c2d1c295c5c</xwas:probeID>
            <xwas:probennahmestelleID>ID14aeb6cd-bc5e-443f-890c-cbdfe6f50c86</xwas:probennahmestelleID>
            <xwas:analyseergebnisParameterID>ID753d97bc-4262-45e8-8c1f-cb7b6ab7864a</xwas:analyseergebnisParameterID>
            <xwas:analyseergebnisParameter>
              <xwas:analyseergebnisParameterID>ID753d97bc-4262-45e8-8c1f-cb7b6ab7864a</xwas:analyseergebnisParameterID>
              <xwas:probeID>ID3cd3c929-ee22-4056-a2a0-6c2d1c295c5c</xwas:probeID>
              <xwas:zugelasseneUntersuchungsstelleID>ID14aeb6cd-bc5e-443f-890c-cbdfe6f50c86</xwas:zugelasseneUntersuchungsstelleID>
              <xwas:anschriftID>IDfcfd2538-f074-4848-b443-d15997e42c9e</xwas:anschriftID>
              <xwas:analyseImRahmenDerAkkreditierung>true</xwas:analyseImRahmenDerAkkreditierung>
              <xwas:untersuchungsverfahren>
                <code>1010</code>
              </xwas:untersuchungsverfahren>
              <xwas:untersuchterParameter>
                <code>1021</code>
              </xwas:untersuchterParameter>
              <xwas:bewertungUntersuchungswert>
                <code>1010</code>
              </xwas:bewertungUntersuchungswert>
            </xwas:analyseergebnisParameter>
            <xwas:probennehmerID>IDb05bfc54-c2a9-4ff1-92c8-47b2c4fd9804</xwas:probennehmerID>
            <xwas:probennehmer>
              <xwas:probennehmerID>IDb05bfc54-c2a9-4ff1-92c8-47b2c4fd9804</xwas:probennehmerID>
              <xwas:probennehmer>
                <xwas:natuerlichePerson>
                  <xwas:nameNatuerlichePerson>
                    <xwas:familienname>
                      <xwas:name>Doe</xwas:name>
                    </xwas:familienname>
                    <xwas:vorname>
                      <xwas:name>John</xwas:name>
                    </xwas:vorname>
                  </xwas:nameNatuerlichePerson>
                </xwas:natuerlichePerson>
              </xwas:probennehmer>
            </xwas:probennehmer>
            <xwas:anlassDerUntersuchung>
              <code>1010</code>
            </xwas:anlassDerUntersuchung>
            <xwas:medium>
              <code>1010</code>
            </xwas:medium>
            <xwas:zeitpunktProbennahme>2024-05-27T09:00:00</xwas:zeitpunktProbennahme>
            <xwas:probennahmeverfahren>
              <code>1010</code>
            </xwas:probennahmeverfahren>
            <xwas:kommentarZurProbennahme></xwas:kommentarZurProbennahme>
            <xwas:eingangProbeBeiUntersuchungsstelle>2024-05-27T10:00:00</xwas:eingangProbeBeiUntersuchungsstelle>
            <xwas:beginnAnalytik>2024-05-27T10:01:00</xwas:beginnAnalytik>
            <xwas:abschlussAnalytik>2024-05-27T10:02:00</xwas:abschlussAnalytik>
            <xwas:probenbewertung>
              <code>1010</code>
            </xwas:probenbewertung>
            <xwas:vonProbennehmerVergebeneProbeID>qqq</xwas:vonProbennehmerVergebeneProbeID>
            <xwas:probeID_ausLabor>small pink elephants dancing on my table</xwas:probeID_ausLabor>
          </xwas:probe>
          <xwas:nameProbennahmestelle></xwas:nameProbennahmestelle>
          <xwas:artProbennahmestelle>
            <code>1010</code>
          </xwas:artProbennahmestelle>
          <xwas:mediumAnDerProbennahmestelle>
            <code>1010</code>
          </xwas:mediumAnDerProbennahmestelle>
        </xwas:probennahmestelle>
        <xwas:nameBeauftragteUntersuchungsstelle>
          <code>09010</code>
        </xwas:nameBeauftragteUntersuchungsstelle>
        <xwas:pruefgerichtGemVorgabenAkkredition>true</xwas:pruefgerichtGemVorgabenAkkredition>
        <xwas:titel></xwas:titel>
        <xwas:gesamtbewertung>
          <code>1010</code>
        </xwas:gesamtbewertung>
        <xwas:auffaelligkeiten>ids are duplicated, makes no sense</xwas:auffaelligkeiten>
        <xwas:zeitpunktValidierungPruefbericht>2024-05-28T09:10:00</xwas:zeitpunktValidierungPruefbericht>
        <xwas:fuerValidierungVerantwortlichePerson></xwas:fuerValidierungVerantwortlichePerson>
        <xwas:pruefberichtIDLabor>aaa</xwas:pruefberichtIDLabor>
        <xwas:swVersion>0.1.0</xwas:swVersion>
        <xwas:sprachePruefbericht>
          <code>DE</code>
        </xwas:sprachePruefbericht>
        <xwas:rechtlicherDisclaimer></xwas:rechtlicherDisclaimer>
        <xwas:zeitpunktUebermittlungAnSHAPTH>2024-05-28T09:11:00</xwas:zeitpunktUebermittlungAnSHAPTH>
        <xwas:auftraggeber_Rel>
          <xwas:auftraggeberID>ID4510d774-13a7-414f-82b0-60e8176e5e19</xwas:auftraggeberID>
          <xwas:auftraggeberart>
            <code>1010</code>
          </xwas:auftraggeberart>
          <xwas:auftraggeber>
            <xwas:natuerlichePerson>
              <xwas:nameNatuerlichePerson>
                <xwas:familienname>
                  <xwas:name>Doe</xwas:name>
                </xwas:familienname>
                <xwas:vorname>
                  <xwas:name>John</xwas:name>
                </xwas:vorname>
              </xwas:nameNatuerlichePerson>
            </xwas:natuerlichePerson>
          </xwas:auftraggeber>
        </xwas:auftraggeber_Rel>
        <xwas:zustaemdigeBehoerde_Rel>
          <xwas:id>IDff3e594e-c836-46b8-b44b-6d9c8a6efe47</xwas:id>
          <xwas:zusatz></xwas:zusatz>
          <xwas:behoerdenKennung></xwas:behoerdenKennung>
          <xwas:kommunikation></xwas:kommunikation>
          <xwas:behoerdenIdentifikation></xwas:behoerdenIdentifikation>
          <xwas:behoerdenname></xwas:behoerdenname>
          <xwas:nachgeordneteBehoerde></xwas:nachgeordneteBehoerde>
          <xwas:verwaltungspolitischeZustaendigkeit></xwas:verwaltungspolitischeZustaendigkeit>
          <xwas:anschrift id="IDfcfd2538-f074-4848-b443-d15997e42c9e"/>
          <xwas:anlageNachTrinkwVID>IDcd7df392-08ca-4915-bbd9-c14be7b69d02</xwas:anlageNachTrinkwVID>
          <xwas:probennehmerID>IDb05bfc54-c2a9-4ff1-92c8-47b2c4fd9804</xwas:probennehmerID>
          <xwas:pruefberichtID>IDcd7df392-08ca-4915-bbd9-c14be7b69d02</xwas:pruefberichtID>
          <xwas:laenderkuerzel>
            <code>DEAA</code>
          </xwas:laenderkuerzel>
        </xwas:zustaemdigeBehoerde_Rel>
        <xwas:beauftragteUntersuchungsstelle_Rel>
          <xwas:rechtsform listVersionID="">
            <code></code>
          </xwas:rechtsform>
          <xwas:branche>
            <code></code>
          </xwas:branche>
          <xwas:zweck>
            <code></code>
          </xwas:zweck>
          <xwas:name></xwas:name>
          <xwas:unterorganisation></xwas:unterorganisation>
          <xwas:kommunikation></xwas:kommunikation>
          <xwas:registrierung></xwas:registrierung>
          <xwas:identifikation></xwas:identifikation>
          <xwas:existenzzeitraum></xwas:existenzzeitraum>
          <xwas:anschrift id="fcfd2538-f074-4848-b443-d15997e42c9e"/>
          <xwas:zugelasseneUntersuchungsstelleID>ID14aeb6dd-bc5e-443f-890c-cbdfe6f50c86</xwas:zugelasseneUntersuchungsstelleID>
          <xwas:nameZugelasseneUntersuchungsstelle>
            <code>09010</code>
          </xwas:nameZugelasseneUntersuchungsstelle>
          <xwas:pruefgebieteUntersuchungenPhysChem>true</xwas:pruefgebieteUntersuchungenPhysChem>
          <xwas:pruefgebieteUntersuchungenMikrobio>true</xwas:pruefgebieteUntersuchungenMikrobio>
          <xwas:pruefgebieteUntersuchungenRadionuklide>true</xwas:pruefgebieteUntersuchungenRadionuklide>
          <xwas:akkreditierungsnummer></xwas:akkreditierungsnummer>
          <xwas:kommentarZugelasseneUntersuchungsstelle></xwas:kommentarZugelasseneUntersuchungsstelle>
          <xwas:pruefberichtID>IDcd7df393-08ca-4915-bbd9-c14be7b69d02</xwas:pruefberichtID>
          <xwas:kommentarBeauftragteUntersuchungsstelle></xwas:kommentarBeauftragteUntersuchungsstelle>
        </xwas:beauftragteUntersuchungsstelle_Rel>
        <xwas:ortDerLabortaetigkeiten id="ID46c72370-11b0-41df-b482-4f14b2562744"/>
      </xwas:pruefbericht>
    </xwas:vorgangType>
  </xwas:vorgang>
</vorgang.transportieren.2010>`

describe("simple xml generation via wasm", async () => {
    it("should be able to create and parse quality report xml", async () => {
        const xml = create_quality_report_xml({
            produkt: "SHAPTH CLI",
            test: true,
            nachrichtenkopf_g2g: {
                leser: {
                    name: "Reader",
                    verzeichnisdienst: {
                        code: {
                            code: "",
                        },
                    },
                    kennung: "psw:01003110",
                },
                autor: {
                    kennung: "psw:01003110",
                    name: "Author",
                    verzeichnisdienst: {
                        code: {
                            code: "",
                        },
                    },
                },
                identifikation_nachricht: {
                    nachrichten_uuid: "693c64d6-456f-4d14-abe7-fe9681c74aae",
                    nachrichten_typ: {
                        code: {
                            code: "2010",
                        },
                    },
                    erstellungszeitpunkt: "2024-05-28T09:00:00",
                },
                referenz_uuid: "238b7cc7-6d64-4db8-9c69-779bb65d60b1",
                dvdv_dienstkennung: "s",
            },
            vorgang: {
                identifikation_vorgang: {
                    vorgangs_id: "5e08e073-4e06-438d-9444-1275f6cbf061",
                },
                vorgang_type: {
                    pruefbericht: {
                        pruefbericht_id: "IDcd7df392-08ca-4915-bbd9-c14be7b69d02",
    
                        probennahmestelle: {
                            probennahmestelle_id: "ID14aeb6cd-bc5e-443f-890c-cbdfe6f50c86",
                            objekt_id: "",
                            probe: 
                                {
                                    probe_id: "ID3cd3c929-ee22-4056-a2a0-6c2d1c295c5c",
                                    probennahmestelle_id: "ID14aeb6cd-bc5e-443f-890c-cbdfe6f50c86",
                                    // untersuchungsplan_id: "untersuchungsplanID",
                                    analyseergebnis_parameter_id: "ID753d97bc-4262-45e8-8c1f-cb7b6ab7864a",
                                    analyseergebnis_parameter: {
                                        analyseergebnis_parameter_id: "ID753d97bc-4262-45e8-8c1f-cb7b6ab7864a",
                                        probe_id: "ID3cd3c929-ee22-4056-a2a0-6c2d1c295c5c",
                                        zugelassene_untersuchungsstelle_id: "ID14aeb6cd-bc5e-443f-890c-cbdfe6f50c86",
                                        anschrift_id: "IDfcfd2538-f074-4848-b443-d15997e42c9e",
                                        analyse_im_rahmen_der_akkreditierung: true,
                                        untersuchungsverfahren: {
                                            code: {
                                                code: "1010"
                                            }
                                        },
                                        untersuchter_parameter: {
                                            code: {
                                                code: "1021"
                                            }
                                        },
                                        bewertung_untersuchungswert: {
                                            code: {
                                                code: "1010"
                                            }
                                        },
                                    },
                                    probennehmer_id: "IDb05bfc54-c2a9-4ff1-92c8-47b2c4fd9804",
                                    probennehmer: {
                                        probennehmer_id: "IDb05bfc54-c2a9-4ff1-92c8-47b2c4fd9804",
                                        probennehmer: {
                                            natuerliche_person: {
                                                name_natuerliche_person: {
                                                    vorname: [
                                                        {
                                                            name: "John"
                                                        }
                                                    ],
                                                    familienname: [
                                                        {
                                                            name: "Doe"
                                                        }
                                                    ]
                                                },
                                            },
                                        },
                                    },
                                    anlass_der_untersuchung: {
                                        code: {
                                            code: "1010"
                                        }
                                    },
                                    medium: {
                                        code: {
                                            code: "1010"
                                        }
                                    },
                                    zeitpunkt_probennahme: "2024-05-27T09:00:00",
                                    probennahmeverfahren: {
                                        code: {
                                            code: "1010"
                                        }
                                    },
                                    kommentar_zur_probennahme: "",
                                    eingang_probe_bei_untersuchungsstelle: "2024-05-27T10:00:00",
                                    beginn_analytik: "2024-05-27T10:01:00",
                                    abschluss_analytik: "2024-05-27T10:02:00",
                                    probenbewertung: {
                                        code: {
                                            code: "1010"
                                        }
                                    },
                                    von_probennehmer_vergebene_probe_id: "qqq",
                                    probe_id_aus_labor: "small pink elephants dancing on my table",
                                },
    
                            name_probennahmestelle: "",
                            art_probennahmestelle: {
                                code: {
                                    code: "1010"
                                }
                            },
                            medium_an_der_probennahmestelle: {
                                code: {
                                    code: "1010"
                                }
                            }
                        },
                        name_beauftragte_untersuchungsstelle: {
                            code: {
                                code: "09010"
                            }
                        },
                        pruefgericht_gem_vorgaben_akkredition: true,
                        titel: "",
                        gesamtbewertung: {
                            code: {
                                code: "1010"
                            }
                        },
                        auffaelligkeiten: "ids are duplicated, makes no sense",
                        zeitpunkt_validierung_pruefbericht: "2024-05-28T09:10:00",
                        fuer_validierung_verantwortliche_person: "",
                        pruefbericht_id_labor: "aaa",
                        sw_version: "0.1.0",
                        sprache_pruefbericht: {
                            code: {
                                code: "DE"
                            }
                        },
                        rechtlicher_disclaimer: "",
                        zeitpunkt_uebermittlung_an_shapth: "2024-05-28T09:11:00",
                        auftraggeber_rel: {
                            auftraggeber_id: "ID4510d774-13a7-414f-82b0-60e8176e5e19",
                            auftraggeberart: {
                                code: {
                                    code: "1010"
                                },
                            },
                            auftraggeber: {
                                natuerliche_person: {
                                        name_natuerliche_person: {
                                            vorname: [
                                                {
                                                    name: "John"
                                                }
                                            ],
                                            familienname: [
                                                {
                                                    name: "Doe"
                                                }
                                            ]
                                        }
                                },
                            },
                        },
                        zustaemdige_behoerde_rel: {
                            id: "IDff3e594e-c836-46b8-b44b-6d9c8a6efe47",
                            zusatz: "",
                            behoerden_kennung: "",
                            kommunikation: "",
                            behoerden_identifikation: "",
                            behoerdenname: "",
                            nachgeordnete_behoerde: "",
                            verwaltungspolitische_zustaendigkeit: "",
                            anschrift: {
                                    id: "IDfcfd2538-f074-4848-b443-d15997e42c9e",
                            },
                            anlage_nach_trinkw_vid: "IDcd7df392-08ca-4915-bbd9-c14be7b69d02",
                            probennehmer_id: "IDb05bfc54-c2a9-4ff1-92c8-47b2c4fd9804",
                            pruefbericht_id: "IDcd7df392-08ca-4915-bbd9-c14be7b69d02",
                            laenderkuerzel: {
                                code: {
                                    code: "DEAA"
                                }
                            },
                        
                        },
                        beauftragte_untersuchungsstelle_rel: {
                            rechtsform: {
                                list_version_id: "",
                                code: {
                                    code: "",
                                }
                            },
                            branche: {
                                code: {
                                    code: "",
                                }
                            },
                            zweck: {
                                code: {
                                    code: "",
                                }
                            },
                            name: "",
                            unterorganisation: "",
                            kommunikation: "",
                            registrierung: "",
                            identifikation: "",
                            existenzzeitraum: "",
                            anschrift: {
                                id: "fcfd2538-f074-4848-b443-d15997e42c9e",
                            },
                            zugelassene_untersuchungsstelle_id: "ID14aeb6dd-bc5e-443f-890c-cbdfe6f50c86",
                            name_zugelassene_untersuchungsstelle: {
                                code: {
                                    code: "09010",
                                }
                            },
                            pruefgebiete_untersuchungen_phys_chem: true,
                            pruefgebiete_untersuchungen_mikrobio: true,//Vec<PruefgebieteUntersuchungenMikr
                            pruefgebiete_untersuchungen_radionuklide: true,//Vec<PruefgebieteUntersuchungenRadionuk
                            akkreditierungsnummer: "", //Akkreditierungsn
                            kommentar_zugelassene_untersuchungsstelle: "", //KommentarZugelasseneUntersuchungss
                            pruefbericht_id: "IDcd7df393-08ca-4915-bbd9-c14be7b69d02",
                            kommentar_beauftragte_untersuchungsstelle: ""
                        },
                        ort_der_labortaetigkeiten: {
                            _id: "ID46c72370-11b0-41df-b482-4f14b2562744"
                        }
    
                    }
                },
            },
        });
        expect(xml).toEqual(GH_XML);
        const parsed = parse_quality_report_xml(xml);
        console.log(xml);
    //     expect(parsed.produkt).toEqual("SHAPTH");
    //     expect(parsed.test).toBeTruthy();
    //     expect(parsed.nachrichtenkopf_g2g.autor.name).toEqual("Author");
    //     expect(parsed.nachrichtenkopf_g2g.autor.kennung).toEqual(
    //         "psw:01003110"
    //     );
    //     expect(parsed.nachrichtenkopf_g2g.leser.name).toEqual("Reader");
    //     expect(parsed.nachrichtenkopf_g2g.leser.kennung).toEqual(
    //         "psw:11113110"
    //     );
    });
});
