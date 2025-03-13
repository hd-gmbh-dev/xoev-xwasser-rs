pub(crate) mod cl;
pub mod v0_9_0;

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

pub trait CodeListsProvider {
    fn contains(&self, codelist: &str, value: &str) -> bool;
}

impl CodeListsProvider for CodeLists {
    fn contains(&self, codelist: &str, value: &str) -> bool {
        self.get(codelist)
            .map(|codelist| codelist.validate(value))
            .unwrap_or_default()
    }
}

impl<T> CodeListsProvider for Arc<T>
where
    T: CodeListsProvider,
{
    fn contains(&self, codelist: &str, value: &str) -> bool {
        T::contains(self, codelist, value)
    }
}

pub trait CodeListValue {
    const CODELIST: &str;

    fn validate(&self, codelists: &impl CodeListsProvider) -> bool {
        codelists.contains(Self::CODELIST, self.as_value())
    }

    fn as_value(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<T> CodeListValue for T
    where
        T: AsRef<str>,
    {
        const CODELIST: &str = "urn:xoev-de:xwasser:codeliste:parameterauspraegung";

        fn as_value(&self) -> &str {
            self.as_ref()
        }
    }

    #[test]
    fn test_data_set_v0_9_0() -> anyhow::Result<()> {
        let source = map::<crate::v0_9_0::Source>()?;

        assert_eq!(source.len(), 71);

        assert!("10005-2".validate(source.as_ref()));
        assert!(!"qqq".validate(source.as_ref()));

        Ok(())
    }
}
