use xoev_xwasser_codelists::{CodeLists, XWasserCodeListValue};

pub trait XWasserValidate {
    fn xwasser_validate(&self, codelists: &CodeLists) -> Result<(), XWasserValidateError>;
}

pub enum XWasserValidateError {
    CodeListValueNotFound { codelist: String, value: String },
}

pub trait XWasserValidateMarker {}

impl<T> XWasserValidate for T
where
    T: XWasserCodeListValue + XWasserValidateMarker,
{
    fn xwasser_validate(&self, codelists: &CodeLists) -> Result<(), XWasserValidateError> {
        if self.validate(codelists) {
            Ok(())
        } else {
            Err(XWasserValidateError::CodeListValueNotFound {
                codelist: Self::CODELIST.to_string(),
                value: self.as_value().to_string(),
            })
        }
    }
}

impl<T> XWasserValidate for Option<T>
where
    T: XWasserValidate,
{
    fn xwasser_validate(&self, codelists: &CodeLists) -> Result<(), XWasserValidateError> {
        self.as_ref()
            .map(|t| t.xwasser_validate(codelists))
            .unwrap_or(Ok(()))
    }
}

impl<T> XWasserValidate for Vec<T>
where
    T: XWasserValidate,
{
    fn xwasser_validate(&self, codelists: &CodeLists) -> Result<(), XWasserValidateError> {
        self.iter()
            .find_map(|t| t.xwasser_validate(codelists).err())
            .map(Err)
            .unwrap_or(Ok(()))
    }
}

impl XWasserValidate for raxb::value::ConstStr {
    fn xwasser_validate(&self, _: &CodeLists) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for String {
    fn xwasser_validate(&self, _: &CodeLists) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for bool {
    fn xwasser_validate(&self, _: &CodeLists) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for isize {
    fn xwasser_validate(&self, _: &CodeLists) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for u64 {
    fn xwasser_validate(&self, _: &CodeLists) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for u32 {
    fn xwasser_validate(&self, _: &CodeLists) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for u16 {
    fn xwasser_validate(&self, _: &CodeLists) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for u8 {
    fn xwasser_validate(&self, _: &CodeLists) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for i64 {
    fn xwasser_validate(&self, _: &CodeLists) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for i32 {
    fn xwasser_validate(&self, _: &CodeLists) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for i16 {
    fn xwasser_validate(&self, _: &CodeLists) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for i8 {
    fn xwasser_validate(&self, _: &CodeLists) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for usize {
    fn xwasser_validate(&self, _: &CodeLists) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for f64 {
    fn xwasser_validate(&self, _: &CodeLists) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}

impl XWasserValidate for () {
    fn xwasser_validate(&self, _: &CodeLists) -> Result<(), XWasserValidateError> {
        Ok(())
    }
}
