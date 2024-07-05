import { describe, it, expect } from "vitest";
import { Gueltigkeit, create_quality_report_xml, parse_quality_report_xml } from "../pkg/xwasser_rs";

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

function code(name: string, code: string): any {
  return {
    code: {
        code: "2010",
    },
    name: [name],
  }
}

function geburt(): any {
  return {
    datum: [
      "asdf"
    ],
    zusatz: [
      "asokdfm"
    ],
    geburtsort: [anschrift()],
  }
}

function alternativeRepraesentation(): any {
  return {
    repraesentation: "r1",
    algorithmus: [
      "bubble_sort"
    ],
    hinweis: [
      "be careful"
    ],
  }
}

function allgemeinerNameType(name: string): any {
  return {
    name: [
      name
    ],
    nicht_vorhanden: false,
    namensart: {
        code: "1010"
    },
    alternative_repraesentation: [alternativeRepraesentation()],
  }
}

function verwaltungspolitischeKodierung(): any {
  return {
    kreis: [code("name","1111")],
    bezirk: [code("name","1111")],
    bundesland: [code("name","1111")],
    gemeindeschluessel: [code("name","1111")],
    regionalschluessel: [code("name","1111")],
    nation: [code("name","1111")]
  }
}


function anschrift(): any {
  return  {
    id: "IDfcfd2538-f074-4848-b443-d15997e42c9e",
    strassenschluessel: code("name","1234"),
    strasse: "Mustermann Strasse",
    hausnummer: "1",
    postfach: "1234",
    postleitzahl: "123456",
    ort: "Musterhausen",
    ortsteil: [
      "Neuhausen"
    ],
    ort_frueherer_gemeindename: [
      "wurstling"
    ],
    wohnungsgeber: [
      ""
    ],
    zusatz: [
      ""
    ],
    typ: {
      code: "1234",
      name: ""
    },
    staat: [code("absurdistan","1111")],
    verwaltungspolitische_kodierung: verwaltungspolitischeKodierung(),
  }
}

function gueltigkeit(beginn: string, ende: string, zusatz: string): any {
  return {                                                    
    beginn: beginn,
    ende: ende,
    zusatz: zusatz,
  }
}

function zeitraumType(beginn: string[], ende: string[], zusatz: string[]): any {
  return {                                                    
    beginn: beginn,
    ende: ende,
    zusatz: zusatz,
  }
}

function identifikationType() {
  return {
    id: ["238b7cc7-6d64-4db8-9c69-779bb65d60b1"],
    beschreibung: ["bal bla bla"],
    gueltigkeit: [gueltigkeit("9:00","19:00","iieudjod")]

  }
}

function kommunikationType(kanal: string[], kennung: string[], ist_dienstlich: boolean[], zusatz: string[]): any {
  return {
    kanal: code(kanal[0],kanal[1]),
    kennung: kennung,
    ist_dienstlich: ist_dienstlich,
    zusatz:zusatz,
  }
}

function spracheType(sprache: string, zusatz: string[]): any {
  return {
    sprache: sprache, 
    zusatz: zusatz
  }
}

function vertreterBevollmaechtigterType(): any {
  return {
    id: "1234",
    art_vertreter: code("vertretungsart","1233")
  }
}

