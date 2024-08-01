use wasm_bindgen::prelude::*;

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
    rdr.trim_text(true);
    rdr.check_comments(false);
    rdr.trim_markup_names_in_closing_tags(true);
    raxb::de::deserialize_with_reader(rdr).map_err(|err| JsValue::from_str(&err.to_string()))
}
