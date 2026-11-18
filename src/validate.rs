use std::fmt;

use xoev_xwasser_codelists::{CodeListValue, CodeListsProvider};

pub trait XWasserValidate {
    fn xwasser_validate(
        &self,
        codelists: &impl CodeListsProvider,
    ) -> Result<(), XWasserValidateError>;
}

/// Why a message failed the codelist check.
///
/// `#[non_exhaustive]`: a receiver that does not know a list yet will get its own
/// variant, and that must not be a breaking change.
#[derive(Debug)]
#[non_exhaustive]
pub enum XWasserValidateError {
    /// The version is the one the message claimed: a value can be valid in one
    /// published version and unknown in another, and a version can be missing
    /// from the codelist altogether.
    CodeListValueNotFound {
        codelist: String,
        value: String,
        version: String,
    },
}

impl fmt::Display for XWasserValidateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CodeListValueNotFound {
                codelist,
                value,
                version,
            } => write!(
                f,
                "value {value} is not in codelist {codelist} version {version}"
            ),
        }
    }
}

impl std::error::Error for XWasserValidateError {}

pub trait XWasserValidateMarker {}

impl<T> XWasserValidate for T
where
    T: CodeListValue + XWasserValidateMarker,
{
    fn xwasser_validate(
        &self,
        codelists: &impl CodeListsProvider,
    ) -> Result<(), XWasserValidateError> {
        if self.validate(codelists) {
            Ok(())
        } else {
            Err(XWasserValidateError::CodeListValueNotFound {
                codelist: Self::CODELIST.to_string(),
                value: self.as_value().to_string(),
                version: self.version().to_string(),
            })
        }
    }
}

impl<T> XWasserValidate for Option<T>
where
    T: XWasserValidate,
{
    fn xwasser_validate(
        &self,
        codelists: &impl CodeListsProvider,
    ) -> Result<(), XWasserValidateError> {
        self.as_ref()
            .map(|t| t.xwasser_validate(codelists))
            .unwrap_or(Ok(()))
    }
}

impl<T> XWasserValidate for Vec<T>
where
    T: XWasserValidate,
{
    fn xwasser_validate(
        &self,
        codelists: &impl CodeListsProvider,
    ) -> Result<(), XWasserValidateError> {
        self.iter()
            .find_map(|t| t.xwasser_validate(codelists).err())
            .map(Err)
            .unwrap_or(Ok(()))
    }
}

impl XWasserValidate for raxb::value::ConstStr {
    fn xwasser_validate(&self, _: &impl CodeListsProvider) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for String {
    fn xwasser_validate(&self, _: &impl CodeListsProvider) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for bool {
    fn xwasser_validate(&self, _: &impl CodeListsProvider) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for isize {
    fn xwasser_validate(&self, _: &impl CodeListsProvider) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for u64 {
    fn xwasser_validate(&self, _: &impl CodeListsProvider) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for u32 {
    fn xwasser_validate(&self, _: &impl CodeListsProvider) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for u16 {
    fn xwasser_validate(&self, _: &impl CodeListsProvider) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for u8 {
    fn xwasser_validate(&self, _: &impl CodeListsProvider) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for i64 {
    fn xwasser_validate(&self, _: &impl CodeListsProvider) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for i32 {
    fn xwasser_validate(&self, _: &impl CodeListsProvider) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for i16 {
    fn xwasser_validate(&self, _: &impl CodeListsProvider) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for i8 {
    fn xwasser_validate(&self, _: &impl CodeListsProvider) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for usize {
    fn xwasser_validate(&self, _: &impl CodeListsProvider) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for f64 {
    fn xwasser_validate(&self, _: &impl CodeListsProvider) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for () {
    fn xwasser_validate(&self, _: &impl CodeListsProvider) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}
