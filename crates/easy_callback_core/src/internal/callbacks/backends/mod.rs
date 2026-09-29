#[cfg(feature = "rkyv")]
pub mod rkyv;

#[cfg(feature = "wincode")]
pub mod wincode;

#[cfg(feature = "postcard")]
pub mod postcard;

#[cfg(feature = "bitcode")]
pub mod bitcode;

#[cfg(feature = "rmp")]
pub mod rmp;

#[cfg(feature = "ciborium")]
pub mod ciborium;
