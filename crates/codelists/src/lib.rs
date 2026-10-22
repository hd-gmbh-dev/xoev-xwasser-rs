pub(crate) mod cl;
pub mod v1_0_0;

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

/// Answers "is this code part of codelist `codelist` in version `version`".
///
/// The version is part of the question on purpose: for XÖV code type 3 the
/// sender chooses the version (`listVersionID`), so a value that is valid in one
/// version may be unknown in another.
pub trait CodeListsProvider {
    fn contains(&self, codelist: &str, version: &str, value: &str) -> bool;
}

impl CodeListsProvider for CodeLists {
    fn contains(&self, codelist: &str, version: &str, value: &str) -> bool {
        self.get(codelist)
            .filter(|codelist| codelist.header.identification.version.as_ref() == version)
            .map(|codelist| codelist.validate(value))
            .unwrap_or_default()
    }
}

impl<T> CodeListsProvider for Arc<T>
where
    T: CodeListsProvider,
{
    fn contains(&self, codelist: &str, version: &str, value: &str) -> bool {
        T::contains(self, codelist, version, value)
    }
}

pub trait CodeListValue {
    const CODELIST: &str;

    /// Version the value belongs to: the constant for code type 1/2, the
    /// transmitted `listVersionID` for code type 3.
    fn version(&self) -> &str;

    fn validate(&self, codelists: &impl CodeListsProvider) -> bool {
        codelists.contains(Self::CODELIST, self.version(), self.as_value())
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

        fn version(&self) -> &str {
            "5"
        }

        fn as_value(&self) -> &str {
            self.as_ref()
        }
    }

    #[test]
    fn test_data_set() -> anyhow::Result<()> {
        let source = map::<crate::v1_0_0::Source>()?;

        assert_eq!(source.len(), 71);

        assert!("10005-2".validate(source.as_ref()));
        assert!(!"qqq".validate(source.as_ref()));

        Ok(())
    }
}
