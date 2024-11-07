use std::sync::Arc;

use serde::{Deserialize, Serialize};

pub trait DataSource {
    fn name() -> &'static str;
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct Description {
    pub short_name: Arc<str>,
    pub codelist_description: Arc<str>,
    pub agency_short_name: Arc<str>,
    pub version_handbook: Arc<str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct Identification {
    pub short_name: Arc<str>,
    pub long_name: Arc<str>,
    pub version: Arc<str>,
    pub canonical_uri: Arc<str>,
    pub canonical_version_uri: Arc<str>,
    pub agency_long_name: Arc<str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub enum FieldType {
    RecommendedKey,
    Key,
    Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub enum Usage {
    Optional,
    Required,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct Field {
    pub field_type: FieldType,
    // pub value_type: ValueType,
    pub id: Arc<str>,
    pub usage: Usage,
    pub short_name: Arc<str>,
    pub lang: Option<Arc<str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct Header {
    pub identification: Identification,
    pub description: Description,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct CodeList {
    pub header: Header,
    pub values: Arc<[Arc<[Arc<str>]>]>,
}

impl DataSource for CodeList {
    fn name() -> &'static str {
        "cl"
    }
}
