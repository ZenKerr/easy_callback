use crate::internal::constants::FALLBACK;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as base64};
use teloxide::prelude::CallbackQuery;

pub fn bytes_to_base_64(bytes: Vec<u8>) -> String {
    base64.encode(bytes)
}

pub fn callback_query_to_bytes(callback_query: &CallbackQuery) -> Option<Vec<u8>> {
    let data = callback_query.data.as_deref()?;

    if data == FALLBACK {
        None
    } else {
        base64.decode(data).ok()
    }
}
