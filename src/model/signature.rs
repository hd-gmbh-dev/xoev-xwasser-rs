use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};
use xoev_xwasser_derive::XWasserValidate;

#[cfg(feature = "wasm")]
use tsify_next::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct Signature {
    #[xml(ns = b"ds", name = b"SignedInfo", ty = "child")]
    pub signed_info: SignedInfo,
    #[xml(ns = b"ds", name = b"SignatureValue", ty = "child")]
    pub signature_value: SignatureValue,
    #[xml(ns = b"ds", name = b"KeyInfo", ty = "child")]
    pub key_info: Option<KeyInfo>,
    #[xml(ns = b"ds", name = b"Object", ty = "child")]
    pub object: Vec<Object>,
    #[xml(name = b"Id", ty = "attr")]
    pub id: Option<String>,
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct SignedInfo {
    #[xml(ns = b"ds", name = b"CanonicalizationMethod", ty = "child")]
    pub canonicalization_method: CanonicalizationMethod,
    #[xml(ns = b"ds", name = b"SignatureMethod", ty = "child")]
    pub signature_method: SignatureMethod,
    #[xml(ns = b"ds", name = b"Reference", ty = "child")]
    pub reference: Vec<Reference>,
    #[xml(name = b"Id", ty = "attr")]
    pub id: Option<String>,
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct SignatureValue {
    #[xml(ty = "text")]
    pub content: String, // base64Binary
    #[xml(name = b"Id", ty = "attr")]
    pub id: Option<String>,
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct CanonicalizationMethod {
    #[xml(name = b"Algorithm", ty = "attr")]
    pub algorithm: String,
    // #[xml(any, ty = "child")]
    // pub any: Option<Vec<Any>>,
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct SignatureMethod {
    #[xml(ns = b"ds", name = b"HMACOutputLength", ty = "child")]
    pub hmac_output_length: Option<i64>,
    #[xml(name = b"Algorithm", ty = "attr")]
    pub algorithm: String,
    // #[xml(any, ty = "child")]
    // pub any: Option<Vec<Any>>,
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct Reference {
    #[xml(ns = b"ds", name = b"Transforms", ty = "child")]
    pub transforms: Option<Transforms>,
    #[xml(ns = b"ds", name = b"DigestMethod", ty = "child")]
    pub digest_method: DigestMethod,
    #[xml(ns = b"ds", name = b"DigestValue", ty = "child")]
    pub digest_value: String,
    // pub digest_value: DigestValue,
    #[xml(name = b"Id", ty = "attr")]
    pub id: Option<String>,
    #[xml(name = b"URI", ty = "attr")]
    pub uri: Option<String>,
    #[xml(name = b"Type", ty = "attr")]
    pub type_attr: Option<String>, // anyURI
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct Transforms {
    #[xml(ns = b"ds", name = b"Transform", ty = "child")]
    pub transform: Vec<Transform>,
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct Transform {
    #[xml(ns = b"ds", name = b"XPath", ty = "child")]
    pub xpath: Vec<String>,
    // pub choice: Option<Vec<TransformChoice>>,
    #[xml(name = b"Algorithm", ty = "attr")]
    pub algorithm: String,
}

/*
#[derive(
   Clone, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
#[serde(tag = "t", content = "c")]
pub enum TransformChoice {
    #[xml(ns = b"ds", name = b"XPath")]
    XPath(String),
    #[xml(ns = b"ds", name = b"")]
    Any(Any),
}
*/

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct DigestMethod {
    // #[xml(any, ty = "child")]
    // pub any: Option<Vec<Any>>,
    #[xml(name = b"Algorithm", ty = "attr")]
    pub algorithm: String,
}

// #[derive(
//    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
//)]
// #[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
// pub struct DigestValue {
//     #[xml(text)]
//     pub value: Vec<u8>, // base64Binary
// }

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct KeyInfo {
    #[xml(ns = b"ds", name = b"KeyName", ty = "child")]
    pub choice: Vec<KeyInfoChoice>,
    #[xml(name = b"Id", ty = "attr")]
    pub id: Option<String>,
}

#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
#[serde(tag = "t", content = "c")]
pub enum KeyInfoChoice {
    #[xml(ns = b"ds", name = b"KeyName")]
    KeyName(String),
    #[xml(ns = b"ds", name = b"KeyValue")]
    KeyValue(KeyValue),
    #[xml(ns = b"ds", name = b"RetrievalMethod")]
    RetrievalMethod(RetrievalMethod),
    #[xml(ns = b"ds", name = b"X509Data")]
    X509Data(X509Data),
    #[xml(ns = b"ds", name = b"PGPData")]
    PGPData(PGPData),
    #[xml(ns = b"ds", name = b"SPKIData")]
    SPKIData(SPKIData),
    #[xml(ns = b"ds", name = b"MgmtData")]
    MgmtData(String),
    // Any(Any),
}

#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct KeyValue {
    #[xml(ns = b"ds", name = b"KeyValueType", ty = "child")]
    pub key_value_choice: KeyValueType,
}

#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
#[serde(tag = "t", content = "c")]
pub enum KeyValueType {
    #[xml(ns = b"ds", name = b"DSA")]
    DSA(DSAKeyValue),
    #[xml(ns = b"ds", name = b"RSA")]
    RSA(RSAKeyValue),
    // Any(Any),
}

#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct DSAKeyValue {
    #[xml(ns = b"ds", name = b"P", ty = "child")]
    pub p: Option<String>,
    #[xml(ns = b"ds", name = b"Q", ty = "child")]
    pub q: Option<String>,
    #[xml(ns = b"ds", name = b"G", ty = "child")]
    pub g: Option<String>,
    #[xml(ns = b"ds", name = b"Y", ty = "child")]
    pub y: String,
    #[xml(ns = b"ds", name = b"J", ty = "child")]
    pub j: Option<String>,
    #[xml(ns = b"ds", name = b"Seed", ty = "child")]
    pub seed: Option<String>,
    #[xml(ns = b"ds", name = b"PgenCounter", ty = "child")]
    pub pgen_counter: Option<String>,
}

#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct RSAKeyValue {
    #[xml(ns = b"ds", name = b"Modulus", ty = "child")]
    pub modulus: String,
    #[xml(ns = b"ds", name = b"Exponent", ty = "child")]
    pub exponent: String,
}

#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct Object {
    // #[xml(any, ty = "child")]
    // pub any: Option<Vec<Any>>,
    #[xml(name = b"Id", ty = "attr")]
    pub id: Option<String>,
    #[xml(name = b"MimeType", ty = "attr")]
    pub mime_type: Option<String>,
    #[xml(name = b"Encoding", ty = "attr")]
    pub encoding: Option<String>,
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct RetrievalMethod {
    #[xml(ns = b"ds", name = b"Transforms", ty = "child")]
    pub transforms: Option<Transforms>,
    #[xml(name = b"URI", ty = "attr")]
    pub uri: Option<String>,
    #[xml(name = b"Type", ty = "attr")]
    pub type_attr: Option<String>, // anyURI
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct X509Data {
    #[xml(ns = b"ds", name = b"X509DataType", ty = "child")]
    choice: Vec<X509DataType>,
}

#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
#[serde(tag = "t", content = "c")]
pub enum X509DataType {
    #[xml(ns = b"ds", name = b"X509IssuerSerial")]
    X509IssuerSerial(X509IssuerSerial),
    #[xml(ns = b"ds", name = b"X509SKI")]
    X509SKI(String),
    #[xml(ns = b"ds", name = b"X509SubjectName")]
    X509SubjectName(String),
    #[xml(ns = b"ds", name = b"X509Certificate")]
    X509Certificate(String),
    #[xml(ns = b"ds", name = b"X509CRL")]
    X509CRL(String),
    // Any(Any),
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct X509IssuerSerial {
    #[xml(ns = b"ds", name = b"X509IssuerName", ty = "child")]
    pub x509_issuer_name: String,
    #[xml(name = b"URI", name = b"X509SerialNumber", ty = "child")]
    pub x509_serial_number: i64,
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct PGPData {
    #[xml(ns = b"ds", name = b"PGPKeyID", ty = "child")]
    pgp_key_id: Option<String>,
    #[xml(ns = b"ds", name = b"PGPKeyPacket", ty = "child")]
    pgp_key_packet: Option<String>,
    // #[xml(any, ty = "child")]
    // pub any: Option<Vec<Any>>,
}

#[derive(
    Clone, Debug, Default, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"ds", b"http://www.w3.org/2000/09/xmldsig#"))]
pub struct SPKIData {
    #[xml(ns = b"ds", name = b"SPKISexp", ty = "child")]
    spki_sexp: Vec<String>,
    // #[xml(any, ty = "child")]
    // pub any: Option<Vec<Any>>,
}
