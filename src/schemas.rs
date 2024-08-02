use raxb_validate::{ValidationMap, ValidationResult};
use rust_embed::RustEmbed;
use std::sync::Arc;

#[derive(RustEmbed)]
#[folder = "target/schemas/out"]
pub struct Source;

#[derive(Clone, Debug)]
pub struct XmlValidation {
    inner: Arc<ValidationMap>,
}

impl XmlValidation {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            inner: Arc::new(ValidationMap::try_from_iter(
                Source::iter()
                    .filter_map(|s| Source::get(&s))
                    .map(|v| v.data),
            )?),
        })
    }

    pub fn validate(&self, xml: &[u8]) -> ValidationResult<()> {
        self.inner.validate(xml)
    }
}
