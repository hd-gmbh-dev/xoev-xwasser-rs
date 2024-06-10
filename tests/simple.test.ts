import { describe, it, expect } from "vitest";
import { create_quality_report_xml, parse_quality_report_xml } from "../pkg";

const TEST_XML = `<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<xwas:vorgang.transportieren.2010 xmlns:xwas="xwasser" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="xwasser xwasser.xsd" xmlns:xwas="xwasser" produkt="SHAPTH" produkthersteller="H &amp; D GmbH" produktversion="H &amp; D GmbH" standard="XWasser" test="true" version="0.2.0">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>6161D5A2-EC09-412E-A715-9950607BAD68</nachrichtenUUID>
      <nachrichtentyp listURI="urn:xoev-de:xwasser:codeliste:nachrichtentyp" listVersionID="1">
        <code listURI="" listVersionID=""></code>
      </nachrichtentyp>
      <erstellungszeitpunkt>2024-05-28T09:00:00</erstellungszeitpunkt>
    </identifikation.nachricht>
    <leser>
      <verzeichnisdienst listVersionID="">
        <code listURI="" listVersionID=""></code>
      </verzeichnisdienst>
      <kennung>psw:11113110</kennung>
      <name>Reader</name>
    </leser>
    <autor>
      <verzeichnisdienst listVersionID="">
        <code listURI="" listVersionID=""></code>
      </verzeichnisdienst>
      <kennung>psw:01003110</kennung>
      <name>Author</name>
    </autor>
  </nachrichtenkopf.g2g>
  <xwas:vorgang xmlns:xwas="xwasser">
    <xwas:identifikationVorgang xmlns:xwas="xwasser">
      <xwas:vorgangsID>ID1</xwas:vorgangsID>
    </xwas:identifikationVorgang>
    <xwas:vorgangType xmlns:xwas="xwasser">
      <xwas:pruefbericht xmlns:xwas="xwasser">
        <xwas:pruefberichtID>DUMMY</xwas:pruefberichtID>
        <xwas:nameBeauftragteUntersuchungsstelle>DUMMY</xwas:nameBeauftragteUntersuchungsstelle>
        <xwas:pruefgerichtGemVorgabenAkkredition>DUMMY</xwas:pruefgerichtGemVorgabenAkkredition>
        <xwas:titel>DUMMY</xwas:titel>
        <xwas:gesamtbewertung>DUMMY</xwas:gesamtbewertung>
        <xwas:auffaelligkeiten>DUMMY</xwas:auffaelligkeiten>
        <xwas:zeitpunktValidierungPruefbericht>DUMMY</xwas:zeitpunktValidierungPruefbericht>
        <xwas:fuerValidierungVerantwortlichePerson>DUMMY</xwas:fuerValidierungVerantwortlichePerson>
        <xwas:pruefberichtIDLabor>DUMMY</xwas:pruefberichtIDLabor>
        <xwas:swVersion>DUMMY</xwas:swVersion>
        <xwas:sprachePruefbericht>DUMMY</xwas:sprachePruefbericht>
        <xwas:rechtlicherDisclaimer>DUMMY</xwas:rechtlicherDisclaimer>
        <xwas:zeitpunktUebermittlungAnSHAPTH>DUMMY</xwas:zeitpunktUebermittlungAnSHAPTH>
        <xwas:auftraggeber_Rel>DUMMY</xwas:auftraggeber_Rel>
      </xwas:pruefbericht>
    </xwas:vorgangType>
  </xwas:vorgang>
</xwas:vorgang.transportieren.2010>`;

describe("simple xml generation via wasm", async () => {
    it("should be able to create and parse quality report xml", async () => {
        const xml = create_quality_report_xml({
            produkt: "SHAPTH",
            test: true,
            nachrichtenkopf_g2g: {
                leser: {
                    kennung: "psw:11113110",
                    name: "Reader",
                    verzeichnisdienst: {
                        code: {
                            list_uri: "",
                            list_version_id: "",
                            code: "",
                        },
                    },
                },
                autor: {
                    kennung: "psw:01003110",
                    name: "Author",
                    verzeichnisdienst: {
                        code: {
                            list_uri: "",
                            list_version_id: "",
                            code: "",
                        },
                    },
                },
                identifikation_nachricht: {
                    erstellungszeitpunkt: "2024-05-28T09:00:00",
                    nachrichten_typ: {
                        code: {
                            list_uri: "",
                            list_version_id: "",
                            code: "",
                        },
                    },
                    nachrichten_uuid: "6161D5A2-EC09-412E-A715-9950607BAD68",
                },
            },
            vorgang: {
                identifikation_vorgang: {
                    vorgangs_id: "ID1",
                },
                vorgang_type: {
                    pruefbericht: {
                        auffaelligkeiten: "DUMMY",
                        pruefbericht_id: "DUMMY",
                        probennahmestelle: [],
                        name_beauftragte_untersuchungsstelle: "DUMMY",
                        pruefgericht_gem_vorgaben_akkredition: "DUMMY",
                        titel: "DUMMY",
                        gesamtbewertung: "DUMMY",
                        zeitpunkt_validierung_pruefbericht: "DUMMY",
                        fuer_validierung_verantwortliche_person: "DUMMY",
                        pruefbericht_id_labor: "DUMMY",
                        sw_version: "DUMMY",
                        sprache_pruefbericht: "DUMMY",
                        rechtlicher_disclaimer: "DUMMY",
                        zeitpunkt_uebermittlung_an_shapth: "DUMMY",
                        auftraggeber_rel: "DUMMY",
                    },
                },
            },
        });
        expect(xml).toEqual(TEST_XML);
        const parsed = parse_quality_report_xml(xml);
        expect(parsed.produkt).toEqual("SHAPTH");
        expect(parsed.test).toBeTruthy();
        expect(parsed.nachrichtenkopf_g2g.autor.name).toEqual("Author");
        expect(parsed.nachrichtenkopf_g2g.autor.kennung).toEqual(
            "psw:01003110"
        );
        expect(parsed.nachrichtenkopf_g2g.leser.name).toEqual("Reader");
        expect(parsed.nachrichtenkopf_g2g.leser.kennung).toEqual(
            "psw:11113110"
        );
    });
});
