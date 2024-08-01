use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn xoev_xwasser_code(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as Literal);
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    quote! {
        #[derive(Default, Debug, raxb::XmlSerialize, raxb::XmlDeserialize, serde::Serialize, serde::Deserialize)]
        #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
        #[xml(tns(
            b"xwas",
            b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
        ))]
        pub struct #name {
            #[xml(name = b"code", ty = "child")]
            pub code: String,
            #[xml(name = b"name", ty = "child")]
            pub name: Option<String>,
            #[xml(
                name = b"listURI",
                ty = "attr",
                value = #attr
            )]
            #[serde(skip)]
            _list_uri: raxb::value::ConstStr,
            #[xml(name = b"listVersionID", ty = "attr")]
            pub list_version_id: String,
        }
    }.into()
}
