macro_rules! derive_callback {
    ($format:ident => [$($derive:path),* $(,)?] $(,)?) => {
        paste::paste! {
            #[doc = concat!("Derive `", stringify!([<$format:camel>]), "Callback` callback.")]
            ///
            /// The type must implement:
            /// * `Default`
            $(#[doc = concat!("* `", stringify!($derive), "`")])*
            ///
            /// The generated implementation provides:
            /// * `AbstractCallback`
            /// * `From<Self> for String`
            /// * `From<&Self> for String`
            /// * `From<CallbackQuery>`
            /// * `From<&CallbackQuery>`
            ///
            /// ```rust
            #[doc = concat!("#[derive(Default, ", stringify!($($derive),*), ", ", stringify!([<$format:camel>]), "Callback)]")]
            /// enum Callback {
            ///     Action { id: i64 },
            ///     #[default]
            ///     Fallback,
            /// }
            /// ```
            #[proc_macro_derive([<$format:camel Callback>])]
            pub fn [<derive_ $format _callback>](input: proc_macro::TokenStream) -> proc_macro::TokenStream {
                internal::backend(input, internal::Format::[<$format:camel>])
                    .unwrap_or_else(|error| error.into_compile_error().into())
            }
        }
    };
}

pub(crate) use derive_callback;
