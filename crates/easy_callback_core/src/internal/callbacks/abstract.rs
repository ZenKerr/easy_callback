use teloxide::prelude::CallbackQuery;

/// An abstraction for callbacks, used to avoid tightly coupling the program to a specific implementation.
pub trait AbstractCallback {
    fn encode(&self) -> String;
    fn decode(callback_query: &CallbackQuery) -> Self;
}
