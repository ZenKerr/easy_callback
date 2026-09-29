use crate::internal::{
    constants::FALLBACK,
    utils::{bytes_to_base_64, callback_query_to_bytes},
};
use teloxide::prelude::CallbackQuery;
use wincode::{SchemaRead, SchemaWrite, config::DefaultConfig, deserialize, serialize};

/// A trait that ensures all traits required by wincode are implemented.
pub trait WincodeCallback:
    Default
    + SchemaWrite<DefaultConfig, Src = Self>
    + for<'de> SchemaRead<'de, DefaultConfig, Dst = Self>
{
}

/// Encodes a value using wincode.
pub fn encode(callback: &impl WincodeCallback) -> String {
    serialize(callback)
        .map(bytes_to_base_64)
        .unwrap_or(FALLBACK.to_string())
}

/// Decodes a value using wincode.
pub fn decode<T: WincodeCallback>(callback_query: &CallbackQuery) -> T {
    callback_query_to_bytes(callback_query)
        .and_then(|bytes| deserialize(bytes.as_slice()).ok())
        .unwrap_or_default()
}
