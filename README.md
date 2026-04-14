# xoev-xwasser

The XÖV data exchange standard XWasser is used for the digital exchange of data between operators of water supply systems (operators), who are responsible for regular drinking water testing in accordance with the quality standards of the German Drinking Water Ordinance (TrinkwV), and the responsible public health authorities (GA) as well as other authorities and diagnostic service providers.
The standard therefore also covers the direct transmission of test reports from approved testing institutes (laboratories) to the health authorities as well as the annual transmission of drinking water quality data by the health authorities to the competent supreme state authority (OLB), which is subject to reporting requirements.
The XWasser standard is part of the project "Nationwide standardized digital data exchange for drinking water hygiene (SHAPTH)" of all 16 federal states within the framework of the Pact for the Public Health Service (ÖGD).


## Legal

Licensed under `MIT` license.

## Versioning

This crate follows the latest minor and patch versions for each maintained major version.

The crate versions follow the `X.Y.Z+A.B.C` pattern:

- The major version `X` is the upstream XÖV XWasser API compatibility version:
  - `300` for 3.Y.Z
- The patch `Y` version is incremented when making XÖV XWasser update. It is
  equal to `B*100 + C`.
- The patch `Z` version is incremented when making internal changes
  to the crate.
- `A.B.C` contains the full upstream XÖV XWasser version, like `0.2.1` or `0.5.0`.
  Note that this field is actually ignored in comparisons and only there for
  documentation.

## Features

This crate provides several features that can be enabled:

| Feature | Description |
|---------|-------------|
| `builder` | Enables builder patterns for type construction using `TypedBuilder` |
| `wasm` | Enables WebAssembly bindings using `wasm-bindgen` and `tsify` |
| `schema` | Enables XSD schema validation |
| `trace` | Enables tracing for debugging |

### Default Features

By default, no features are enabled. To use the crate with builders and schema validation:

```toml
xoev-xwasser = { version = "1.0.0", features = ["builder", "schema"] }
```

### Builder Pattern

The builder feature enables a fluent API for constructing XML messages:

```rust
use xoev_xwasser::{
    builder::{
        pruefbericht::pruefbericht_type,
        transport::{nachrichtenkopf_g2g, NachrichtenTypEnum},
        vorgang::identifikation_vorgang,
    },
    model::{
        signature::Signature,
        transport::VorgangTransportieren2010,
        vorgang::{Vorgang, VorgangType},
    },
};

// Create a Pruefbericht
let pruefbericht = pruefbericht_type(
    "1.0".into(),
    None,
    "test-context".into(),
    Default::default(),
);

// Build the message with all required fields
let message = VorgangTransportieren2010::builder()
    .produkt("MyApp".into())
    .produkthersteller("My Company".into())
    .produktversion("1.0.0".into())
    .nachrichtenkopf_g2g(nachrichtenkopf_g2g(
        NachrichtenTypEnum::VorgangTransportieren2010,
    ))
    .vorgang(
        Vorgang::builder()
            .identifikation_vorgang(identifikation_vorgang(None))
            .vorgang_type(VorgangType::Pruefbericht(pruefbericht))
            .bemerkung(None)
            .anlage(Default::default())
            .build(),
    )
    .zusatzinformationen(Default::default())
    .signature(Some(Signature { exists: true }))
    .build();
```

### Serialization

The crate supports both XML and JSON serialization:

```rust
use raxb::{ser::to_string, de::from_str};
use serde_json;

// Serialize to XML
let xml = to_string(&message)?;

// Serialize to JSON
let json = serde_json::to_string(&message)?;

// Deserialize from XML
let parsed: VorgangTransportieren2010 = from_str(&xml)?;
```

### Schema Validation

The `schema` feature compiles XSD schemas into a binary format (`.xsdb`) and provides validation:

```rust
use xoev_xwasser::schemas::XmlValidation;

let validation = XmlValidation::new()?;
validation.validate(xml_bytes)?;
```

**How it works:**
1. During build/test, `raxb_xmlschema_build` compiles `schemas/V1_0_0/xwasser.xsd` to `.xsdb` format
2. The compiled schemas are embedded via `rust_embed`
3. At runtime, XML is validated against the compiled schemas using `raxb_validate`

**Note:** Schema compilation runs automatically via the `schema` feature test:
```sh
cargo test --features schema --test generate_schemas
```

## Usage

### Basic Example

