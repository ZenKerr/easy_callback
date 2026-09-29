use easy_callback::derive::PostcardCallback;
use serde::{Deserialize, Serialize};
use std::io::{Write, stdin, stdout};
use teloxide::{
    Bot,
    dispatching::{HandlerExt, UpdateFilterExt},
    dptree::entry,
    macros::BotCommands,
    payloads::{EditMessageTextSetters, SendMessageSetters},
    prelude::{CallbackQuery, Dispatcher, Message, Requester, ResponseResult, Update},
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

// Standard enum used for teloxide bot commands
#[derive(BotCommands)]
#[command(rename_rule = "lowercase")]
enum Command {
    Start,
}

// Callback enum encoded using postcard
#[derive(Default, Serialize, Deserialize, PostcardCallback)]
enum Callback {
    Increment(i64),
    Decrement(i64),
    #[default]
    Fallback,
}

// Creates an inline keyboard with increment and decrement buttons
fn keyboard(value: i64) -> InlineKeyboardMarkup {
    // The callback is automatically encoded using postcard
    InlineKeyboardMarkup::new([[
        InlineKeyboardButton::callback("+1", Callback::Increment(value)),
        InlineKeyboardButton::callback("-1", Callback::Decrement(value)),
    ]])
}

// Handles the /start command and initializes the counter
async fn start(bot: Bot, message: Message) -> ResponseResult<()> {
    bot.send_message(message.chat.id, "0")
        .reply_markup(keyboard(0))
        .await?;

    Ok(())
}

// Handles callback queries from inline keyboard buttons
async fn on_callback(bot: Bot, callback_query: CallbackQuery) -> ResponseResult<()> {
    bot.answer_callback_query(callback_query.id.clone()).await?;

    if let Some(message) = callback_query.regular_message() {
        // Decode the callback query into the Callback enum
        let callback = Callback::from(&callback_query);

        // Apply the corresponding action
        let new_value = match callback {
            Callback::Increment(value) => value + 1,
            Callback::Decrement(value) => value - 1,
            Callback::Fallback => 0,
        };

        // Update the counter message and keyboard
        bot.edit_message_text(message.chat.id, message.id, new_value.to_string())
            .reply_markup(keyboard(new_value))
            .await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // Ask for the bot token used to run the example
    let mut bot_token = String::new();
    print!("Enter your bot token: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut bot_token).unwrap();
    let token = bot_token.trim();

    // Create the bot instance
    let bot = Bot::new(token);

    // Build the update handler for commands and callback queries
    let handler = entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(start),
        )
        .branch(Update::filter_callback_query().endpoint(on_callback));

    // Start the bot
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
