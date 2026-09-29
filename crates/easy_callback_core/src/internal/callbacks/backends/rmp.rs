use crate::internal::{
    constants::FALLBACK,
    utils::{bytes_to_base_64, callback_query_to_bytes},
};
use rmp_serde::{from_slice, to_vec};
use serde::{Serialize, de::DeserializeOwned};
use teloxide::prelude::CallbackQuery;

/// A trait that ensures all traits required by rmp are implemented.
pub trait RmpCallback: Default + Serialize + DeserializeOwned {}

/// Encodes a value using rmp.
pub fn encode(callback: &impl RmpCallback) -> String {
    to_vec(callback)
        .map(bytes_to_base_64)
        .unwrap_or(FALLBACK.to_string())
}

/// Decodes a value using rmp.
pub fn decode<T: RmpCallback>(callback_query: &CallbackQuery) -> T {
    callback_query_to_bytes(callback_query)
        .and_then(|bytes| from_slice(bytes.as_slice()).ok())
        .unwrap_or_default()
}
