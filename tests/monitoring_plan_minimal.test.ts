import { describe, it, expect } from "vitest";
import {
  DokumentType,
  UntersuchungsplanType,
  VorgangTransportieren2010,
  create_vorgang_transportieren_2010,
  parse_vorgang_transportieren_2010,
} from "../pkg/xoev_xwasser";
import monitoring_plan_minimal from "./monitoring_plan_minimal.json";
import fs from "fs";
import path from "path";
const __dirname = import.meta.dirname;
import xmlvalidate, { XmlValidatorError } from "@raxb/validate-wasm";
const xsdBundle = fs.readFileSync(
  path.resolve(__dirname, "../pkg/xwasser-v080.xsdb.bin"),
).buffer;

describe("minimal monitoring plan xml generation via wasm", async () => {
  const { XmlValidator } = await xmlvalidate();
  const validator = new XmlValidator(new Uint8Array(xsdBundle));
  validator.init((err: string) => {
    console.error(err);
  });

  it("should be able to create minimal monitoring plan xml", async () => {
    const xml = create_vorgang_transportieren_2010(
      monitoring_plan_minimal as any as VorgangTransportieren2010,
    ).replace(
      "https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_8_0/ xwasser.xsd",
      "https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_8_0/ ../schemas/V0_8_0/xwasser.xsd",
    );
    const expected_xml = fs.readFileSync(
      path.resolve(__dirname, "./monitoring_plan_minimal_test_result.xml"),
      "utf-8",
    );
    expect(xml).to.equal(expected_xml);
    validator.validateXml(xml, (err: XmlValidatorError) => {
      console.log({ level: err.level, line: err.line, message: err.message });
    });
  });

  it("should be able to parse minimal monitoring plan xml", async () => {
    const source = fs.readFileSync(
      path.resolve(__dirname, "./monitoring_plan_minimal.xml"),
      "utf-8",
    );
    const obj = parse_vorgang_transportieren_2010(source);
    expect(obj.vorgang.vorgang_type.t).to.equal("Untersuchungsplan");
    const p = (obj.vorgang.vorgang_type as { t: "Untersuchungsplan"; c: UntersuchungsplanType })
      .c;
    expect(p.id).toEqual("untersuchungsplan1");
  });
});
