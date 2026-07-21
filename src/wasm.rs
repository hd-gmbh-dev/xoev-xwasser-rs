use serde::Deserialize;
use wasm_bindgen::prelude::*;

use crate::transform;
use crate::model::{
    administration::AdministrationQuittung0020, transport::VorgangTransportieren2010,
};

/// Returns the XML namespace used in the XML documents.
#[wasm_bindgen]
pub fn xmlns() -> String {
    crate::XMLNS.into()
}

/// Returns the XML schema used in the XML documents.
#[wasm_bindgen]
pub fn schema() -> String {
    crate::SCHEMA.into()
}

/// Returns the local XML schema used in the XML documents.
#[wasm_bindgen]
pub fn local_schema() -> String {
    crate::LOCAL_SCHEMA.into()
}

/// Returns the current version of the XML schema used in the XML documents.
#[wasm_bindgen]
pub fn version() -> String {
    crate::VERSION.into()
}

#[wasm_bindgen]
pub fn detect_version(xml: String) -> Result<String, JsValue> {
    Ok(crate::detect_version(&xml).to_string())
}
/// Helper struct to deserialize the options parameter.
#[derive(Deserialize, Default)]
struct TransformOptionsParam {
    leser: Option<ElementParam>,
    autor: Option<ElementParam>,
    #[serde(default)]
    authorities: Vec<AuthorityParam>,
}

#[derive(Deserialize, Default)]
struct ElementParam {
    kennung: Option<String>,
    name: Option<String>,
}

#[derive(Deserialize)]
struct AuthorityParam {
    kennung: Option<String>,
    name: Option<String>,
}

/// Transforms an XML string by mutating `<leser>`, `<autor>`, and/or
/// `<zustaendigeBehoerde>` elements in-place, preserving all comments,
/// whitespace, and attribute order.
///
/// Accepts a plain JS options object:
/// ```ts
/// transformXml(xml, {
///   leser?: { kennung?: string, name?: string },
///   autor?: { kennung?: string, name?: string },
///   authorities?: Array<{ kennung?: string, name?: string }>,
/// })
/// ```
#[wasm_bindgen(js_name = transformXml)]
pub fn transform_xml(xml: String, options: Option<JsValue>) -> String {
    let opts: TransformOptionsParam = options
        .and_then(|v| {
            if v.is_null() || v.is_undefined() {
                None
            } else {
                serde_wasm_bindgen::from_value(v).ok()
            }
        })
        .unwrap_or_default();

    let leser = opts.leser.map(|p| transform::ElementUpdate {
        kennung: p.kennung,
        name: p.name,
    });
    let autor = opts.autor.map(|p| transform::ElementUpdate {
        kennung: p.kennung,
        name: p.name,
    });
    let authorities: Vec<transform::AuthorityUpdate> = opts
        .authorities
        .into_iter()
        .map(|a| transform::AuthorityUpdate {
            kennung: a.kennung,
            name: a.name,
        })
        .collect();

    transform::transform_xml(
        &xml,
        &transform::TransformOptions {
            leser,
            autor,
            authorities: &authorities,
        },
    )
}

#[wasm_bindgen]
pub fn create_vorgang_transportieren_2010(
    data: VorgangTransportieren2010,
) -> Result<String, JsValue> {
    raxb::ser::to_string_pretty_with_decl(&data).map_err(|err| JsValue::from_str(&err.to_string()))
}

#[wasm_bindgen]
pub fn parse_vorgang_transportieren_2010(
    xml: String,
) -> Result<VorgangTransportieren2010, JsValue> {
    use raxb::quick_xml::NsReader;

    let mut rdr = NsReader::from_str(&xml);
    rdr.config_mut().trim_text(true);
    raxb::de::deserialize_with_reader(rdr).map_err(|err| JsValue::from_str(&err.to_string()))
}

#[wasm_bindgen]
pub fn create_administration_quittung_0020(
    data: AdministrationQuittung0020,
) -> Result<String, JsValue> {
    raxb::ser::to_string_pretty_with_decl(&data).map_err(|err| JsValue::from_str(&err.to_string()))
}

#[wasm_bindgen]
pub fn parse_administration_quittung_0020(
    xml: String,
) -> Result<AdministrationQuittung0020, JsValue> {
    use raxb::quick_xml::NsReader;

    let mut rdr = NsReader::from_str(&xml);
    rdr.config_mut().trim_text(true);
    raxb::de::deserialize_with_reader(rdr).map_err(|err| JsValue::from_str(&err.to_string()))
}