function natuerlichePerson(): any {
  return {
    auskunftssperre: {
      grund: {
        code: "69"
      },
      gueltigkeit: [gueltigkeit("9:00","19:00","iieudjod")]

    },
    name_natuerliche_person: {
        titel: [
            "Prof",
            "Dr"
        ],

        anrede: "Herr",
        namenssuffix: [
          "Sir",
          "Jr"
        ],

        familienname: [allgemeinerNameType("Doe")],
        ehename: [allgemeinerNameType("Meier")],
        lebenspartnerschaftsname: [allgemeinerNameType("Mueller")],
        geburtsname: [allgemeinerNameType("Schroeder")],
        frueherer_familienname: [allgemeinerNameType("Heinz")],
        vorname: [allgemeinerNameType("Alexander")],
        rufname: [allgemeinerNameType("Alex")],
        frueherer_vorname: [allgemeinerNameType("Nadine")],
        alternative_repraesentation: [alternativeRepraesentation()],
        ordensname: [allgemeinerNameType("Andechs")],
        kuenstlername: [allgemeinerNameType("sikis")],
        weiterer_name: [allgemeinerNameType("superman")],
    },
    familienstand: code("familienstand","1111"),
    geburt: [geburt()],
    doktorgrad: [
      {
        bezeichnung: "DR."
      }
    ],
    staatsangehoerigkeit: code("staatsangehoerigkeit","1111"),
    ausweisdokument: code("ausweisdokument","1234"),
    anschrift: anschrift(),
    geschlecht: code("geschlecht","1111"),
    identifikationsnummer: [identifikationType()],
    kommunikation: [kommunikationType(["email", "123"], ["text"], [true], ["zusatz"])],
    muttersprache: spracheType("Deutsch", ["zusatz"]),
    fremdsprache: spracheType("Englisch", ["zusatz"]),
    vertreter_bevollmaechtigter: vertreterBevollmaechtigterType()
  }
}

function nameOrganisationType(org_name: string[], kurzbezeichnung: string[], gueltigkeit: Gueltigkeit[]): any {
  return {
    name: org_name,
    kurzbezeichnung: kurzbezeichnung,
    gueltigkeit: gueltigkeit,
  }
}

function behoerdeType():any {
  return {
    id: ["ID753d97bc-4262-45e8-8c1f-cb7b6ab7864a"],
    code: [code("behoerde","1234")],
    zusatz: ["zusatz"],
    behoerdenkennung: [{
      kennung: [code("behoerdenkennung","1234")],
      praefix: [code("praefixtype","1234")]
    }],
    kommunikation: [kommunikationType(["email", "123"], ["text"], [true], ["zusatz"])],
    behordenidentifikation: [identifikationType()],
    behordenname: [nameOrganisationType(["behorde 1"], ["does stuff"], [gueltigkeit("9","12","")])],
    nachgeordnete_behoerde: [],
    verwaltungspolitische_zustaendigkeit: [verwaltungspolitischeKodierung()],
    anschrift: [anschrift()],
    organisationsstruktur: [
      {
        name: "org1",
        hierarchieebene: [1],
        hierarchiename: ["First level"]
      }
    ]
  }
}

function registrierungType():any {
  return {
    id: ["ID753d97bc-4262-45e8-8c1f-cb7b6ab7864a"],
    registertyp: [code("registertyp","1234")],
    registrierende_behoerde: [behoerdeType()],
    gueltigkeit: [gueltigkeit("9:00","19:00","iieudjod")]
  }
}

function organisationType() {
  return {
    rechtsform: code("rechtsform", "0815"),
    branche: code("branche", "666"),
    zweck: code("zweck", "666"),
    name: nameOrganisationType(["abc corp"], ["produces stuff"], [gueltigkeit("9","12","")]),
    unterorganisation: [], // hier kann man noch eine weitere organisation eintragen
    kommunikation: [kommunikationType(["brief", "123"], ["text"], [true], ["zusatz"])],
    registrierung: [registrierungType()],
    identifikation: [identifikationType()],
    existenzzeitraum: [zeitraumType(["9"],["19"],["zusatz"])],
    anschrift: [anschrift()]
  }
}

