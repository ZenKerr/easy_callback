use crate::internal::{
    constants::FALLBACK,
    utils::{bytes_to_base_64, callback_query_to_bytes},
};
use oxicode::{Decode, Encode, decode_from_slice, encode_to_vec};
use teloxide::prelude::CallbackQuery;

/// A trait that ensures all traits required by oxicode are implemented.
pub trait OxicodeCallback: Default + Encode + Decode {}

/// Encodes a value using oxicode.
pub fn encode(callback: &impl OxicodeCallback) -> String {
    encode_to_vec(callback)
        .map(bytes_to_base_64)
        .unwrap_or(FALLBACK.to_string())
}

/// Decodes a value using oxicode.
pub fn decode<T: OxicodeCallback>(callback_query: &CallbackQuery) -> T {
    callback_query_to_bytes(callback_query)
        .and_then(|bytes| {
            let (callback, _) = decode_from_slice(bytes.as_slice()).ok()?;

            Some(callback)
        })
        .unwrap_or_default()
}