```rust
use xoev_xwasser::{
    builder::{
        pruefbericht::pruefbericht_type,
        transport::{nachrichtenkopf_g2g, NachrichtenTypEnum},
        vorgang::identifikation_vorgang,
    },
    model::{
        signature::Signature,
        transport::VorgangTransportieren2010,
        vorgang::{Vorgang, VorgangType},
    },
};

let pruefbericht = pruefbericht_type(
    "1.0".into(),
    None,
    "context".into(),
    Default::default(),
);

let message = VorgangTransportieren2010::builder()
    .produkt("MyApp".into())
    .produkthersteller("My Company".into())
    .produktversion("1.0.0".into())
    .nachrichtenkopf_g2g(nachrichtenkopf_g2g(
        NachrichtenTypEnum::VorgangTransportieren2010,
    ))
    .vorgang(
        Vorgang::builder()
            .identifikation_vorgang(identifikation_vorgang(None))
            .vorgang_type(VorgangType::Pruefbericht(pruefbericht))
            .bemerkung(None)
            .anlage(Default::default())
            .build(),
    )
    .zusatzinformationen(Default::default())
    .signature(Some(Signature { exists: true }))
    .build();
```

### Working with Addresses

```rust
use xoev_xwasser::builder::shared::anschrift::anschrift_type;

let addr = anschrift_type(
    "Teststr.".into(),
    "1".into(),
    "12345".into(),
    "Testort".into(),
);
```

### Codelist Codes

```rust
use xoev_xwasser::model::codes::CodeGesamtbewertungType;

let bewertung = CodeGesamtbewertungType::builder()
    .code("1010".into())
    .name(None)
    .build();
```

## TypeScript / WebAssembly

The crate compiles to WebAssembly for use in TypeScript/JavaScript applications:

```bash
npm install xoev-xwasser
```

### Creating XML Messages

```typescript
import { 
  create_vorgang_transportieren_2010,
  parse_vorgang_transportieren_2010,
  VorgangTransportieren2010,
  PruefberichtType,
  schema,
  local_schema,
} from "xoev-xwasser";

import quality_report from "./quality_report.json";

// Create XML from JSON object
const xml = create_vorgang_transportieren_2010(
  quality_report as unknown as VorgangTransportieren2010
).replace(schema(), local_schema());
```

### Parsing XML

```typescript
import { parse_vorgang_transportieren_2010 } from "xoev-xwasser";

const source = `<?xml version="1.0"?>
<VorgangTransportieren2010>...</VorgangTransportieren2010>`;

const obj = parse_vorgang_transportieren_2010(source);
if (obj.vorgang.vorgang_type.t === "Pruefbericht") {
  const p = obj.vorgang.vorgang_type.c;
  console.log(p.id);
}
```

### Utility Functions

```typescript
import { vorname, familienname } from "xoev-xwasser/utils";
import { natuerliche_person_type } from "xoev-xwasser";

const person = natuerliche_person_type("Sepp", "Meier");
console.log(vorname(person));    // "Sepp"
console.log(familienname(person)); // "Meier"
```

### Schema Validation (Node.js)

```typescript
import fs from "fs";
import xmlvalidate, { XmlValidatorError } from "@raxb/validate-wasm";

// Load compiled schema
const xsdBundle = fs.readFileSync(
  "node_modules/xoev-xwasser/xwasser-v100.xsdb.bin"
).buffer;

const { XmlValidator } = await xmlvalidate();
const validator = new XmlValidator(new Uint8Array(xsdBundle));
validator.init((err: string) => console.error(err));

// Validate XML
validator.validateXml(xmlString, (err: XmlValidatorError) => {
  if (err.level === "fatal") {
    console.error({ line: err.line, message: err.message });
  }
});
```

## Development

### Building

```sh
# Build with all features
cargo build --features builder,wasm,schema

# Run tests
cargo test --features builder,schema

# Run clippy
cargo clippy --features builder
```

### Full Build (includes WASM)

```sh
./build.sh
```

### Testing

Tests are run with specific feature combinations:

```sh
# Core tests
cargo test

# Schema validation tests
cargo test --features schema,trace

# Builder tests
cargo test --features builder

# Full test suite
cargo test --no-default-features --features schema,trace,builder
```

### Code Linting

```sh
cargo clippy --features builder --fix --allow-dirty
```

### WASM Development

For WASM builds during development:

```sh
# Watch mode with auto-rebuild
cargo watch --watch src -s "./build.sh"
```

For TypeScript testing:

```sh
pnpm i
pnpm test
```
