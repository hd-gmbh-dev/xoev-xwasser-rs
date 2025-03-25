use raxb::quick_xml::events::Event;

#[cfg(feature = "builder")]
pub mod builder;
pub mod model;
#[cfg(not(feature = "wasm"))]
#[cfg(feature = "schema")]
pub mod schemas;
#[cfg(feature = "validate")]
mod validate;
#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(feature = "validate")]
pub use xoev_xwasser_codelists::{self as codelists, CodeListValue, CodeLists, CodeListsProvider};

#[cfg(feature = "validate")]
pub use validate::{XWasserValidate, XWasserValidateError, XWasserValidateMarker};

#[derive(Debug, PartialEq, Eq)]
pub enum Version {
    V0_5_2,
    V0_5_3,
    V0_6_0,
    V0_7_0,
    V0_7_2,
    V0_8_0,
    V0_9_0,
    V0_9_1,
    Unknown,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::V0_5_2 => write!(f, "052"),
            Version::V0_5_3 => write!(f, "053"),
            Version::V0_6_0 => write!(f, "060"),
            Version::V0_7_0 => write!(f, "070"),
            Version::V0_7_2 => write!(f, "072"),
            Version::V0_8_0 => write!(f, "080"),
            Version::V0_9_0 => write!(f, "090"),
            Version::V0_9_1 => write!(f, "091"),
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
                                b"0.7.2" => {
                                    return Version::V0_7_2;
                                }
                                b"0.8.0" => {
                                    return Version::V0_8_0;
                                }
                                b"0.9.0" => {
                                    return Version::V0_9_0;
                                }
                                b"0.9.1" => {
                                    return Version::V0_9_1;
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
