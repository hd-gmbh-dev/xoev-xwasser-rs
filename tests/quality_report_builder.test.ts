import { describe, it, expect } from "vitest";
import {
    identifikation_nachricht,
    IdentifikationNachricht,
    anschrift_type,
    natuerliche_person_type,
    
} from "../pkg";
import {
    vorname,
    familienname,
} from "../pkg/xoev-xwasser-utils";

import fs from 'fs'
import path from 'path'
const __dirname = import.meta.dirname;

describe("quality report xml generation via builder functions in wasm", async () => {
  it("should be able to create quality report xml via builder", async () => {
    const v: IdentifikationNachricht = identifikation_nachricht("VorgangTransportieren2010");
    expect(v.nachrichten_typ?.code.code).to.eq('2010');
    expect(v.erstellungszeitpunkt).toMatch(/^[0-9]{4}\-[0-9]{2}\-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}$/)
    expect(v.nachrichten_uuid).toMatch(/^[a-fA-F0-9]{8}\-[a-fA-F0-9]{4}\-[a-fA-F0-9]{4}\-[a-fA-F0-9]{4}\-[a-fA-F0-9]{12}$/)

    let a = anschrift_type(
      "teststrasse",
      "testhausnummer",
      "testpostleitzahl",
      "testort",
    );

    expect(a.strasse).to.eq("teststrasse");
    expect(a.hausnummer).to.eq("testhausnummer");
    expect(a.postleitzahl).to.eq("testpostleitzahl");
    expect(a.ort).to.eq("testort");
    expect(a.id).toMatch(/^anschrift-[a-zA-Z0-9]{12}$/)

    let p = natuerliche_person_type("Sepp", "Meier");
    expect(vorname(p)).eq('Sepp')
    expect(familienname(p)).eq('Meier')
    
  });
});