use paste::paste;

macro_rules! enum_format {
    ($($format:ident),* $(,)?) => {
        paste! {
            #[derive(Copy, Clone)]
            #[repr(u8)]
            pub enum Format {
                $(
                    #[cfg(feature = "" [<$format:lower>])]
                    $format,
                )*
            }

            impl Format {
                pub fn package_name(self) -> &'static str {
                    match self {
                        $(
                            #[cfg(feature = "" [<$format:lower>])]
                            Self::$format => stringify!([<$format:lower>]),
                        )*
                    }
                }

                pub fn trait_name(self) -> &'static str {
                    match self {
                        $(
                            #[cfg(feature = "" [<$format:lower>])]
                            Self::$format => stringify!([<$format Callback>]),
                        )*
                    }
                }
            }
        }
    };
}

enum_format! {
    Rkyv,
    Wincode,
    Postcard,
    Bitcode,
    Rmp,
    Ciborium,
    Borsh,
    Speedy,
}
