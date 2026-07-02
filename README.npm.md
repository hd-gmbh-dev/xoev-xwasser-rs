# xoev-xwasser

[![npm version](https://img.shields.io/npm/v/xoev-xwasser)](https://www.npmjs.com/package/xoev-xwasser)

The XÖV data exchange standard XWasser is used for the digital exchange of data between operators of water supply systems (operators), who are responsible for regular drinking water testing in accordance with the quality standards of the German Drinking Water Ordinance (TrinkwV), and the responsible public health authorities (GA) as well as other authorities and diagnostic service providers.
The standard therefore also covers the direct transmission of test reports from approved testing institutes (laboratories) to the health authorities as well as the annual transmission of drinking water quality data by the health authorities to the competent supreme state authority (OLB), which is subject to reporting requirements.
The XWasser standard is part of the project "Nationwide standardized digital data exchange for drinking water hygiene (SHAPTH)" of all 16 federal states within the framework of the Pact for the Public Health Service (ÖGD).

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Creating XML Messages](#creating-xml-messages)
- [Parsing XML](#parsing-xml)
- [Builder Functions](#builder-functions)
- [Utility Functions](#utility-functions)
- [Schema Validation (Node.js)](#schema-validation-nodejs)
- [Version Detection](#version-detection)
- [Web (Browser) Usage](#web-browser-usage)
- [TypeScript Types](#typescript-types)
- [Examples](#examples)
- [For Rust Development](#for-rust-development)
- [License](#license)

---

## Installation

```bash
npm install xoev-xwasser
```

For browser usage, see [Web (Browser) Usage](#web-browser-usage).

## Quick Start

```typescript
import {
  create_vorgang_transportieren_2010,
  parse_vorgang_transportieren_2010,
  schema,
  local_schema,
  VorgangTransportieren2010,
} from "xoev-xwasser";

import qualityReport from "./quality_report.json";

// Create XML from a JSON object
const xml = create_vorgang_transportieren_2010(
  qualityReport as unknown as VorgangTransportieren2010
).replace(schema(), local_schema());

// Parse XML back to JSON
const obj = parse_vorgang_transportieren_2010(xml);
```

## Creating XML Messages

### create_vorgang_transportieren_2010

Serializes a `VorgangTransportieren2010` object to an XML string suitable for the XWasser transport protocol:

```typescript
import { create_vorgang_transportieren_2010, schema, local_schema } from "xoev-xwasser";

const xml = create_vorgang_transportieren_2010(data)
  .replace(schema(), local_schema());
```

The `schema()` / `local_schema()` replacement is needed because the serialized XML uses the full namespace URI, but the XSD schema reference in the XML should point to the local schema file location.

### create_administration_quittung_0020

Serializes an `AdministrationQuittung0020` object (administration receipt/acknowledgement):

```typescript
import { create_administration_quittung_0020 } from "xoev-xwasser";

const xml = create_administration_quittung_0020(data);
```

### xmlns / version / schema / local_schema

Utility functions for namespace and version metadata:

```typescript
import { xmlns, version, schema, local_schema } from "xoev-xwasser";

console.log(xmlns());        // XML namespace URI
console.log(version());      // Current XWasser schema version
console.log(schema());       // Full schema namespace URI
console.log(local_schema()); // Local schema file reference
```

## Parsing XML

### parse_vorgang_transportieren_2010

Parses an XML string back into a typed `VorgangTransportieren2010` object:

```typescript
import { parse_vorgang_transportieren_2010 } from "xoev-xwasser";

const source = `<?xml version="1.0"?>
<VorgangTransportieren2010 xmlns="...">...</VorgangTransportieren2010>`;

const obj = parse_vorgang_transportieren_2010(source);

// Tagged union pattern for variant types
if (obj.vorgang.vorgang_type.t === "Pruefbericht") {
  const pruefbericht = obj.vorgang.vorgang_type.c;
  console.log(pruefbericht.id);
}
```

### parse_administration_quittung_0020

Parses an administration receipt XML:

```typescript
import { parse_administration_quittung_0020 } from "xoev-xwasser";

const receipt = parse_administration_quittung_0020(xmlSource);
```

## Builder Functions

The package provides factory functions to construct typed objects with sensible defaults, auto-generated IDs, and timestamp creation.

### Transport Layer

```typescript
import {
  identifikation_nachricht,
  nachrichtenkopf_g2g,
  identifikation_vorgang,
} from "xoev-xwasser";

const msgId = identifikation_nachricht("VorgangTransportieren2010");
// Returns: { nachrichten_uuid: "...", nachrichten_typ: {...}, erstellungszeitpunkt: "..." }

const header = nachrichtenkopf_g2g("VorgangTransportieren2010");

const vId = identifikation_vorgang();                  // auto-generated vorgangs_id
const vIdCustom = identifikation_vorgang("my-id-123"); // custom vorgangs_id
```

### Addresses

```typescript
import { anschrift_type } from "xoev-xwasser";

const addr = anschrift_type(
  "Musterstr.",    // strasse
  "42",            // hausnummer
  "12345",         // postleitzahl
  "Musterstadt",   // ort
);
// Returns: { strasse, hausnummer, postleitzahl, ort, id: "anschrift-..." }
```

### Persons and Organizations

```typescript
import {
  natuerliche_person_type,
  allgemeiner_name_type,
  name_organisation_type,
} from "xoev-xwasser";
import { vorname, familienname } from "xoev-xwasser/utils";

const person = natuerliche_person_type("Sepp", "Meier");
// Returns: { name_natuerliche_person: {...}, id: "person-..." }
// Use utility functions to extract name parts:
const first = vorname(person);       // "Sepp"
const last  = familienname(person);  // "Meier"

const name = allgemeiner_name_type("Max Mustermann");
// Returns: { name: "Max Mustermann" }

const orgName = name_organisation_type("ACME GmbH", "ACME");
// Returns: { name: { text: "ACME GmbH" }, kurzbezeichnung: "ACME", ... }
```

### Authorities and Institutions

```typescript
import {
  behoerde_type,
  autor,
  leser,
  zustaendige_behoerde_type,
  zugelassene_untersuchungsstelle_type,
  beauftragte_untersuchungsstelle_type,
  betreiber_type,
  UntersuchungsstelleDetails,
} from "xoev-xwasser";

const authority = behoerde_type();
const author = autor("Name", "KENNUNG");
const reader = leser("Name", "KENNUNG");
const responsible = zustaendige_behoerde_type("NW"); // Bundesland code

// Untersuchungsstelle with required accreditation details
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
const commissioner = beauftragte_untersuchungsstelle_type(labDetails);
const operator = betreiber_type();
```

### Pruefbericht (Test Report)

```typescript
import {
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
  UntersuchungsstelleDetails,
} from "xoev-xwasser";

// Untersuchungsstelle details
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

// Create a test report
const report = pruefbericht_type(
  "1.0",          // sw_version
  null,           // id (null = auto-generated)
  "test-context", // context
  labDetails,
);

const sig = pruefbericht_signature_template(); // Signature template

// Sampling
const sample = probe_type();
const site = probennahmestelle_type("Sampling Point 1");
// site.name_probennahmestelle === "Sampling Point 1"

const sampler = probennehmer_type();

// Analysis
const param = parameterangaben_type();
const result = analyseergebnis_parameter_type("addr-id", "lab-id");

const comment = kommentar_type();
const period = zeitraum_type();
const history = aenderungshistorie_type();
```

### Monitoring Plans, Objects, and More

```typescript
import {
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
} from "xoev-xwasser";

const plan = untersuchungsplan_type();
const schedule = terminplan_type("sampling-site-id");
const wvg = wasserversorgungsgebiet_type();
const obj = objekt_type();
const system = anlage_nach_trinkw_v_type();
const org = organisation_type();
const ident = identifikation_type();
const qm = quality_and_monitoring_type();
```

### Full List of Builder Functions

| Function | Returns | Description |
|----------|---------|-------------|
| `identifikation_nachricht(typ)` | `IdentifikationNachricht` | Message ID with auto-generated UUID and timestamp |
| `nachrichtenkopf_g2g(typ)` | `NachrichtenkopfG2g` | G2G message header |
| `identifikation_vorgang(id?)` | `IdentifikationVorgang` | Process/transaction ID |
| `anschrift_type(str, nr, plz, ort)` | `AnschriftType` | Address with auto-generated ID |
| `natuerliche_person_type(vorname, name)` | `NatuerlichePersonType` | Natural person |
| `allgemeiner_name_type(name)` | `AllgemeinerNameType` | General name |
| `name_organisation_type(name?, kurz?)` | `NameOrganisationType` | Organisation name |
| `behoerde_type()` | `BehoerdeType` | Authority |
| `autor(name, kennung)` | `BehoerdeG2GType` | Author authority |
| `leser(name, kennung)` | `BehoerdeG2GType` | Reader authority |
| `zustaendige_behoerde_type(kuerzel)` | `ZustaendigeBehoerdeType` | Responsible authority (by Bundesland) |
| `zugelassene_untersuchungsstelle_type(details)` | `ZugelasseneUntersuchungsstelleType` | Approved testing lab |
| `beauftragte_untersuchungsstelle_type(details)` | `BeauftragteUntersuchungsstelleType` | Commissioned testing lab |
| `betreiber_type()` | `BetreiberType` | Operator |
| `pruefbericht_type(ver, id, ctx, details)` | `PruefberichtType` | Test report |
| `pruefbericht_signature_template()` | `Signature` | Signature template |
| `probe_type()` | `ProbeType` | Sample |
| `probennahmestelle_type(name)` | `ProbennahmestelleType` | Sampling point |
| `probenehmer_type()` | `ProbennehmerType` | Sampler |
| `parameterangaben_type()` | `ParameterangabenType` | Parameter specification |
| `analyseergebnis_parameter_type(addr, lab)` | `AnalyseergebnisParameterType` | Analysis result |
| `kommentar_type()` | `KommentarType` | Comment |
| `zeitraum_type()` | `ZeitraumType` | Time period |
| `aenderungshistorie_type()` | `AenderungshistorieType` | Change history |
| `untersuchungsplan_type()` | `UntersuchungsplanType` | Investigation plan |
| `terminplan_type(siteId)` | `TerminplanType` | Schedule plan |
| `wasserversorgungsgebiet_type()` | `WasserversorgungsgebietType` | Water supply area |
| `objekt_type()` | `ObjektType` | Object |
| `anlage_nach_trinkw_v_type()` | `AnlageNachTrinkwVType` | System per TrinkwV |
| `organisation_type()` | `OrganisationType` | Organisation |
| `identifikation_type()` | `IdentifikationType` | Identification |
| `quality_and_monitoring_type()` | `QualityAndMonitoringType` | Quality & monitoring |
| `derogation_type()` | `DerogationType` | Derogation |
| `derogation_remedial_action_type()` | `DerogationRemedialActionType` | Derogation remedial action |
| `exceedance_type()` | `ExceedanceType` | Exceedance |
| `exceedance_cause_and_remedial_action_type()` | `ExceedanceCauseAndRemedialActionType` | Exceedance cause & remediation |
| `incident_type()` | `IncidentType` | Incident |
| `incident_cause_and_remedial_action_type()` | `IncidentCauseAndRemedialActionType` | Incident cause & remediation |

## Working with Codelists

The XWasser standard defines many codelists (controlled vocabularies) for fields like water type, sampling location category, test result evaluation, parameters, and more. The package provides both typed code interfaces and the raw codelist data.

### Typed Code Interfaces

Each codelist has a corresponding TypeScript interface with `code` and `name` fields:

```typescript
import { CodeGesamtbewertungType } from "xoev-xwasser";

// Use in builder pattern
const bewertung: CodeGesamtbewertungType = {
  code: "1010",
  name: "Einwandfrei",
};
```

Common code interfaces include:

| Interface | Codelist |
|-----------|----------|
| `CodeGesamtbewertungType` | Overall assessment |
| `CodeMediumType` | Medium (water type) |
| `CodeProbennahmeverfahrenType` | Sampling procedure |
| `CodeUntersuchungsanlassType` | Investigation reason |
| `CodeShapthParameterType` | SHAPTH parameter |
| `CodeShapthParameterEinheitType` | SHAPTH parameter unit |
| `CodeArtProbennahmestelleType` | Sampling point type |
| `CodeKategorieProbennahmestelleType` | Sampling point category |
| `CodeMassnahmeType` | Measures |
| `CodeAbhilfemassnahmeType` | Remedial actions |
| `CodeNamensartType` | Name type |
| `CodeStaatType` | Country |
| `CodeStaatsangehoerigkeitType` | Nationality |
| `CodeAgsType` | Municipality (AGS) |
| `CodeKreisType` | District |
| `CodeBundeslandType` | Federal state |
| `CodeNachrichtentypType` | Message type |
| `CodePersonenrolleType` | Person role |
| `CodeKommunikationKanalType` | Communication channel |

Refer to the included `xoev_xwasser.d.ts` for the complete list of code interfaces.

### Raw Codelist Data

The full codelist dataset is included as `codelist.json` in the package root. It contains all 72 codelists with their metadata, field definitions, and permitted values:

```typescript
import codelists from "xoev-xwasser/codelist.json" with { type: "json" };
// or: const codelists = require("xoev-xwasser/codelist.json");

// Find a codelist by its short name
const samplingSites = codelists.find(
  (cl) => cl.header.identification.short_name === "art-probennahmestelle"
);

console.log(samplingSites.header.identification.long_name);
// "Art der Probennahmestelle"

// Iterate permitted values
for (const value of samplingSites.values) {
  // Each value is an array matching the fields defined in header.fields
  console.log(value[0]); // Key/Code
  console.log(value[1]); // Description
}
```

Example codelist JSON structure:

```json
{
  "header": {
    "identification": {
      "short_name": "medium",
      "long_name": "Medium",
      "version": "2",
      "canonical_uri": "urn:xoev-de:xwasser:codeliste:medium"
    },
    "description": {
      "codelist_description": "Diese Codeliste dient im Kontext von XWasser der Angabe des Mediums einer Probennahme."
    },
    "fields": [
      { "id": "Key", "field_type": "RecommendedKey", "usage": "Required" },
      { "id": "Medium", "field_type": "Value", "usage": "Required" }
    ],
    "key_index": 0
  },
  "values": [
    ["1010", "Trinkwasser"],
    ["1020", "Rohwasser"],
    ["1030", "Grundwasser"],
    ["1040", "Oberflächenwasser"]
  ]
}
```

### Validating Code Values

In Rust, the `validate` feature provides `XWasserValidate` to check code values against the codelists. In TypeScript, you can validate manually by loading `codelist.json` and checking values at runtime.

## Utility Functions

Additional helpers are available from the `xoev-xwasser/utils` subpath:

```typescript
import { vorname, familienname } from "xoev-xwasser/utils";
import { natuerliche_person_type } from "xoev-xwasser";

const person = natuerliche_person_type("Sepp", "Meier");
console.log(vorname(person));     // "Sepp"
console.log(familienname(person)); // "Meier"
```

## Schema Validation

Schema validation requires the separate `@raxb/validate-wasm` package:

```bash
npm install @raxb/validate-wasm
```

The `.xsdb.bin` file is included in the npm package and contains the compiled XWasser XSD schema for version 1.0.0.

### Node.js

```typescript
import fs from "fs";
import xmlvalidate, { XmlValidatorError } from "@raxb/validate-wasm";

// Load the compiled XSD bundle from the package
const xsdBundle = fs.readFileSync(
  "node_modules/xoev-xwasser/xwasser-v100.xsdb.bin"
).buffer;

const { XmlValidator } = await xmlvalidate();
const validator = new XmlValidator(new Uint8Array(xsdBundle));
validator.init((err: string) => console.error(err));

// Validate XML against the schema
validator.validateXml(xmlString, (err: XmlValidatorError) => {
  if (err.level === "fatal") {
    console.error({ line: err.line, message: err.message });
  }
});
```

### Browser

```typescript
import xmlvalidate, { XmlValidatorError } from "@raxb/validate-wasm";

// Fetch the compiled XSD bundle (served from your public directory)
const response = await fetch("/xwasser-v100.xsdb.bin");
const xsdBundle = await response.arrayBuffer();

const { XmlValidator } = await xmlvalidate();
const validator = new XmlValidator(new Uint8Array(xsdBundle));
validator.init((err: string) => console.error(err));

// Validate XML against the schema
validator.validateXml(xmlString, (err: XmlValidatorError) => {
  if (err.level === "fatal") {
    console.error({ line: err.line, message: err.message });
  }
});
```

## Version Detection

Automatically detect the XWasser schema version from an XML document:

```typescript
import { detect_version } from "xoev-xwasser";

const version = detect_version(xmlString.slice(0, 1024));
console.log(version); // e.g. "100"
```

The function only needs the first ~1KB of the XML to detect the version from the namespace declaration.

## Web (Browser) Usage

For browser environments, use the `xoev-xwasser-web` package which compiles with the Web target:

```bash
npm install xoev-xwasser-web
```

The API is the same as the Node.js version, but the WASM binary is built for web bundlers (ES modules with top-level await).

```typescript
import {
  create_vorgang_transportieren_2010,
  parse_vorgang_transportieren_2010,
} from "xoev-xwasser-web";
```



## TypeScript Types

The package includes full TypeScript type definitions. All model types are exported from the main entry point:

```typescript
import {
  VorgangTransportieren2010,
  AdministrationQuittung0020,
  PruefberichtType,
  AnschriftType,
  NatuerlichePersonType,
  ProbennahmestelleType,
  // ... and many more
} from "xoev-xwasser";
```

Key types include:

| Interface | Description |
|-----------|-------------|
| `VorgangTransportieren2010` | Root transport message |
| `AdministrationQuittung0020` | Administration receipt |
| `PruefberichtType` | Test report (water quality) |
| `UntersuchungsplanType` | Monitoring/investigation plan |
| `MassnahmenType` | Measures |
| `AnschriftType` | Address |
| `NatuerlichePersonType` | Natural person |
| `OrganisationType` | Organisation |
| `BehoerdeType` | Authority |
| `ProbennahmestelleType` | Sampling point |
| `ProbeType` | Sample |
| `ParameterangabenType` | Parameter specification |
| `Vorgang` | Process/transaction wrapper |

Refer to the included `xoev_xwasser.d.ts` file for the complete type reference.

## Examples

See the [tests directory](https://github.com/hd-gmbh-dev/xoev-xwasser-rs/tree/main/tests) in the repository for complete examples with JSON payloads and expected XML output:

| Test | Description |
|------|-------------|
| `quality_report_builder.json` | Quality report with full data |
| `quality_report_minimal.json` | Minimal quality report |
| `administration_receipt.json` | Administration receipt |
| `monitoring_plan_maximal.json` | Full monitoring plan |
| `monitoring_plan_minimal.json` | Minimal monitoring plan |
| `olb_report_minimal.json` | OLB (state authority) report |

## For Rust Development

This package is built from the [xoev-xwasser-rs](https://github.com/hd-gmbh-dev/xoev-xwasser-rs) repository. If you need:

- **Rust crate** — published on [crates.io](https://crates.io/crates/xoev-xwasser) as `xoev-xwasser`
- **Building from source** — see the repository README for Rust development setup
- **Contributing** — PRs and issues are welcome on GitHub

## License

Licensed under the [MIT](https://github.com/hd-gmbh-dev/xoev-xwasser-rs/blob/main/LICENSE) license.
