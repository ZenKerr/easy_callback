use crate::internal::{
    constants::FALLBACK,
    utils::{bytes_to_base_64, callback_query_to_bytes},
};
use ciborium::{from_reader, into_writer};
use serde::{Serialize, de::DeserializeOwned};
use teloxide::prelude::CallbackQuery;

/// A trait that ensures all traits required by ciborium are implemented.
pub trait CiboriumCallback: Default + Serialize + DeserializeOwned {}

/// Encodes a value using ciborium.
pub fn encode(callback: &impl CiboriumCallback) -> String {
    let mut bytes = Vec::new();

    into_writer(callback, &mut bytes)
        .map(|_| bytes_to_base_64(bytes))
        .unwrap_or(FALLBACK.to_string())
}

/// Decodes a value using ciborium.
pub fn decode<T: CiboriumCallback>(callback_query: &CallbackQuery) -> T {
    callback_query_to_bytes(callback_query)
        .and_then(|bytes| from_reader(bytes.as_slice()).ok())
        .unwrap_or_default()
}
