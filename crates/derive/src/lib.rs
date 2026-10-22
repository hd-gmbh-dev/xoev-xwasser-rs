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
        current,
        validate,
    }: XoevXWasserCodeArgs = xoev_xwasser_code_args(attr)?;
    let ast: syn::DeriveInput = syn::parse2(item)?;
    let name = ast.ident;

    // Code-Typ 3 (no version argument) transmits the version chosen by the
    // sender, so it has to be a real field; Code-Typ 1/2 keep the constant.
    let list_version_field = if let Some(version) = version.as_deref() {
        quote! {
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
    } else {
        quote! {
            #[xml(name = b"listVersionID", ty = "attr")]
            #[serde(rename = "listVersionID", default)]
            #[cfg_attr(feature = "builder", builder(default))]
            pub list_version_id: String,
        }
    };

    let validation = if validate {
        None
    } else {
        Some(quote! {
            fn validate(&self, _: &impl crate::CodeListsProvider) -> bool {
                true
            }
        })
    };

    // code type 1/2: the version is fixed by the standard
    // code type 3 : the version is transmitted and therefore stored in the field
    let version_accessor = match version.as_deref() {
        Some(version) => quote! { fn version(&self) -> &str { #version } },
        None => quote! { fn version(&self) -> &str { &self.list_version_id } },
    };
    let default_derive = if version.is_some() {
        quote! { #[derive(Default)] }
    } else {
        quote! {}
    };

    // code type 3 needs a hand-written Default: the version starts at the
    // currently published one instead of an empty string
    let default_impl = if version.is_some() {
        quote! {}
    } else {
        let default_version = current.clone().unwrap_or_default();
        quote! {
            impl Default for #name {
                fn default() -> Self {
                    Self {
                        code: String::new(),
                        name: None,
                        list_version_id: #default_version.to_string(),
                        _list_uri: ConstStr,
                    }
                }
            }
        }
    };

    Ok(quote! {
        #[derive(Clone, Debug, raxb::XmlSerialize, raxb::XmlDeserialize, serde::Serialize, serde::Deserialize)]
        #default_derive
        #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
        #[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
        #[cfg_attr(feature = "builder", derive(TypedBuilder))]
        #[xml(tns(b"xwas", crate::TNS))]
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
            #list_version_field
        }

        #default_impl


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
            #version_accessor
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
    /// Version fixed by the standard (code type 1/2): part of the schema, not
    /// of the document.
    version: Option<String>,
    /// Currently published version of a sender-chosen codelist (code type 3):
    /// the default a freshly created value starts with.
    current: Option<String>,
    validate: bool,
}

impl syn::parse::Parse for XoevXWasserCodeArgs {
    /// `#[xoev_xwasser_code("<urn>", fixed = "3", validate)]`
    /// `#[xoev_xwasser_code("<urn>", current = "5")]`
    ///
    /// The codelist URI is the only positional argument; everything else is
    /// named so a declaration says what it means.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let uri = input.parse().map(|litstr: syn::LitStr| litstr.value())?;

        let mut fixed = None;
        let mut current = None;
        let mut validate = false;

        while !input.is_empty() {
            let _: syn::token::Comma = input.parse()?;
            if input.is_empty() {
                break; // trailing comma
            }
            let meta: syn::Meta = input.parse()?;
            match &meta {
                syn::Meta::Path(path) if path.is_ident("validate") => validate = true,
                syn::Meta::NameValue(named) => {
                    let value = match &named.value {
                        syn::Expr::Lit(expr) => match &expr.lit {
                            syn::Lit::Str(lit) => lit.value(),
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    &named.value,
                                    "expected a string, e.g. fixed = \"3\"",
                                ));
                            }
                        },
                        _ => {
                            return Err(syn::Error::new_spanned(
                                &named.value,
                                "expected a string, e.g. current = \"5\"",
                            ));
                        }
                    };
                    if named.path.is_ident("fixed") {
                        fixed = Some(value);
                    } else if named.path.is_ident("current") {
                        current = Some(value);
                    } else {
                        return Err(syn::Error::new_spanned(
                            &named.path,
                            "unknown argument, expected `fixed`, `current` or `validate`",
                        ));
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        meta,
                        "unexpected argument, expected `fixed = \"..\"`, `current = \"..\"` or `validate`",
                    ));
                }
            }
        }

        if fixed.is_some() && current.is_some() {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "`fixed` (version set by the standard) and `current` (version chosen by the sender) are mutually exclusive",
            ));
        }

        Ok(Self {
            uri,
            version: fixed,
            current,
            validate,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(attr: proc_macro2::TokenStream) -> XoevXWasserCodeArgs {
        xoev_xwasser_code_args(attr).expect("valid attribute")
    }

    #[test]
    fn parses_named_arguments() {
        for (case, expected) in [
            (
                quote! { "urn" },
                XoevXWasserCodeArgs {
                    uri: "urn".into(),
                    version: None,
                    current: None,
                    validate: false,
                },
            ),
            (
                quote! { "urn", },
                XoevXWasserCodeArgs {
                    uri: "urn".into(),
                    version: None,
                    current: None,
                    validate: false,
                },
            ),
            (
                quote! { "urn", fixed = "3" },
                XoevXWasserCodeArgs {
                    uri: "urn".into(),
                    version: Some("3".into()),
                    current: None,
                    validate: false,
                },
            ),
            (
                quote! { "urn", fixed = "3", validate },
                XoevXWasserCodeArgs {
                    uri: "urn".into(),
                    version: Some("3".into()),
                    current: None,
                    validate: true,
                },
            ),
            (
                quote! { "urn", current = "5" },
                XoevXWasserCodeArgs {
                    uri: "urn".into(),
                    version: None,
                    current: Some("5".into()),
                    validate: false,
                },
            ),
            (
                quote! { "urn", current = "5", validate },
                XoevXWasserCodeArgs {
                    uri: "urn".into(),
                    version: None,
                    current: Some("5".into()),
                    validate: true,
                },
            ),
            (
                quote! { "urn", validate },
                XoevXWasserCodeArgs {
                    uri: "urn".into(),
                    version: None,
                    current: None,
                    validate: true,
                },
            ),
        ] {
            assert_eq!(parse(case), expected);
        }
    }

    #[test]
    fn rejects_positional_and_unknown_arguments() {
        for case in [
            quote! { "urn", "3" },
            quote! { "urn", true },
            quote! { "urn", version = "3" },
        ] {
            assert!(xoev_xwasser_code_args(case).is_err());
        }
    }

    #[test]
    fn rejects_fixed_together_with_current() {
        let err = xoev_xwasser_code_args(quote! { "urn", fixed = "3", current = "5" })
            .expect_err("must not accept both");
        assert!(err.to_string().contains("mutually exclusive"));
    }
}
