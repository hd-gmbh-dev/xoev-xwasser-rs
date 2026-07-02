import { describe, it, expect } from "vitest";
import {
  // Quick Start / create / parse
  create_vorgang_transportieren_2010,
  parse_vorgang_transportieren_2010,
  schema,
  local_schema,
  xmlns,
  version,
  create_administration_quittung_0020,
  parse_administration_quittung_0020,
  // Transport layer
  identifikation_nachricht,
  nachrichtenkopf_g2g,
  identifikation_vorgang,
  // Address
  anschrift_type,
  // Persons & organizations
  natuerliche_person_type,
  allgemeiner_name_type,
  name_organisation_type,
  // Authorities
  behoerde_type,
  autor,
  leser,
  zustaendige_behoerde_type,
  zugelassene_untersuchungsstelle_type,
  beauftragte_untersuchungsstelle_type,
  betreiber_type,
  // Pruefbericht
  pruefbericht_type,
  pruefbericht_signature_template,
  probe_type,
  probennahmestelle_type,
  probennehmer_type,
  parameterangaben_type,
  analyseergebnis_parameter_type,
  kommentar_type,
  zeitraum_type,
  aenderungshistorie_type,
  // Monitoring plans etc
  untersuchungsplan_type,
  terminplan_type,
  wasserversorgungsgebiet_type,
  objekt_type,
  anlage_nach_trinkw_v_type,
  organisation_type,
  identifikation_type,
  quality_and_monitoring_type,
  derogation_type,
  derogation_remedial_action_type,
  exceedance_type,
  exceedance_cause_and_remedial_action_type,
  incident_type,
  incident_cause_and_remedial_action_type,
  // Types
  VorgangTransportieren2010,
  AdministrationQuittung0020,
  PruefberichtType,
  AnschriftType,
  ProbennahmestelleType,
  IdentifikationNachricht,
  UntersuchungsstelleDetails,
  // Code types
  CodeGesamtbewertungType,
  // Misc
  detect_version,
} from "../pkg";
import { vorname, familienname } from "../pkg/xoev-xwasser-utils";

import fs from "fs";
import path from "path";

const __dirname = import.meta.dirname;

// ---------------------------------------------------------------------------
// README tests — verify each code example in README.npm.md actually works
// ---------------------------------------------------------------------------

describe("Quick Start", () => {
  it("should create XML from a JSON object and parse it back", async () => {
    // Load one of the test JSON fixtures as a realistic payload
    const qualityReport = JSON.parse(
      fs.readFileSync(
        path.resolve(__dirname, "./quality_report_minimal.json"),
        "utf-8"
      )
    );

    // Create XML (as shown in README)
    const xml = create_vorgang_transportieren_2010(
      qualityReport as unknown as VorgangTransportieren2010
    ).replace(schema(), local_schema());

    expect(xml).toContain("<?xml");
    expect(xml).toContain("vorgang.transportieren.2010");

    // Parse XML back to JSON (as shown in README)
    const obj = parse_vorgang_transportieren_2010(xml);
    expect(obj).toBeDefined();
    expect(obj.produkt).toBeDefined();
  });
});

describe("Creating XML Messages", () => {
  it("should create VorgangTransportieren2010 XML", () => {
    const qualityReport = JSON.parse(
      fs.readFileSync(
        path.resolve(__dirname, "./quality_report_minimal.json"),
        "utf-8"
      )
    );
    const xml = create_vorgang_transportieren_2010(
      qualityReport as unknown as VorgangTransportieren2010
    ).replace(schema(), local_schema());
    expect(xml).toContain("vorgang.transportieren.2010");
  });

  it("should create AdministrationQuittung0020 XML", () => {
    const receipt = JSON.parse(
      fs.readFileSync(
        path.resolve(__dirname, "./administration_receipt.json"),
        "utf-8"
      )
    );
    const xml = create_administration_quittung_0020(
      receipt as unknown as AdministrationQuittung0020
    );
    expect(xml).toContain("administration.quittung.0020");
  });

  it("should return metadata from xmlns/version/schema/local_schema", () => {
    expect(xmlns()).toBeTruthy();
    expect(typeof xmlns()).toBe("string");

    expect(version()).toBeTruthy();
    expect(typeof version()).toBe("string");

    expect(schema()).toBeTruthy();
    expect(typeof schema()).toBe("string");

    expect(local_schema()).toBeTruthy();
    expect(typeof local_schema()).toBe("string");
  });
});

describe("Parsing XML", () => {
  it("should parse VorgangTransportieren2010 XML and handle tagged unions", () => {
    const source = fs.readFileSync(
      path.resolve(__dirname, "./quality_report_minimal.xml"),
      "utf-8"
    );

    const obj = parse_vorgang_transportieren_2010(source);

    expect(obj.vorgang).toBeDefined();
    expect(obj.vorgang.vorgang_type).toBeDefined();
    expect(typeof obj.vorgang.vorgang_type.t).toBe("string");
    expect(typeof obj.vorgang.vorgang_type.c).toBe("object");

    // Tagged union pattern as shown in README
    if (obj.vorgang.vorgang_type.t === "Pruefbericht") {
      const pruefbericht: PruefberichtType = obj.vorgang.vorgang_type.c;
      expect(pruefbericht.id).toBeDefined();
    }
  });

  it("should parse AdministrationQuittung0020 XML", () => {
    const source = fs.readFileSync(
      path.resolve(__dirname, "./administration_receipt.xml"),
      "utf-8"
    );
    const receipt = parse_administration_quittung_0020(source);
    expect(receipt).toBeDefined();
  });
});

