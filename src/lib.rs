use std::{collections::HashMap, sync::Arc};

use raxb::quick_xml::events::Event;

pub use xoev_xwasser_codelists::{CodeList, XWasserCodeListValue};

#[cfg(feature = "builder")]
pub mod builder;
pub mod model;
#[cfg(not(feature = "wasm"))]
#[cfg(feature = "schema")]
pub mod schemas;
#[cfg(feature = "wasm")]
pub mod wasm;

#[derive(Debug, PartialEq, Eq)]
pub enum Version {
    V0_5_2,
    V0_5_3,
    V0_6_0,
    V0_7_0,
    Unknown,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::V0_5_2 => write!(f, "052"),
            Version::V0_5_3 => write!(f, "053"),
            Version::V0_6_0 => write!(f, "060"),
            Version::V0_7_0 => write!(f, "070"),
            Version::Unknown => write!(f, ""),
        }
    }
}

pub fn detect_version(xml: &str) -> Version {
    let mut rdr = raxb::quick_xml::Reader::from_str(xml);
    rdr.config_mut().allow_unmatched_ends = true;
    rdr.config_mut().trim_text(true);
    loop {
        match rdr.read_event() {
            Ok(ev) => match ev {
                Event::Start(ev) => {
                    let attrs = ev.attributes();
                    for attr in attrs.flatten() {
                        if attr.key.local_name().as_ref() == b"version" {
                            match &attr.value[..] {
                                b"0.5.2" => {
                                    return Version::V0_5_2;
                                }
                                b"0.5.3" => {
                                    return Version::V0_5_3;
                                }
                                b"0.6.0" => {
                                    return Version::V0_6_0;
                                }
                                b"0.7.0" => {
                                    return Version::V0_7_0;
                                }
                                _ => {
                                    return Version::Unknown;
                                }
                            }
                        }
                    }
                }
                Event::Decl(_) => {}
                _ => break,
            },
            Err(_) => {
                return Version::Unknown;
            }
        }
    }
    Version::Unknown
}

pub trait XWasserValidate {
    fn xwasser_validate(
        &self,
        codelists: &HashMap<Arc<str>, CodeList>,
    ) -> Result<(), XWasserValidateError>;
}

pub enum XWasserValidateError {
    CodeListValueNotFound { codelist: String, value: String },
}

pub trait XWasserValidateMarker {}

impl<T> XWasserValidate for T
where
    T: XWasserCodeListValue + XWasserValidateMarker,
{
    fn xwasser_validate(
        &self,
        codelists: &HashMap<Arc<str>, CodeList>,
    ) -> Result<(), XWasserValidateError> {
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
    fn xwasser_validate(
        &self,
        codelists: &HashMap<Arc<str>, CodeList>,
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
        codelists: &HashMap<Arc<str>, CodeList>,
    ) -> Result<(), XWasserValidateError> {
        self.iter()
            .find_map(|t| t.xwasser_validate(codelists).err())
            .map(Err)
            .unwrap_or(Ok(()))
    }
}
