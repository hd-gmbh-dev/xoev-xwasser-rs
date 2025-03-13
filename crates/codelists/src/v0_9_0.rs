use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "$OUT_DIR/data/V0_9_0"]
pub struct Source;
