use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput};

#[proc_macro_attribute]
pub fn xoev_xwasser_code(attr: TokenStream, item: TokenStream) -> TokenStream {
    let types: Vec<syn::LitStr> =
        parse_macro_input!(attr with Punctuated::<syn::LitStr, syn::Token![,]>::parse_terminated)
            .into_iter()
            .collect();
    let mut iter = types.iter();
    let uri = iter.next().unwrap();
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    if let Some(version) = iter.next() {
        quote! {
            #[derive(Clone, Default, Debug, raxb::XmlSerialize, raxb::XmlDeserialize, serde::Serialize, serde::Deserialize)]
            #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
            #[xml(tns(
                b"xwas",
                b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_6_0/"
            ))]
            pub struct #name {
                #[xml(name = b"code", ty = "child")]
                pub code: String,
                #[xml(name = b"name", ty = "child")]
                pub name: Option<String>,
                #[serde(skip)]
                #[xml(
                    default,
                    name = b"listURI",
                    ty = "attr",
                    value = #uri
                )]
                _list_uri: ConstStr,
                #[serde(skip)]
                #[xml(
                    default,
                    name = b"listVersionID",
                    ty = "attr",
                    value = #version
                )]
                _list_version_id: ConstStr,
            }
        }
    } else {
        quote! {
            #[derive(Clone, Default, Debug, raxb::XmlSerialize, raxb::XmlDeserialize, serde::Serialize, serde::Deserialize)]
            #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
            #[xml(tns(
                b"xwas",
                b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_6_0/"
            ))]
            pub struct #name {
                #[xml(name = b"code", ty = "child")]
                pub code: String,
                #[xml(name = b"name", ty = "child")]
                pub name: Option<String>,
                #[serde(skip)]
                #[xml(
                    default,
                    name = b"listURI",
                    ty = "attr",
                    value = #uri
                )]
                _list_uri: ConstStr,
                #[serde(skip)]
                #[xml(
                    default,
                    name = b"listVersionID",
                    ty = "attr",
                    value = ""
                )]
                _list_version_id: ConstStr,
            }
        }
    }.into()
}

#[proc_macro_attribute]
pub fn xoev_xwasser_code_with_version(attr: TokenStream, item: TokenStream) -> TokenStream {
    let types: Vec<syn::LitStr> =
        parse_macro_input!(attr with Punctuated::<syn::LitStr, syn::Token![,]>::parse_terminated)
            .into_iter()
            .collect();
    let mut iter = types.iter();
    let uri = iter.next().unwrap();
    let version = iter.next().unwrap();
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    quote! {
        #[derive(Default, Debug, raxb::XmlSerialize, raxb::XmlDeserialize, serde::Serialize, serde::Deserialize)]
        #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
        #[xml(tns(
            b"xwas",
            b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_6_0/"
        ))]
        pub struct #name {
            #[xml(name = b"code", ty = "child")]
            pub code: String,
            #[xml(name = b"name", ty = "child")]
            pub name: Option<String>,
            #[xml(
                default,
                name = b"listURI",
                ty = "attr",
                value = #uri
            )]
            _list_uri: ConstStr,
            #[xml(
                default,
                name = b"listVersionID",
                ty = "attr",
                value = #version
            )]
            _list_version_id: ConstStr,
        }
    }.into()
}
