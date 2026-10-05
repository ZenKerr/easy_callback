use easy_callback_derive::{
    BitcodeCallback, BorshCallback, CiboriumCallback, PostcardCallback, RkyvCallback, RmpCallback,
    WincodeCallback,
};
use teloxide::types::{CallbackQuery, CallbackQueryId, User, UserId};

macro_rules! check_implementation {
    ($name:ident, [$($derive:path),* $(,)?] $(,)?) => {
        #[test]
        fn $name() {
            #[derive(PartialEq, Debug, Default, $($derive),*)]
            #[repr(u8)]
            enum Callback {
                Action {
                    id: i64,
                    value: f32,
                },
                #[default]
                Fallback,
            }

            let callback = Callback::Action {
                id: 1234,
                value: 1.234,
            };
            let encoded_callback = String::from(&callback);
            let callback_query = CallbackQuery {
                id: CallbackQueryId(String::new()),
                from: User {
                    id: UserId(0),
                    is_bot: false,
                    first_name: String::new(),
                    last_name: None,
                    username: None,
                    language_code: None,
                    is_premium: false,
                    added_to_attachment_menu: false,
                },
                message: None,
                inline_message_id: None,
                chat_instance: String::new(),
                data: Some(encoded_callback),
                game_short_name: None,
            };
            let decoded_callback = Callback::from(callback_query);

            assert_eq!(decoded_callback, callback);
        }
    };
}

check_implementation!(
    rkyv,
    [
        rkyv::Serialize,
        rkyv::Deserialize,
        rkyv::Archive,
        RkyvCallback,
    ],
);
check_implementation!(
    wincode,
    [wincode::SchemaWrite, wincode::SchemaRead, WincodeCallback],
);
check_implementation!(
    postcard,
    [serde::Serialize, serde::Deserialize, PostcardCallback],
);
check_implementation!(bitcode, [bitcode::Encode, bitcode::Decode, BitcodeCallback]);
check_implementation!(rmp, [serde::Serialize, serde::Deserialize, RmpCallback]);
check_implementation!(
    ciborium,
    [serde::Serialize, serde::Deserialize, CiboriumCallback],
);
check_implementation!(
    borsh,
    [
        borsh::BorshSerialize,
        borsh::BorshDeserialize,
        BorshCallback,
    ],
);
