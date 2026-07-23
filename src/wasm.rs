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

/// Options targeting the `<nachrichtenkopf.g2g>` header block.
#[derive(Deserialize, Default, Tsify)]
#[tsify(from_wasm_abi)]
pub struct NachrichtenkopfG2gOptionsParam {
    #[tsify(optional)]
    pub leser: Option<ElementParam>,
    #[tsify(optional)]
    pub autor: Option<ElementParam>,
    #[tsify(optional)]
    pub nachrichten_uuid: Option<String>,
}

/// Options targeting the `<zusatzinformationen>` extra-info block.
#[derive(Deserialize, Default, Tsify)]
#[tsify(from_wasm_abi)]
pub struct ZusatzinformationenOptionsParam {
    #[tsify(optional)]
    pub zustaendige_behoerde_id: Option<Vec<String>>,
}

/// Helper struct to deserialize the options parameter.
///
/// The structure mirrors the XML document: a header group
/// (`nachrichtenkopf_g2g`) and a `zusatzinformationen` group, so each field's
/// destination is explicit and the `zusatzinformationen` group can grow later
/// (e.g. a `kommentar` update) without reshaping the top-level shape.
#[derive(Deserialize, Default, Tsify)]
#[tsify(from_wasm_abi)]
pub struct TransformOptionsParam {
    #[tsify(optional)]
    pub nachrichtenkopf_g2g: Option<NachrichtenkopfG2gOptionsParam>,
    #[tsify(optional)]
    pub zusatzinformationen: Option<ZusatzinformationenOptionsParam>,
}

#[derive(Deserialize, Default, Tsify)]
#[tsify(from_wasm_abi)]
pub struct ElementParam {
    #[tsify(optional)]
    pub kennung: Option<String>,
    #[tsify(optional)]
    pub name: Option<String>,
}

/// Transforms an XML string by mutating `<leser>`, `<autor>`,
/// `<nachrichtenUUID>`, and/or `<zusatzinformationen>` elements in-place,
/// preserving all comments, whitespace, and attribute order.
///
/// Accepts a plain JS options object whose shape mirrors the XML document:
/// ```ts
/// transform_vorgang_transportieren_2010(xml, {
///   nachrichtenkopf_g2g?: {
///     leser?: { kennung?: string, name?: string },
///     autor?: { kennung?: string, name?: string },
///     nachrichten_uuid?: string,
///   },
///   zusatzinformationen?: {
///     zustaendige_behoerde_id?: Array<string>,
///   },
/// })
/// ```
#[wasm_bindgen]
pub fn transform_vorgang_transportieren_2010(
    xml: String,
    opts: Option<TransformOptionsParam>,
) -> String {
    let opts = opts.unwrap_or_default();

    // Destructure so the owned `nachrichten_uuid` / `zustaendige_behoerde_id`
    // live as locals that the borrowed `TransformOptions` can reference.
    let (leser, autor, nachrichten_uuid) = match opts.nachrichtenkopf_g2g {
        Some(h) => {
            let NachrichtenkopfG2gOptionsParam {
                leser,
                autor,
                nachrichten_uuid,
            } = h;
            (
                leser.map(|p| transform::ElementUpdate {
                    kennung: p.kennung,
                    name: p.name,
                }),
                autor.map(|p| transform::ElementUpdate {
                    kennung: p.kennung,
                    name: p.name,
                }),
                nachrichten_uuid,
            )
        }
        None => (None, None, None),
    };

    let zustaendige_behoerde_ids = opts
        .zusatzinformationen
        .and_then(|z| z.zustaendige_behoerde_id);

    let opts_struct = transform::TransformOptions {
        nachrichtenkopf_g2g: Some(transform::NachrichtenkopfG2gOptions {
            leser,
            autor,
            nachrichten_uuid: nachrichten_uuid.as_deref(),
        }),
        zusatzinformationen: zustaendige_behoerde_ids
            .as_deref()
            .filter(|v| !v.is_empty())
            .map(|ids| transform::ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(ids),
            }),
    };
    transform::transform_vorgang_transportieren_2010_impl(&xml, &opts_struct)
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