describe("Builder Functions — Transport Layer", () => {
  it("should create identifikation_nachricht", () => {
    const msgId = identifikation_nachricht("VorgangTransportieren2010");
    expect(msgId.nachrichten_uuid).toMatch(
      /^[a-fA-F0-9]{8}\-[a-fA-F0-9]{4}\-[a-fA-F0-9]{4}\-[a-fA-F0-9]{4}\-[a-fA-F0-9]{12}$/
    );
    expect(msgId.erstellungszeitpunkt).toBeTruthy();
  });

  it("should create nachrichtenkopf_g2g", () => {
    const header = nachrichtenkopf_g2g("VorgangTransportieren2010");
    expect(header).toBeDefined();
  });

  it("should create identifikation_vorgang with auto-generated and custom IDs", () => {
    const vId = identifikation_vorgang();
    expect(vId.vorgangs_id).toBeTruthy();

    const vIdCustom = identifikation_vorgang("my-id-123");
    expect(vIdCustom.vorgangs_id).toBe("my-id-123");
  });
});

describe("Builder Functions — Addresses", () => {
  it("should create anschrift_type as shown in README", () => {
    const addr = anschrift_type(
      "Musterstr.", // strasse
      "42", // hausnummer
      "12345", // postleitzahl
      "Musterstadt" // ort
    );
    expect(addr.strasse).toBe("Musterstr.");
    expect(addr.hausnummer).toBe("42");
    expect(addr.postleitzahl).toBe("12345");
    expect(addr.ort).toBe("Musterstadt");
    expect(addr.id).toMatch(/^anschrift-/);
  });
});

describe("Builder Functions — Persons and Organizations", () => {
  it("should create natuerliche_person_type and extract name via utility functions", () => {
    const person = natuerliche_person_type("Sepp", "Meier");
    expect(vorname(person)).toBe("Sepp");
    expect(familienname(person)).toBe("Meier");
    expect(person.id).toMatch(/^person-/);
  });

  it("should create allgemeiner_name_type and name_organisation_type", () => {
    const name = allgemeiner_name_type("Max Mustermann");
    expect(name.name).toBe("Max Mustermann");

    const orgName = name_organisation_type("ACME GmbH", "ACME");
    expect(orgName.name?.text).toBe("ACME GmbH");
    expect(orgName.kurzbezeichnung).toBe("ACME");
  });
});

describe("Builder Functions — Authorities and Institutions", () => {
  it("should create behoerde_type, autor, leser", () => {
    const authority = behoerde_type();
    expect(authority).toBeDefined();

    const author = autor("Name", "KENNUNG");
    expect(author.name).toBe("Name");

    const reader = leser("Name", "KENNUNG");
    expect(reader.name).toBe("Name");
  });

  it("should create zustaendige_behoerde_type with Bundesland code", () => {
    const responsible = zustaendige_behoerde_type("NW");
    expect(responsible.laenderkuerzel).toBeDefined();
  });

  it("should create zugelassene_untersuchungsstelle_type", () => {
    const labDetails: UntersuchungsstelleDetails = {
      id: "lab-1",
      name: "Umweltlabor GmbH",
      zugelassene_untersuchungsstelle_id: "ZUL-12345",
      pruefgebiete_untersuchungen_phys_chem: true,
      pruefgebiete_untersuchungen_mikrobio: true,
      pruefgebiete_untersuchungen_radionuklide: false,
      pruefgebiete_nur_vor_ort_parameter: false,
      akkreditierungsnummer: "AKK-D-PL-12345-01",
      unterorganisation: undefined,
    };
    const lab = zugelassene_untersuchungsstelle_type(labDetails);
    expect(lab).toBeDefined();
  });

  it("should create betreiber_type", () => {
    const operator = betreiber_type();
    expect(operator).toBeDefined();
  });
});

