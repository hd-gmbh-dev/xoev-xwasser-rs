import { describe, it, expect } from "vitest";
import {
  AdministrationQuittung0020,
  create_administration_quittung_0020,
  parse_administration_quittung_0020,
} from "../pkg/xoev_xwasser";
import administration_receipt from "./administration_receipt.json";
import fs from "fs";
import path from "path";
const __dirname = import.meta.dirname;
import xmlvalidate, { XmlValidatorError } from "@raxb/validate-wasm";
const xsdBundle = fs.readFileSync(path.resolve(__dirname, '../pkg/xwasser-v052.xsdb.bin')).buffer;

describe("administration receipt xml generation via wasm", async () => {
  const { XmlValidator } = await xmlvalidate();
  const validator = new XmlValidator(new Uint8Array(xsdBundle));
  validator.init((err: string) => {
    console.error(err);
  });

  it("should be able to create administration receipt xml", async () => {
    const xml = create_administration_quittung_0020(
      administration_receipt as any as AdministrationQuittung0020,
    ).replace(
      "https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_5_2/ xwasser.xsd",
      "https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_5_2/ ../schemas/V0_5_2/xwasser.xsd",
    );
    const expected_xml = fs.readFileSync(
      path.resolve(__dirname, "./administration_receipt_test_result.xml"),
      "utf-8",
    );
    expect(xml).to.equal(expected_xml);
    validator.validateXml(xml, (err: XmlValidatorError) => {
      console.log({ level: err.level, line: err.line, message: err.message });
    });
  });

  it("should be able to parse administration receipt xml", async () => {
    const source = fs.readFileSync(
      path.resolve(__dirname, "./administration_receipt.xml"),
      "utf-8",
    );
    const obj = parse_administration_quittung_0020(source);
    expect(obj.identifikation_vorgang.vorgangs_id).to.equal(
      "5e08e073-4e06-438d-9444-1275f6cbf061",
    );
    expect(obj.quittung.aktueller_status_technisch.code).toEqual("200");
  });
});
