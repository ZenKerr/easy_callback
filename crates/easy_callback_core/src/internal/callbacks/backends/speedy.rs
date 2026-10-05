use crate::internal::{
    constants::FALLBACK,
    utils::{bytes_to_base_64, callback_query_to_bytes},
};
use speedy::{LittleEndian, Readable, Writable};
use teloxide::prelude::CallbackQuery;

/// A trait that ensures all traits required by speedy are implemented.
pub trait SpeedyCallback:
    Default + Writable<LittleEndian> + for<'a> Readable<'a, LittleEndian>
{
}

/// Encodes a value using speedy.
pub fn encode(callback: &impl SpeedyCallback) -> String {
    callback
        .write_to_vec()
        .map(bytes_to_base_64)
        .unwrap_or(FALLBACK.to_string())
}

/// Decodes a value using speedy.
pub fn decode<T: SpeedyCallback>(callback_query: &CallbackQuery) -> T {
    callback_query_to_bytes(callback_query)
        .and_then(|bytes| T::read_from_buffer(bytes.as_slice()).ok())
        .unwrap_or_default()
}
