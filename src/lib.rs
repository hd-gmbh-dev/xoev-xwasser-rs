use raxb::quick_xml::events::Event;

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
    Unknown,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::V0_5_2 => write!(f, "052"),
            Version::V0_5_3 => write!(f, "053"),
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
                                },
                                b"0.5.3" => {
                                    return Version::V0_5_3;
                                },
                                _ => {
                                    return Version::Unknown;
                                }
                            }
                        }
                    }
                },
                Event::Decl(_) => {},
                _ => break,
            }
            Err(_) => {
                return Version::Unknown;
            }
        }
    }
    Version::Unknown
}