import { describe, it, expect } from "vitest";
import {
  VorgangTransportieren2010,
  create_vorgang_transportieren_2010,
  parse_vorgang_transportieren_2010,
  PruefberichtType
} from "../pkg/xoev_xwasser";
import quality_report_minimal from './quality_report_minimal.json'
import fs from 'fs'
import path from 'path'
const __dirname = import.meta.dirname;
import xmlvalidate, { XmlValidatorError } from '@raxb/validate-wasm'
const xsdBundle = fs.readFileSync(path.resolve(__dirname, '../pkg/xwasser-v051.xsdb.bin')).buffer;

describe("minimal quality report xml generation via wasm", async () => {
  const { XmlValidator } = await xmlvalidate();
  const validator = new XmlValidator(new Uint8Array(xsdBundle));
  validator.init((err: string) => {
    console.error(err);
  });

  it("should be able to create minimal quality report xml", async () => {
    const xml = create_vorgang_transportieren_2010(quality_report_minimal as any as VorgangTransportieren2010)
      .replace("https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_5_1 xwasser.xsd", "https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_5_1 ../schemas/V0_5_1/xwasser.xsd");
    const expected_xml = fs.readFileSync(path.resolve(__dirname, './quality_report_minimal_test_result.xml'), 'utf-8');
    expect(xml).to.equal(expected_xml);
    validator.validateXml(xml, (err: XmlValidatorError) => {
      console.log({ level: err.level, line: err.line, message: err.message });
    })
  });

  it("should be able to parse minimal quality report xml", async () => {
    const source = fs.readFileSync(path.resolve(__dirname, './quality_report_minimal.xml'), 'utf-8');
    const obj = parse_vorgang_transportieren_2010(source)
    expect(obj.vorgang.vorgang_type.t).to.equal("Pruefbericht");
    const p = (obj.vorgang.vorgang_type as { t: "Pruefbericht"; c: PruefberichtType }).c;
    expect(p.id).toEqual("ID5e08e073-4e06-438d-9444-1275f6cbf061");
  });
});