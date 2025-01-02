import { describe, it, expect } from "vitest";
import {
  detect_version,
} from "../pkg/xoev_xwasser";
import fs from 'fs'
import path from 'path'
const __dirname = import.meta.dirname;

describe("version of a xml can be detected via wasm", async () => {
  it("should be able to detect xml version", async () => {
    const source = fs.readFileSync(path.resolve(__dirname, './quality_report_minimal.xml'), 'utf-8');
    const version = detect_version(source.slice(0, 1024));
    expect(version).to.equal("080");
  });
});