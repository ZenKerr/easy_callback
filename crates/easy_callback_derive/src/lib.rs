#[cfg(feature = "_any_feature")]
mod internal;

#[cfg(feature = "_any_feature")]
use internal::derive_callback;

#[cfg(feature = "rkyv")]
derive_callback! {
    rkyv => [
        rkyv::Serialize,
        rkyv::Deserialize,
        rkyv::Archive,
    ],
}

#[cfg(feature = "wincode")]
derive_callback! {
    wincode => [
        wincode::SchemaWrite,
        wincode::SchemaRead,
    ],
}

#[cfg(feature = "postcard")]
derive_callback! {
    postcard => [
        serde::Serialize,
        serde::Deserialize,
    ],
}

#[cfg(feature = "bitcode")]
derive_callback! {
    bitcode => [
        bitcode::Encode,
        bitcode::Decode,
    ],
}

#[cfg(feature = "rmp")]
derive_callback! {
    rmp => [
        serde::Serialize,
        serde::Deserialize,
    ],
}

#[cfg(feature = "ciborium")]
derive_callback! {
    ciborium => [
        serde::Serialize,
        serde::Deserialize,
    ],
}

#[cfg(feature = "borsh")]
derive_callback! {
    borsh => [
        borsh::BorshSerialize,
        borsh::BorshDeserialize,
    ],
}

#[cfg(feature = "speedy")]
derive_callback! {
    speedy => [
        speedy::Writable,
        speedy::Readable,
    ],
}
