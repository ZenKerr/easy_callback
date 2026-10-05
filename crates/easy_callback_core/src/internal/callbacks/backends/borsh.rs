use crate::internal::{
    constants::FALLBACK,
    utils::{bytes_to_base_64, callback_query_to_bytes},
};
use borsh::{BorshDeserialize, BorshSerialize, from_slice, to_vec};
use teloxide::prelude::CallbackQuery;

/// A trait that ensures all traits required by borsh are implemented.
pub trait BorshCallback: Default + BorshSerialize + BorshDeserialize {}

/// Encodes a value using borsh.
pub fn encode(callback: &impl BorshCallback) -> String {
    to_vec(callback)
        .map(bytes_to_base_64)
        .unwrap_or(FALLBACK.to_string())
}

/// Decodes a value using borsh.
pub fn decode<T: BorshCallback>(callback_query: &CallbackQuery) -> T {
    callback_query_to_bytes(callback_query)
        .and_then(|bytes| from_slice(&bytes).ok())
        .unwrap_or_default()
}