describe("Builder Functions — Pruefbericht (Test Report)", () => {
  it("should create pruefbericht_type as shown in README", () => {
    const labDetails: UntersuchungsstelleDetails = {
      id: "lab-1",
      name: "Umweltlabor GmbH",
      zugelassene_untersuchungsstelle_id: "ZUL-12345",
      pruefgebiete_untersuchungen_phys_chem: true,
      pruefgebiete_untersuchungen_mikrobio: true,
      pruefgebiete_untersuchungen_radionuklide: false,
      pruefgebiete_nur_vor_ort_parameter: false,
      akkreditierungsnummer: "AKK-D-PL-12345-01",
      unterorganisation: undefined,
    };
    const report = pruefbericht_type(
      "1.0", // sw_version
      null, // id (null = auto-generated)
      "test-context", // context
      labDetails
    );
    expect(report.sw_version).toBe("1.0");
    expect(report.id).toBeTruthy();
  });

  it("should create pruefbericht_signature_template", () => {
    const sig = pruefbericht_signature_template();
    expect(sig).toBeDefined();
  });

  it("should create sampling-related types", () => {
    const sample = probe_type();
    expect(sample).toBeDefined();

    const site = probennahmestelle_type("Sampling Point 1");
    expect(site.name_probennahmestelle).toBe("Sampling Point 1");

    const sampler = probennehmer_type();
    expect(sampler).toBeDefined();
  });

  it("should create analysis-related types", () => {
    const param = parameterangaben_type();
    expect(param).toBeDefined();

    const result = analyseergebnis_parameter_type("addr-id", "lab-id");
    expect(result).toBeDefined();
  });

  it("should create comment, period, and history types", () => {
    const comment = kommentar_type();
    expect(comment).toBeDefined();

    const period = zeitraum_type();
    expect(period).toBeDefined();

    const history = aenderungshistorie_type();
    expect(history).toBeDefined();
  });
});

describe("Builder Functions — Monitoring Plans, Objects, etc.", () => {
  it("should create all remaining builder types", () => {
    const plan = untersuchungsplan_type();
    expect(plan).toBeDefined();

    const schedule = terminplan_type("sampling-site-id");
    expect(schedule).toBeDefined();

    const wvg = wasserversorgungsgebiet_type();
    expect(wvg).toBeDefined();

    const obj = objekt_type();
    expect(obj).toBeDefined();

    const system = anlage_nach_trinkw_v_type();
    expect(system).toBeDefined();

    const org = organisation_type();
    expect(org).toBeDefined();

    const ident = identifikation_type();
    expect(ident).toBeDefined();

    const qm = quality_and_monitoring_type();
    expect(qm).toBeDefined();

    const derogation = derogation_type();
    expect(derogation).toBeDefined();

    const derogationRemedial = derogation_remedial_action_type();
    expect(derogationRemedial).toBeDefined();

    const exceedance = exceedance_type();
    expect(exceedance).toBeDefined();

    const exceedanceCause = exceedance_cause_and_remedial_action_type();
    expect(exceedanceCause).toBeDefined();

    const incident = incident_type();
    expect(incident).toBeDefined();

    const incidentCause = incident_cause_and_remedial_action_type();
    expect(incidentCause).toBeDefined();
  });
});

describe("Utility Functions", () => {
  it("should export vorname and familienname as shown in README", () => {
    const person = natuerliche_person_type("Sepp", "Meier");
    expect(vorname(person)).toBe("Sepp");
    expect(familienname(person)).toBe("Meier");
  });
});

describe("Schema Validation", () => {
  it("should have the .xsdb.bin file in the package", () => {
    const xsdbPath = path.resolve(__dirname, "../pkg/xwasser-v100.xsdb.bin");
    expect(fs.existsSync(xsdbPath)).toBe(true);
    const stat = fs.statSync(xsdbPath);
    expect(stat.size).toBeGreaterThan(1000);
  });
});

describe("Version Detection", () => {
  it("should detect version from XML as shown in README", () => {
    const source = fs.readFileSync(
      path.resolve(__dirname, "./quality_report_minimal.xml"),
      "utf-8"
    );
    const detectedVersion = detect_version(source.slice(0, 1024));
    expect(detectedVersion).toBe("100");
  });
});

describe("Codelists", () => {
  it("should have a codelist.json in the package", () => {
    const codelistPath = path.resolve(__dirname, "../pkg/codelist.json");
    expect(fs.existsSync(codelistPath)).toBe(true);
    const codelists = JSON.parse(fs.readFileSync(codelistPath, "utf-8"));
    expect(Array.isArray(codelists)).toBe(true);
    expect(codelists.length).toBeGreaterThan(0);
  });

  it("should be able to look up a codelist by short_name as shown in README", () => {
    const codelists = JSON.parse(
      fs.readFileSync(
        path.resolve(__dirname, "../pkg/codelist.json"),
        "utf-8"
      )
    );

    // Find a codelist by its short name (as shown in README)
    const samplingSites = codelists.find(
      (cl: any) =>
        cl.header.identification.short_name === "art-probennahmestelle"
    );

    expect(samplingSites).toBeDefined();
    expect(samplingSites.header.identification.long_name).toBe(
      "Art der Probennahmestelle"
    );

    // Iterate permitted values (as shown in README)
    for (const value of samplingSites.values) {
      expect(Array.isArray(value)).toBe(true);
      expect(value[0]).toBeTruthy(); // Key/Code
    }
  });

  it("should have CodeGesamtbewertungType compatible with the README example", () => {
    // As shown in README
    const bewertung: CodeGesamtbewertungType = {
      code: "1010",
      name: "Einwandfrei",
    };
    expect(bewertung.code).toBe("1010");
    expect(bewertung.name).toBe("Einwandfrei");
  });
});
