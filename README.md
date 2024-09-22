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

TBD.

## Usage

TBD.

## Development

wasm-pack watcher:

```sh
cargo watch --watch src -s "./build.sh"
```

vitest:

```sh
pnpm i
pnpm test
```
