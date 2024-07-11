# xoev-xwasser-rs

The XÖV data exchange standard XWasser is used for the digital exchange of data between operators of water supply systems (operators), who are responsible for regular drinking water testing in accordance with the quality standards of the German Drinking Water Ordinance (TrinkwV), and the responsible public health authorities (GA) as well as other authorities and diagnostic service providers.
The standard therefore also covers the direct transmission of test reports from approved testing institutes (laboratories) to the health authorities as well as the annual transmission of drinking water quality data by the health authorities to the competent supreme state authority (OLB), which is subject to reporting requirements.
The XWasser standard is part of the project "Nationwide standardized digital data exchange for drinking water hygiene (SHAPTH)" of all 16 federal states within the framework of the Pact for the Public Health Service (ÖGD).

## Legal

Licensed under `MIT` license.

## Features

TBD.

## Usage

TBD.

## Development

wasm-pack watcher:

```sh
cargo watch --watch src -s "wasm-pack build --release --target nodejs --reference-types --weak-refs"
```

vitest:

```sh
pnpm i
pnpm test
```
