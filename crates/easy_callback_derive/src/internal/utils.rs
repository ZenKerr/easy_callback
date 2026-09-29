use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Ident, Span};

use crate::internal::aliases::{SynError, SynResult};
use syn::{Path, PathArguments, PathSegment};

pub fn get_crate_name(name: &str) -> SynResult<String> {
    let crate_name = crate_name(name)
        .map_err(|_| SynError::new(Span::call_site(), format!("Crate `{name}` not found")))?;

    Ok(match crate_name {
        FoundCrate::Itself => "crate".to_string(),
        FoundCrate::Name(name) => name,
    })
}

pub fn build_path(parts: &[&str]) -> Path {
    Path {
        leading_colon: None,
        segments: parts
            .iter()
            .map(|part| PathSegment {
                ident: Ident::new(part, Span::call_site()),
                arguments: PathArguments::None,
            })
            .collect(),
    }
}
