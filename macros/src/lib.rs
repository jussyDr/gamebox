use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericParam, TypeParamBound};

#[proc_macro_derive(FromLe)]
pub fn derive_from_le(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut generics = input.generics;

    for generic_param in &mut generics.params {
        if let GenericParam::Type(type_param) = generic_param {
            type_param.bounds.push(TypeParamBound::Verbatim(quote!(
                crate::read::reader::FromLe
            )));
        }
    }

    if let Data::Struct(ref data) = input.data {
        if let Fields::Named(ref fields) = data.fields {
            let field_setters = fields.named.iter().map(|field| {
                let name = &field.ident;

                quote!(value.#name = crate::read::reader::FromLe::from_le(value.#name);)
            });

            let ident = input.ident;
            let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

            let tokens = quote!(
                impl #impl_generics crate::read::reader::FromLe for #ident #type_generics #where_clause {
                    fn from_le(mut value: Self) -> Self {
                        #(#field_setters)
                        *

                        value
                    }
                }
            );

            return tokens.into();
        }
    }

    TokenStream::from(
        syn::Error::new(
            input.ident.span(),
            "Only structs with named fields can derive `FromRow`",
        )
        .to_compile_error(),
    )
}
