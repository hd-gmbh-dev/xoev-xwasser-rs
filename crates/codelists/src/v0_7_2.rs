use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "$OUT_DIR/data/V0_7_2"]
pub struct Source;