function analyseergebnisParameter(): any {
  return {
    analyseergebnis_parameter_id: "ID753d97bc-4262-45e8-8c1f-cb7b6ab7864a",
    probe_id: "ID3cd3c929-ee22-4056-a2a0-6c2d1c295c5c",
    zugelassene_untersuchungsstelle_id: "ID14aeb6cd-bc5e-443f-890c-cbdfe6f50c86",
    anschrift_id: "IDfcfd2538-f074-4848-b443-d15997e42c9e",
    analyse_im_rahmen_der_akkreditierung: true,
    untersuchungsverfahren: code("untersuchungsverfahren","9999"),
    untersuchter_parameter: code("untersuchter_parameter","1010"),
    bewertung_untersuchungswert: code("bewertung_untersuchungswert","9999"),
  }
}


function zustaendigeBehoerde():any {
  return {
    behoerde: behoerdeType(),
    anlage_nach_trinkw_vid: "IDfcfd2538-f074-4848-b443-d15997e42c9e",
    probennehmer_id: ["1234"],
    laenderkuerzel: code("laenderkuerzel","DEAA"),
    kommentar: ["kommentar"]
  }
}

function probennehmer():any {
  return {
    probennehmer_id: "IDb05bfc54-c2a9-4ff1-92c8-47b2c4fd9804",
    probennehmer: {
      // alle möglichen Typen
      // natuerliche_person: natuerlichePerson(),
      // organisation: organisationType(),
      zustaendige_behoerde: zustaendigeBehoerde(),
    },
    fremdsystem_id_probennehmer: "fremdsystemid",
    kommentar: ["kommentar"],
  }
}

function probe(): any {
  return {
    probe_id: "ID3cd3c929-ee22-4056-a2a0-6c2d1c295c5c",
    analyseergebnis_parameter: analyseergebnisParameter(),
    probennehmer: probennehmer(),
    anlass_der_untersuchung: code("name","1010"),
    probennahmestelle_id: "ID14aeb6cd-bc5e-443f-890c-cbdfe6f50c86",
    analyseergebnis_parameter_id: "ID753d97bc-4262-45e8-8c1f-cb7b6ab7864a",
    medium: code("medium","1010"),
    ergaenzung_zum_medium: ["Wasser"],
    zeitpunkt_probennahme: "2024-05-27T09:00:00",
    probennahmeverfahren: code("probennahmeverfahren","1010"),
    probenentnahmegeraet: code("probenentnahmegeraet","1234"),
    probengefaess: code("probengefaess","1234"),
    ergaenzende_informationen_zu_probenentnahmegeraet: [""],
    desinfektion_probenentnahmegeraet_durchgefuehrt: [true],
    konservierung_aufbereitung_desinfektion_probe: [code("konservierung_aufbereitung_desinfektion_probe","1234")],
    kommentar_zur_probennahme: "",
    informationen_zum_probentransport: ["Lieferwagen"],
    eingang_probe_bei_untersuchungsstelle: "2024-05-27T10:00:00",
    beginn_analytik: "2024-05-27T10:01:00",
    abschluss_analytik: "2024-05-27T10:02:00",
    probenbewertung: code("probenbewertung","1010"),
    berichtspflichtig: [true],
    von_probennehmer_vergebene_probe_id: "123456789",
    probe_id_aus_labor: "1233",
    anhang: ["Anhang"],
    kommentar: ["Kommentar"],
  }
}

function probennahmestelle() {
  return {
    probennahmestelle_id: "ID14aeb6cd-bc5e-443f-890c-cbdfe6f50c86",
    objekt_id: "",
    probe: probe(),
    terminplan_id: ["terminplan id"],
    name_probennahmestelle: "wasserloch um die ecke",
    art_probennahmestelle: code("wasserloch","123455"),
    stockwerk_probennahmestelle: [1],
    medium_an_der_probennahmestelle: code("medium probennahmestelle","1235"),
    desinfektion_und_aufbereitung_des_wassers: code("desinfektionAufbereitungDesWassers","1234"),
    alt_id: ["altid2"],
    kommentar: ["kommentar zur proebennahmestelle"]
  }
}


