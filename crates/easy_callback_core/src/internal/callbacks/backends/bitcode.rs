use crate::internal::utils::{bytes_to_base_64, callback_query_to_bytes};
use bitcode::{DecodeOwned, Encode, decode as bitcode_decode, encode as bitcode_encode};
use teloxide::prelude::CallbackQuery;

/// A trait that ensures all traits required by bitcode are implemented.
pub trait BitcodeCallback: Default + Encode + DecodeOwned {}

/// Encodes a value using bitcode.
pub fn encode(callback: &impl BitcodeCallback) -> String {
    bytes_to_base_64(bitcode_encode(callback))
}

/// Decodes a value using bitcode.
pub fn decode<T: BitcodeCallback>(callback_query: &CallbackQuery) -> T {
    callback_query_to_bytes(callback_query)
        .and_then(|bytes| bitcode_decode(bytes.as_slice()).ok())
        .unwrap_or_default()
}
