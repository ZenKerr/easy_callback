use easy_callback::derive::{
    BitcodeCallback, BorshCallback, CiboriumCallback, PostcardCallback, RkyvCallback, RmpCallback,
    WincodeCallback,
};
use easy_callback_core::AbstractCallback;

// Example demonstrating a comparison of different encoding
// implementations by the size of their encoded output.
//
// Each implementation produces a report in the following format:
// Implementation name
// Encoded message | Message size | Data that was encoded
// Encoded message | Message size | Data that was encoded
//
// Two variants are encoded for each implementation:
// * `Action` containing `i64` and `f32` values
// * `Fallback` containing no additional data
//
// The goal is to compare how efficiently each implementation represents the same data.

macro_rules! implementation {
    ($name:ident, [$($derive:path),* $(,)?] $(,)?) => {
        {
            #[derive(Default, Debug, $($derive),*)]
            #[repr(u8)]
            enum Callback {
                Action {
                    id: i64,
                    value: f32,
                },
                #[default]
                Fallback,
            }

            let callbacks = [
                Callback::Action {
                    id: 1234,
                    value: 1.234,
                },
                Callback::Fallback,
            ];
            let encoded = callbacks.each_ref().map(Callback::encode);
            let encoded_sizes = encoded.each_ref().map(String::len);

            let first_column_size = encoded_sizes.iter().max().unwrap_or(&0);

            println!("{}", stringify!($name));
            for ((encoded, encoded_size), callback) in encoded.iter().zip(encoded_sizes).zip(callbacks) {
                println!(
                    "{encoded:<first_column_size$} | {encoded_size:>2} bytes | {callback:?}",
                    first_column_size = first_column_size,
                );
            }
            println!();
        }
    };
}

fn main() {
    implementation!(
        rkyv,
        [
            rkyv::Serialize,
            rkyv::Deserialize,
            rkyv::Archive,
            RkyvCallback,
        ],
    );
    implementation!(
        wincode,
        [wincode::SchemaWrite, wincode::SchemaRead, WincodeCallback],
    );
    implementation!(
        postcard,
        [serde::Serialize, serde::Deserialize, PostcardCallback],
    );
    implementation!(bitcode, [bitcode::Encode, bitcode::Decode, BitcodeCallback]);
    implementation!(rmp, [serde::Serialize, serde::Deserialize, RmpCallback]);
    implementation!(
        ciborium,
        [serde::Serialize, serde::Deserialize, CiboriumCallback],
    );
    implementation!(
        borsh,
        [
            borsh::BorshSerialize,
            borsh::BorshDeserialize,
            BorshCallback
        ],
    );
}
