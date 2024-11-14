pub(crate) mod cl;
pub mod v0_7_0;

use std::{collections::HashMap, sync::Arc};

pub use crate::cl::model::*;

#[derive(serde::Deserialize)]
pub struct DataSet {
    pub items: Vec<CodeList>,
}

fn get<S>() -> anyhow::Result<DataSet>
where
    S: rust_embed::RustEmbed,
{
    let file = S::get(CodeList::name()).unwrap();
    Ok(bson::from_slice(&file.data)?)
}

pub fn map<S>() -> anyhow::Result<CodeListMap>
where
    S: rust_embed::RustEmbed,
{
    Ok(Arc::new(
        get::<S>()?
            .items
            .into_iter()
            .map(|v| (v.header.identification.canonical_uri.clone(), v))
            .collect(),
    ))
}

pub type CodeLists = HashMap<Arc<str>, CodeList>;
pub type CodeListMap = Arc<CodeLists>;

pub trait XWasserCodeListValue {
    const CODELIST: &str;

    fn validate(&self, codelists: &HashMap<Arc<str>, CodeList>) -> bool {
        codelists
            .get(Self::CODELIST)
            .map(|codelist| codelist.validate(self.as_value()))
            .unwrap_or_default()
    }

    fn as_value(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_set_v0_7_0() -> anyhow::Result<()> {
        let source = map::<crate::v0_7_0::Source>()?;

        assert_eq!(source.len(), 73);

        impl<T> XWasserCodeListValue for T
        where
            T: AsRef<str>,
        {
            const CODELIST: &str = "urn:xoev-de:xwasser:codeliste:parameterauspraegung";

            fn as_value(&self) -> &str {
                self.as_ref()
            }
        }

        assert!("10005-2".validate(&source));
        assert!(!"qqq".validate(&source));

        Ok(())
    }
}
