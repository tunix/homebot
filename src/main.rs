use std::{time::Duration, sync::Arc};

use dotenv::dotenv;
use teloxide::{prelude::*, utils::command::BotCommands};

use command::Command;

use crate::ad_protection::AdProtection;

mod command;
mod ad_protection;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting home bot...");

    let bot = Bot::from_env();

    let ad_protection = AdProtection::new(
        String::from("http://192.168.1.2"),
        String::from("abc123"),
    );

    let ad_protection = Arc::new(ad_protection);

    Command::repl(bot, move |bot: Bot, msg: Message, cmd: Command| {
        let adp = ad_protection.clone();

        async move {
            match cmd {
                Command::IsTrackingServiceEnabled => {
                    log::info!("Checking if tracking service is enabled...");

                    let enabled = if adp.is_enabled() { "enabled" } else { "disabled" };

                    bot.send_message(msg.chat.id, enabled).await?
                }
                Command::DisableTrackingServices { duration } => {
                    log::info!("Received DisableAdBlock command with duration: {:?}", duration);

                    adp.disable(Duration::from_secs(10));

                    bot.send_message(msg.chat.id, "ok").await?
                }
                _ => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?
            };

            Ok(())
        }
    }).await;
}
