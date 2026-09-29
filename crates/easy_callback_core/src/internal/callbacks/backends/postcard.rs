use crate::internal::{
    constants::FALLBACK,
    utils::{bytes_to_base_64, callback_query_to_bytes},
};
use postcard::{from_bytes, to_allocvec};
use serde::{Serialize, de::DeserializeOwned};
use teloxide::prelude::CallbackQuery;

/// A trait that ensures all traits required by postcard are implemented.
pub trait PostcardCallback: Default + Serialize + DeserializeOwned {}

/// Encodes a value using postcard.
pub fn encode(callback: &impl PostcardCallback) -> String {
    to_allocvec(callback)
        .map(bytes_to_base_64)
        .unwrap_or(FALLBACK.to_string())
}

/// Decodes a value using postcard.
pub fn decode<T: PostcardCallback>(callback_query: &CallbackQuery) -> T {
    callback_query_to_bytes(callback_query)
        .and_then(|bytes| from_bytes(bytes.as_slice()).ok())
        .unwrap_or_default()
}