function auftraggeber():any {
  return {
    auftraggeber_id: "ID4510d774-13a7-414f-82b0-60e8176e5e19",
    auftraggeberart: code("name","1010"),
    auftraggeber: {
      natuerliche_person: natuerlichePerson(),
      },
  }
}

function beauftragteUntersuchungsstelle(): any {
  return {
    zugelassene_untersuchungsstelle: {
      organisation: organisationType(),
      zugelassene_untersuchungsstelle_id: "ID14aeb6dd-bc5e-443f-890c-cbdfe6f50c86",
      name_zugelassene_untersuchungsstelle: code("name","09010"),
      pruefgebiete_untersuchungen_phys_chem: [true],
      pruefgebiete_untersuchungen_mikrobio: [true],
      pruefgebiete_untersuchungen_radionuklide: [true],
      akkreditierungsnummer: [""],
      kommentar_beauftragte_untersuchungsstelle: [""],
      kommentar_zugelassene_untersuchungsstelle: [""],

    },
    kommentar_beauftragte_untersuchungsstelle: [""],
  }
}

function erweiterung():any {
  return {
    feld: [
      {
        name: ["feldname"],
        beschreibung: ["feldbeschreibung"],
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
    xml: [],

  }

}

function pruefberichtType():any {
  return {
    pruefbericht_uuid: "IDcd7df392-08ca-4915-bbd9-c14be7b69d02",
    untersuchungsplan_id: ["IDcd7df392-08ca-4915-bbd9-c14be7b69d02"],
    probennahmestelle: probennahmestelle(),
    name_beauftragte_untersuchungsstelle: code("name","1010"),
    pruefbericht_enthaelt_teilergebnisse: [true],
    pruefgericht_gem_vorgaben_akkredition: true,
    titel: "",
    gesamtbewertung: code("name","1010"),
    auffaelligkeiten: ["ids are duplicated, makes no sense"],
    zeitpunkt_validierung_pruefbericht: "2024-05-28T09:10:00",
    fuer_validierung_verantwortliche_person: natuerlichePerson(),
    freigabe_uebermittlung_betreiber: [true],
    pruefbericht_id_labor: "aaa",
    sw_version: code("sw_version", "1234"),
    sprache_pruefbericht: code("name","DE"),
    rechtlicher_disclaimer: "",
    zeitpunkt_uebermittlung_an_shapth: "2024-05-28T09:11:00",
    kommentar: ["kommentar"],
    auftraggeber: auftraggeber(),
    zustaendige_behoerde: zustaendigeBehoerde(),
    beauftragte_untersuchungsstelle: beauftragteUntersuchungsstelle(),
    ort_der_labortaetigkeiten: anschrift(),
    anhang: ["anhang"],
    erweiterung: [erweiterung()]
  }
}

function vorgang():any {
  return {
    identifikation_vorgang: {
        vorgangs_id: "5e08e073-4e06-438d-9444-1275f6cbf061",
    },
    vorgang_type: {
        pruefbericht: pruefberichtType(),
    },
  }
}

function nachrichtenkopf():any {
  return {
    leser: createHeadInfo("psw:11113110", "Reader"),
    autor: createHeadInfo("psw:01003110", "Author"),
    identifikation_nachricht: {
        nachrichten_uuid: "693c64d6-456f-4d14-abe7-fe9681c74aae",
        nachrichten_typ: code("name","2010"),
        erstellungszeitpunkt: "2024-05-28T09:00:00",
    },
    referenz_uuid: "238b7cc7-6d64-4db8-9c69-779bb65d60b1",
    dvdv_dienstkennung: "s",
  }
}

function qualityReport():any {
  return {
    produkt: "SHAPTH CLI",
    test: true,
    nachrichtenkopf_g2g: nachrichtenkopf(),
    vorgang: vorgang(),
  }
}

describe("simple xml generation via wasm", async () => {
  it("should be able to create and parse quality report xml", async () => {
    const xml = create_quality_report_xml(qualityReport());
    console.log(xml);
  });
});