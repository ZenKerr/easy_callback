use crate::internal::{
    aliases::SynResult,
    format::Format,
    utils::{build_path, get_crate_name},
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse};

pub fn backend(input: TokenStream, format: Format) -> SynResult<TokenStream> {
    let input = parse::<DeriveInput>(input)?;

    let callback = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let easy_callback_crate = get_crate_name("easy_callback")?;
    let teloxide_crate = get_crate_name("teloxide")?;

    let implementation_package =
        build_path([easy_callback_crate.as_str(), "core", format.package_name()].as_slice());
    let implementation_trait = build_path(
        [
            easy_callback_crate.as_str(),
            "core",
            format.package_name(),
            format.trait_name(),
        ]
        .as_slice(),
    );
    let abstract_callback_trait =
        build_path([easy_callback_crate.as_str(), "core", "AbstractCallback"].as_slice());
    let callback_query_type =
        build_path([teloxide_crate.as_str(), "prelude", "CallbackQuery"].as_slice());
    let string_type = build_path(["std", "string", "String"].as_slice());

    Ok(quote! {
        impl #impl_generics #implementation_trait for #callback #type_generics #where_clause {}

        impl #impl_generics #abstract_callback_trait for #callback #type_generics #where_clause {
            fn encode(&self) -> #string_type {
                #implementation_package::encode(self)
            }

            fn decode(callback_query: &#callback_query_type) -> Self {
                #implementation_package::decode(callback_query)
            }
        }

        impl #impl_generics From<#callback #type_generics> for #string_type #where_clause {
            fn from(callback: #callback #type_generics) -> Self {
                #abstract_callback_trait::encode(&callback)
            }
        }

        impl #impl_generics From<&#callback #type_generics> for #string_type #where_clause {
            fn from(callback: &#callback #type_generics) -> Self {
                #abstract_callback_trait::encode(callback)
            }
        }

        impl #impl_generics From<#callback_query_type> for #callback #type_generics #where_clause {
            fn from(callback_query: #callback_query_type) -> Self {
                #abstract_callback_trait::decode(&callback_query)
            }
        }

        impl #impl_generics From<&#callback_query_type> for #callback #type_generics #where_clause {
            fn from(callback_query: &#callback_query_type) -> Self {
                #abstract_callback_trait::decode(callback_query)
            }
        }
    }
    .into())
}
