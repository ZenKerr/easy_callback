# easy_callback

[![Crates.io](https://img.shields.io/crates/v/easy_callback.svg?logo=rust)](https://crates.io/crates/easy_callback)
[![Documentation](https://img.shields.io/docsrs/easy_callback?logo=rust)](https://docs.rs/easy_callback)
[![Tests](https://img.shields.io/github/actions/workflow/status/ZenKerr/easy_callback/rust.yml?label=tests&logo=github)](https://github.com/ZenKerr/easy_callback/actions)
![License](https://img.shields.io/crates/l/easy_callback.svg?logo=readme&logoColor=white)

Automatic callback data encoding for teloxide.

This crate provides a derive macro for quickly converting a custom
user-defined type to a callback data string and converting a `CallbackQuery`
back into the corresponding user-defined type, with support for different
serialization implementations.

When encoding, the data is first serialized using the selected
implementation and then encoded as a URL-safe, unpadded Base64 string.
During decoding, the process is reversed.

---

## Example

This example demonstrates the library's functionality using the
[count_bot](https://github.com/ZenKerr/easy_callback/blob/HEAD/examples/count_bot/src/main.rs) example.

```rust
// ...

// Callback enum encoded using postcard
#[derive(Default, Serialize, Deserialize, PostcardCallback)]
enum Callback {
  Increment(i64),
  Decrement(i64),
  #[default]
  Fallback,
}

// ...

fn keyboard(value: i64) -> InlineKeyboardMarkup {
    // The callback is automatically encoded using postcard
    InlineKeyboardMarkup::new([[
        InlineKeyboardButton::callback("+1", Callback::Increment(value)),
        InlineKeyboardButton::callback("-1", Callback::Decrement(value)),
    ]])
}

// ...

async fn on_callback(bot: Bot, callback_query: CallbackQuery) -> ResponseResult<()> {
  // ...

  // Decode the callback query into the Callback enum
  let callback = Callback::from(callback_query);
  
  // ...
}

// ...
```

---

## Usage

Add the following to your `Cargo.toml`,
choosing one or more features corresponding to the implementations you want to use:

```toml
[dependencies]
easy_callback = { version = "1.1.0", features=["postcard"] }
```

Available implementations:
* [rkyv](https://crates.io/crates/rkyv)
* [wincode](https://crates.io/crates/wincode)
* [postcard](https://crates.io/crates/postcard)
* [bitcode](https://crates.io/crates/bitcode)
* [rmp](https://crates.io/crates/rmp-serde)
* [ciborium](https://crates.io/crates/ciborium)
* [borsh](https://crates.io/crates/borsh)

Multiple implementations can be enabled simultaneously if required.

---

## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/ZenKerr/easy_callback/blob/HEAD/LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/ZenKerr/easy_callback/blob/HEAD/LICENSE-MIT))

at your option.
