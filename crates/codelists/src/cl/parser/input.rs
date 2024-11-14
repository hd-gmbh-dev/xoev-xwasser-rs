use raxb::XmlDeserialize;
use std::sync::Arc;

#[derive(Default, XmlDeserialize)]
pub struct StringValue {
    #[xml(ty = "text")]
    pub content: Option<String>,
}

impl StringValue {
    pub fn new(value: String) -> Self {
        Self {
            content: Some(value),
        }
    }
}

impl From<StringValue> for String {
    fn from(val: StringValue) -> Self {
        val.content.unwrap_or_default()
    }
}

impl From<StringValue> for Arc<str> {
    fn from(val: StringValue) -> Self {
        Arc::from(val.content.unwrap_or_default())
    }
}

impl std::fmt::Display for StringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content.as_deref().unwrap_or("").trim())
    }
}

impl std::fmt::Debug for StringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.content.as_deref().unwrap_or("").trim())
    }
}

#[derive(Debug, XmlDeserialize)]
pub struct Description {
    #[xml(default, ns = b"xoev-cl-4", name = b"shortName", ty = "child")]
    pub short_name: StringValue,
    #[xml(
        default,
        ns = b"xoev-cl-4",
        name = b"codelistDescription",
        ty = "child"
    )]
    pub codelist_description: StringValue,
    #[xml(default, ns = b"xoev-cl-4", name = b"agencyShortName", ty = "child")]
    pub agency_short_name: StringValue,
    #[xml(default, ns = b"xoev-cl-4", name = b"versionHandbook", ty = "child")]
    pub version_handbook: StringValue,
}

#[derive(Debug, XmlDeserialize)]
pub struct AppInfo {
    #[xml(ns = b"xoev-cl-4", name = b"recommendedKeyColumn", ty = "sfc")]
    pub recommended_key_column: Option<bool>,
}

#[derive(Debug, XmlDeserialize)]
pub struct Annotation {
    #[xml(name = b"Description", ty = "child")]
    pub description: Option<Description>,
    #[xml(name = b"AppInfo", ty = "child")]
    pub app_info: Option<AppInfo>,
}

#[derive(Debug, XmlDeserialize)]
pub struct Agency {
    #[xml(name = b"LongName", ty = "child")]
    pub long_name: Option<StringValue>,
}

#[derive(Debug, XmlDeserialize)]
pub struct Identification {
    #[xml(name = b"ShortName", ty = "child")]
    pub short_name: Option<StringValue>,
    #[xml(name = b"LongName", ty = "child")]
    pub long_name: Option<StringValue>,
    #[xml(name = b"Version", ty = "child")]
    pub version: Option<StringValue>,
    #[xml(name = b"CanonicalUri", ty = "child")]
    pub canonical_uri: Option<StringValue>,
    #[xml(name = b"CanonicalVersionUri", ty = "child")]
    pub canonical_version_uri: Option<StringValue>,
    #[xml(name = b"Agency", ty = "child")]
    pub agency: Option<Agency>,
}

#[derive(Debug, XmlDeserialize)]
pub struct Data {
    #[xml(name = b"Type", ty = "attr")]
    pub _ty: Option<String>,
    #[xml(name = b"Lang", ty = "attr")]
    pub lang: Option<String>,
}

#[derive(Debug, XmlDeserialize)]
pub struct Column {
    #[xml(name = b"Id", ty = "attr")]
    pub id: Option<String>,
    #[xml(name = b"Use", ty = "attr")]
    pub usage: Option<String>,
    #[xml(name = b"ShortName", ty = "child")]
    pub short_name: Option<StringValue>,
    #[xml(name = b"Data", ty = "sfc")]
    pub data: Option<Data>,
}

#[derive(Debug, XmlDeserialize)]
pub struct ColumnRef {
    #[xml(name = b"Ref", ty = "attr")]
    pub reference: Option<String>,
}

#[derive(Debug, XmlDeserialize)]
pub struct Key {
    #[xml(name = b"Id", ty = "attr")]
    pub _id: Option<String>,
    #[xml(name = b"Annotation", ty = "child")]
    pub annotation: Option<Annotation>,
    #[xml(name = b"ShortName", ty = "child")]
    pub _short_name: Option<StringValue>,
    #[xml(name = b"ColumnRef", ty = "sfc")]
    pub column_ref: Option<ColumnRef>,
}

#[derive(Debug, XmlDeserialize)]
pub struct ColumnSet {
    #[xml(name = b"Column", ty = "child")]
    pub columns: Vec<Column>,
    #[xml(name = b"Key", ty = "child")]
    pub keys: Vec<Key>,
}

#[derive(Debug, XmlDeserialize)]
pub struct Value {
    #[xml(name = b"ColumnRef", ty = "attr")]
    pub column_ref: Option<String>,
    #[xml(name = b"SimpleValue", ty = "child")]
    pub simple_value: Option<StringValue>,
}

#[derive(Debug, XmlDeserialize)]
pub struct Row {
    #[xml(name = b"Value", ty = "child")]
    pub values: Vec<Value>,
}

#[derive(Debug, XmlDeserialize)]
pub struct SimpleCodeList {
    #[xml(name = b"Row", ty = "child")]
    pub rows: Vec<Row>,
}

#[derive(Debug, XmlDeserialize)]
#[xml(root = b"CodeList")]
#[xml(tns(b"gc", b"http://docs.oasis-open.org/codelist/ns/genericode/1.0/"))]
pub struct CodeList {
    #[xml(name = b"Annotation", ty = "child")]
    pub annotation: Annotation,
    #[xml(name = b"Identification", ty = "child")]
    pub identification: Identification,
    #[xml(name = b"ColumnSet", ty = "child")]
    pub column_set: ColumnSet,
    #[xml(name = b"SimpleCodeList", ty = "child")]
    pub simple_code_list: SimpleCodeList,
}
