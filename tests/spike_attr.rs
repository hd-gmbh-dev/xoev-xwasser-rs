use raxb::value::ConstStr;
use raxb::{XmlDeserialize, XmlSerialize};

#[derive(Debug, Default, XmlSerialize, XmlDeserialize)]
#[raxb(root = b"wrapper")]
pub struct Wrapper {
    #[raxb(name = b"untersuchungsverfahren", ty = "child")]
    pub codes: Vec<CodeLike>,
}

#[derive(Debug, Default, XmlSerialize, XmlDeserialize)]
pub struct CodeLike {
    #[raxb(name = b"code", ty = "child")]
    pub code: String,
    #[raxb(name = b"listURI", ty = "attr", value = "urn:xoev-de:xwasser:codeliste:untersuchungsverfahren")]
    pub list_uri: ConstStr,
    #[raxb(name = b"listVersionID", ty = "attr")]
    pub list_version_id: String,
}

#[test]
fn runtime_attribute_round_trip() -> anyhow::Result<()> {
    let xml = r#"<wrapper><untersuchungsverfahren listURI="urn:xoev-de:xwasser:codeliste:untersuchungsverfahren" listVersionID="1"><code>1760</code></untersuchungsverfahren></wrapper>"#;
    let parsed: Wrapper = raxb::de::from_str(xml)?;
    println!("parsed version: {:?}", parsed.codes[0].list_version_id);
    let out = raxb::ser::to_string(&parsed)?;
    println!("reserialized: {out}");
    assert_eq!(parsed.codes[0].list_version_id, "1");
    assert!(out.contains(r#"listVersionID="1""#));
    Ok(())
}
