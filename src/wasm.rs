use serde::Deserialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::model::{
    administration::AdministrationQuittung0020, transport::VorgangTransportieren2010,
};
use crate::transform;

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

// /// Custom TS type definitions injected into .d.ts.
// #[wasm_bindgen(typescript_custom_section)]
// const TS_TRANSFORM_OPTIONS: &'static str = r#"
// export interface TransformOptions {
//   leser?: { kennung?: string; name?: string };
//   autor?: { kennung?: string; name?: string };
//   zusatzinformationen?: Array<string>;
// }

// export function transform_xml(xml: string, options?: TransformOptions): string;
// "#;

/// Custom deserializer for `zusatzinformationen`: accepts an array of
/// `<zustaendigeBehoerdeID>` strings.

/// Helper struct to deserialize the options parameter.
#[derive(Deserialize, Default, Tsify)]
#[tsify(from_wasm_abi)]
pub struct TransformOptionsParam {
    #[tsify(optional)]
    pub leser: Option<ElementParam>,
    #[tsify(optional)]
    pub autor: Option<ElementParam>,
    #[tsify(optional)]
    #[serde(rename = "zusatzinformationen")]
    pub zusatzinformationen: Option<Vec<String>>,
    #[tsify(optional)]
    pub nachrichten_uuid: Option<String>,
}

#[derive(Deserialize, Default, Tsify)]
#[tsify(from_wasm_abi)]
pub struct ElementParam {
    #[tsify(optional)]
    pub kennung: Option<String>,
    #[tsify(optional)]
    pub name: Option<String>,
}

/// Transforms an XML string by mutating `<leser>`, `<autor>`, and/or
/// `<zusatzinformationen>` elements in-place, preserving all comments,
/// whitespace, and attribute order.
///
/// Accepts a plain JS options object:
/// ```ts
/// transform_xml(xml, {
///   leser?: { kennung?: string, name?: string },
///   autor?: { kennung?: string, name?: string },
///   zusatzinformationen?: Array<string>,
/// })
/// ```
#[wasm_bindgen]
pub fn transform_xml(xml: String, opts: Option<TransformOptionsParam>) -> String {
    let opts = opts.unwrap_or_default();
    let leser = opts.leser.as_ref().map(|p| transform::ElementUpdate {
        kennung: p.kennung.clone(),
        name: p.name.clone(),
    });
    let autor = opts.autor.as_ref().map(|p| transform::ElementUpdate {
        kennung: p.kennung.clone(),
        name: p.name.clone(),
    });

    // Use if let to check for non-empty zusatzinformationen
    let opts_struct = transform::TransformOptions {
        leser,
        autor,
        zusatzinformationen: if let Some(ids) = opts.zusatzinformationen.as_ref()
            && !ids.is_empty()
        {
            Some(ids)
        } else {
            None
        },
        nachrichten_uuid: opts.nachrichten_uuid.as_deref(),
    };
    transform::transform_xml_impl(&xml, &opts_struct)
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
