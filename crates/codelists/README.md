# Codelists

This crate is responsible to offer in memory codelists from XWasser.
The original xml files are stored under the directory `./data/${VERSION}` and the parsed JSON variants under `./public/${VERSION}`.

XML files source: https://www.xrepository.de/api/version_standard/urn:xoev-de:lgl:standard:xwasser_0.9.2/genutzteAktuelleCodelisten

Once the content in `./data` has changed, only run `cargo build` to update the JSON variants.

## Versions

- XWasser Codelist [v0.9.2](./public/V0_9_2/README.md)