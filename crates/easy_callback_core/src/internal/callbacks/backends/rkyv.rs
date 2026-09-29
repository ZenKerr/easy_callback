use crate::internal::{
    constants::FALLBACK,
    utils::{bytes_to_base_64, callback_query_to_bytes},
};
use rkyv::{
    Archive, Deserialize, Serialize, access,
    api::high::{HighDeserializer, HighSerializer, HighValidator},
    bytecheck::CheckBytes,
    deserialize,
    rancor::Error,
    ser::allocator::ArenaHandle,
    to_bytes,
    util::AlignedVec,
};
use teloxide::prelude::CallbackQuery;

type Serializer<'a> = HighSerializer<AlignedVec, ArenaHandle<'a>, Error>;
type Validator<'a> = HighValidator<'a, Error>;
type Deserializer = HighDeserializer<Error>;

/// A trait that ensures all traits required by rkyv are implemented.
pub trait RkyvCallback:
    Default
    + for<'a> Serialize<Serializer<'a>>
    + Archive<Archived: for<'a> CheckBytes<Validator<'a>> + Deserialize<Self, Deserializer>>
{
}

/// Encodes a value using rkyv.
pub fn encode(callback: &impl RkyvCallback) -> String {
    to_bytes(callback)
        .map(|bytes| bytes_to_base_64(bytes.into_vec()))
        .unwrap_or(FALLBACK.to_string())
}

/// Decodes a value using rkyv.
pub fn decode<T: RkyvCallback>(callback_query: &CallbackQuery) -> T {
    callback_query_to_bytes(callback_query)
        .and_then(|bytes| {
            let archived = access::<T::Archived, Error>(bytes.as_slice()).ok()?;

            deserialize(archived).ok()
        })
        .unwrap_or_default()
}
