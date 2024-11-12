use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput, Error, Result};

pub(super) fn derive(input: TokenStream) -> Result<TokenStream> {
    let ast: DeriveInput = syn::parse2(input)?;

    let content = match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => {
                let fields = fields_named
                    .named
                    .iter()
                    .filter_map(|field| field.ident.as_ref());
                quote! { Ok(())#(.and(self.#fields.xwasser_validate(codelists)))* }
            }
            syn::Fields::Unnamed(fields_unnamed) => {
                return Err(Error::new(
                    fields_unnamed.span(),
                    "unnamed fields not supported",
                ));
            }
            syn::Fields::Unit => {
                return Err(Error::new(data_struct.fields.span(), "unit not supported"));
            }
        },
        Data::Enum(data_enum) => {
            let variants = data_enum
                .variants
                .iter()
                .map(|v| match &v.fields {
                    syn::Fields::Unnamed(fields_unnamed) => {
                        if fields_unnamed.unnamed.len() == 1 {
                            let ident = &v.ident;
                            Ok(quote! { Self::#ident(e) => e.xwasser_validate(codelists) })
                        } else {
                            Err(Error::new(
                                fields_unnamed.span(),
                                "unnamed enum fields with multiple members not supported",
                            ))
                        }
                    }
                    syn::Fields::Unit => {
                        let ident = &v.ident;
                        Ok(quote! { Self::#ident => Ok(()) })
                    }
                    syn::Fields::Named(fields_named) => Err(Error::new(
                        fields_named.span(),
                        "named enum fields not supported",
                    )),
                })
                .collect::<Result<Vec<_>>>()?;
            quote! {
                match self {
                    #(#variants,)*
                }
            }
        }
        Data::Union(data_union) => {
            return Err(Error::new(data_union.fields.span(), "union not supported"));
        }
    };

    Ok(trait_xwasser_validate(&ast, content))
}

fn trait_xwasser_validate(ast: &DeriveInput, content: TokenStream) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let type_params = generics.type_params();
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl <#(, #type_params)*> crate::XWasserValidate for #name #ty_generics #where_clause {
            fn xwasser_validate(&self, codelists: &crate::CodeLists) -> Result<(), crate::XWasserValidateError> {
                use crate::XWasserValidate as _;
                #content
            }
        }
    }
}
