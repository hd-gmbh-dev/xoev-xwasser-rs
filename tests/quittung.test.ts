import { describe, it, expect } from "vitest";
import { create_administration_quittung_xml, parse_administration_quittung_xml } from "../pkg";

const TEST_XML = `<administration.quittung.0020 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
  xsi:schemaLocation="xwasser ../xwasser.xsd"
  xmlns="xwasser" produkt="SHAPTH CLI" produkthersteller="H &amp; D GmbH"
  produktversion="H &amp; D GmbH" standard="XWasser" test="true" version="0.2.0">
  <nachrichtenkopf.g2g xmlns="">
    <identifikation.nachricht>
      <nachrichtenUUID>693c64d6-456f-4d14-abe7-fe9681c74aae</nachrichtenUUID>
      <nachrichtentyp>
        <code>2010</code>
      </nachrichtentyp>
      <erstellungszeitpunkt>2024-05-28T09:00:00</erstellungszeitpunkt>
    </dentifikation.nachricht>
    <leser>
      <verzeichnisdienst listVersionID="">
        <code>222</code>
      </verzeichnisdienst>
      <kennung>333</kennung>
      <name>name</name>
    </leser>
    <autor>
      <verzeichnisdienst listVersionID="1">
        <code>444</code>
      </verzeichnisdienst>
      <kennung>psw:01003110</kennung>
      <name>Author</name>
    </autor>
  </nachrichtenkopf.g2g>
  <identifikationVorgang>
    <vorgangsID>5e08e073-4e06-438d-9444-1275f6cbf061</vorgangsID>
  </identifikationVorgang>
  <quittung>
    <aktuellerStatusTechnisch listURI="" listVersionID="">
      <code xmlns="">201</code>
    </aktuellerStatusTechnisch>
  </quittung>
</administration.quittung.0020>`;

describe("quittung xml generation via wasm", async () => {
  it("should be able to create and parse quittung xml", async () => {
    const xml = create_administration_quittung_xml({
      produkt: "SHAPTH",
      test: "true",
      nachrichtenkopf_g2g: {
        leser: {
          kennung: "psw:11113110",
          name: "Reader",
          verzeichnisdienst: {
            code: {
              code: "222"
            }
          }
        },
        autor: {
          kennung: "psw:01003110",
          name: "Author",
          verzeichnisdienst: {
            code: {
              list_uri: "",
              list_version_id: "",
              code: "444",
            },
          },
        },
      },
      identifikation_vorgang: {
        vorgangs_id: "5e08e073-4e06-438d-9444-1275f6cbf061",
      },
      quittung: {
        akuteller_status_technisch: {
          list_uri: "",
          list_version_id: "",
          code: {
            xmlns: "",
            code: "201",
          }
        }

      }
    })
  expect(xml).toEqual(TEST_XML);
  const parsed = parse_administration_quittung_xml(xml);
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

