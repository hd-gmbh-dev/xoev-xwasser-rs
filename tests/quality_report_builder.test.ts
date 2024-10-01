import { describe, it, expect } from "vitest";
import {
    identifikation_nachricht,
    IdentifikationNachricht
} from "../pkg/xoev_xwasser";

import fs from 'fs'
import path from 'path'
const __dirname = import.meta.dirname;

describe("quality report xml generation via builder functions in wasm", async () => {
  it("should be able to create quality report xml via builder", async () => {
    const v: IdentifikationNachricht = identifikation_nachricht("2010");
    expect(v.nachrichten_typ?.code.code).to.eq('2010');
    expect(v.erstellungszeitpunkt).toMatch(/^[0-9]{4}\-[0-9]{2}\-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}$/)
    expect(v.nachrichten_uuid).toMatch(/^[a-fA-F0-9]{8}\-[a-fA-F0-9]{4}\-[a-fA-F0-9]{4}\-[a-fA-F0-9]{4}\-[a-fA-F0-9]{12}$/)
  });
});