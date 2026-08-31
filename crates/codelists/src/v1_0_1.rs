use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "$OUT_DIR/data/V1_0_1"]
pub struct Source;
