use proc_macro::TokenStream;
use quote::quote;

mod xwasser_validate;

#[proc_macro_derive(XWasserValidate)]
pub fn xwasser_validate_derive(input: TokenStream) -> TokenStream {
    xwasser_validate::derive(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn xoev_xwasser_code(attr: TokenStream, item: TokenStream) -> TokenStream {
    xoev_xwasser_code2(attr.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn xoev_xwasser_code2(
    attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let XoevXWasserCodeArgs {
        uri,
        version,
        validate,
    }: XoevXWasserCodeArgs = xoev_xwasser_code_args(attr)?;
    let ast: syn::DeriveInput = syn::parse2(item)?;
    let name = ast.ident;
    let version = version.unwrap_or_default();
    let validation = if validate {
        None
    } else {
        Some(quote! {
            fn validate(&self, _: &impl crate::CodeListsProvider) -> bool {
                true
            }
        })
    };
    Ok(quote! {
        #[derive(Clone, Default, Debug, raxb::XmlSerialize, raxb::XmlDeserialize, serde::Serialize, serde::Deserialize)]
        #[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
        #[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
        #[cfg_attr(feature = "builder", derive(TypedBuilder))]
        #[xml(tns(
            b"xwas",
            b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_7_0/"
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
            #[cfg_attr(feature = "builder", builder(default))]
            _list_uri: ConstStr,
            #[serde(skip)]
            #[xml(
                default,
                name = b"listVersionID",
                ty = "attr",
                value = #version
            )]
            #[cfg_attr(feature = "builder", builder(default))]
            _list_version_id: ConstStr,
        }


        impl <S> From<S> for #name where S: Into<String> {
            fn from(val: S) -> Self {
                Self {
                    code: val.into(),
                    ..Default::default()
                }
            }
        }

        #[cfg(feature = "validate")]
        impl crate::CodeListValue for #name {
            const CODELIST: &str = #uri;
            #validation
            fn as_value(&self) -> &str {
                &self.code
            }
        }

        #[cfg(feature = "validate")]
        impl crate::XWasserValidateMarker for #name {}

    })
}

fn xoev_xwasser_code_args(attr: proc_macro2::TokenStream) -> syn::Result<XoevXWasserCodeArgs> {
    syn::parse2(attr)
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
struct XoevXWasserCodeArgs {
    uri: String,
    version: Option<String>,
    validate: bool,
}

impl syn::parse::Parse for XoevXWasserCodeArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let uri = input.parse().map(|litstr: syn::LitStr| litstr.value())?;
        if !input.is_empty() {
            let _: syn::token::Comma = input.parse()?;
        }
        let mut version = None;
        let mut validate = false;
        if !input.is_empty() {
            let head = input.fork();
            let lit: syn::Lit = input.parse()?;
            match lit {
                syn::Lit::Str(lit_str) => version = Some(lit_str.value()),
                syn::Lit::Bool(lit_bool) => validate = lit_bool.value(),
                _ => return Err(head.error("expected either string or bool")),
            }
            if !input.is_empty() {
                let _: syn::token::Comma = input.parse()?;
            }
        }
        if !input.is_empty() {
            let lit_bool: syn::LitBool = input.parse()?;
            validate = lit_bool.value();
            if !input.is_empty() {
                let _: syn::token::Comma = input.parse()?;
            }
        }

        Ok(Self {
            uri,
            version,
            validate,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn xoev_xwasser_validate_case(attr: proc_macro2::TokenStream) -> XoevXWasserCodeArgs {
        xoev_xwasser_code_args(attr).expect("XoevXWasserValidateArgs")
    }

    #[test]
    fn xoev_xwasser_validate_should_parse_valid_attr() {
        for (case, expected) in [
            (
                quote! { "abc" },
                XoevXWasserCodeArgs {
                    uri: "abc".into(),
                    version: None,
                    validate: false,
                },
            ),
            (
                quote! { "abc", },
                XoevXWasserCodeArgs {
                    uri: "abc".into(),
                    version: None,
                    validate: false,
                },
            ),
            (
                quote! { "abc", "def" },
                XoevXWasserCodeArgs {
                    uri: "abc".into(),
                    version: Some("def".into()),
                    validate: false,
                },
            ),
            (
                quote! { "abc", "def", },
                XoevXWasserCodeArgs {
                    uri: "abc".into(),
                    version: Some("def".into()),
                    validate: false,
                },
            ),
            (
                quote! { "abc", true },
                XoevXWasserCodeArgs {
                    uri: "abc".into(),
                    version: None,
                    validate: true,
                },
            ),
            (
                quote! { "abc", false, },
                XoevXWasserCodeArgs {
                    uri: "abc".into(),
                    version: None,
                    validate: false,
                },
            ),
            (
                quote! { "abc", "def", true },
                XoevXWasserCodeArgs {
                    uri: "abc".into(),
                    version: Some("def".into()),
                    validate: true,
                },
            ),
            (
                quote! { "abc", "def", false, },
                XoevXWasserCodeArgs {
                    uri: "abc".into(),
                    version: Some("def".into()),
                    validate: false,
                },
            ),
        ] {
            assert_eq!(xoev_xwasser_validate_case(case), expected);
        }
    }
}
