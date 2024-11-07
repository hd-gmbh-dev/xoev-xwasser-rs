use std::{cmp::Ordering, sync::Arc};

use unicode_normalization::UnicodeNormalization;

use crate::cl::model::*;

use super::input::StringValue;

fn normalize_str(v: &str) -> String {
    v.trim().nfc().collect()
}

impl From<super::input::Description> for Description {
    fn from(val: super::input::Description) -> Self {
        Description {
            agency_short_name: val.agency_short_name.into(),
            codelist_description: val.codelist_description.into(),
            short_name: val.short_name.into(),
            version_handbook: val.version_handbook.into(),
        }
    }
}

impl From<super::input::Identification> for Identification {
    fn from(val: super::input::Identification) -> Self {
        Identification {
            short_name: val.short_name.unwrap_or_default().into(),
            long_name: val.long_name.unwrap_or_default().into(),
            version: val.version.unwrap_or_default().into(),
            canonical_uri: val.canonical_uri.unwrap_or_default().into(),
            canonical_version_uri: val.canonical_version_uri.unwrap_or_default().into(),
            agency_long_name: val
                .agency
                .and_then(|v| v.long_name)
                .unwrap_or_default()
                .into(),
        }
    }
}

impl From<super::input::Column> for Field {
    fn from(val: super::input::Column) -> Self {
        let lang = val.data.and_then(|v| v.lang).map(Arc::from);
        // let (lang, value_type) = self.data.map(|d| (d.lang, d.ty.map(|ty| match ty.as_str() {
        //     "bool" => ValueType::Bool,
        //     _ => ValueType::String,
        // }).unwrap_or(ValueType::String)))
        // .unwrap_or((None, ValueType::String));
        Field {
            field_type: FieldType::Value,
            // value_type,
            lang,
            id: val.id.map(StringValue::new).unwrap_or_default().into(),
            usage: val
                .usage
                .map(|u| match u.as_str() {
                    "required" => Usage::Required,
                    _ => Usage::Optional,
                })
                .unwrap_or(Usage::Optional),
            short_name: val.short_name.unwrap_or_default().into(),
        }
    }
}

impl From<super::input::CodeList> for CodeList {
    fn from(val: super::input::CodeList) -> Self {
        let mut fields: Vec<Field> = val.column_set.columns.into_iter().map(Into::into).collect();
        for field in fields.iter_mut() {
            if let Some(key) = val.column_set.keys.iter().find(|k| {
                k.column_ref.as_ref().and_then(|v| v.reference.as_deref())
                    == Some(field.id.as_ref())
            }) {
                let is_recommended = key
                    .annotation
                    .as_ref()
                    .and_then(|v| {
                        v.app_info
                            .as_ref()
                            .map(|v| v.recommended_key_column.is_some())
                    })
                    .unwrap_or(false);
                if is_recommended {
                    field.field_type = FieldType::RecommendedKey;
                } else {
                    field.field_type = FieldType::Key;
                }
            }
        }
        fields.sort_by(|a, b| {
            if matches!(a.field_type, FieldType::RecommendedKey)
                || matches!(b.field_type, FieldType::RecommendedKey)
            {
                return Ordering::Greater;
            }
            a.id.cmp(&b.id)
        });
        let mut values = Vec::with_capacity(val.simple_code_list.rows.len());
        for mut row in val.simple_code_list.rows.into_iter() {
            let mut cols = Vec::with_capacity(row.values.len());
            for field in fields.iter() {
                let pos = row
                    .values
                    .iter()
                    .position(|v| v.column_ref.as_deref() == Some(field.id.as_ref()));
                if let Some(pos) = pos {
                    let v = row
                        .values
                        .remove(pos)
                        .simple_value
                        .unwrap_or_default()
                        .content
                        .unwrap_or_default();
                    cols.push(Arc::from(normalize_str(&v)));
                } else {
                    cols.push(Arc::from(""));
                }
            }
            values.push(Arc::from(cols));
        }
        CodeList {
            header: Header {
                identification: val.identification.into(),
                description: val.annotation.description.unwrap().into(),
                fields,
            },
            values: Arc::from(values),
        }
    }
}
