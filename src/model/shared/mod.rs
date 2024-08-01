#![allow(non_snake_case, dead_code)]


#[cfg(feature = "wasm")]
use tsify::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub mod anschrift;
pub mod auftraggeber;
pub mod behoerde;
pub mod betreiber;
pub mod dokument;
pub mod kommunikation;
pub mod misc;
pub mod organisation;
pub mod person;
pub mod probe;
pub mod sprache;
pub mod staat;
pub mod unterssuchungsstelle;
pub mod wasserversorgung;
pub mod xoev;
pub mod zeitraum;
