use std::{time::Duration, sync::Arc};

use dotenv::dotenv;
use teloxide::{prelude::*, utils::command::BotCommands};

use command::{Command, Param};
use ad_protection::AdProtection;

mod command;
mod configuration;
mod ad_protection;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting home bot...");

    let configuration = configuration::read_config();
    let bot = Bot::from_env();

    let ad_protection = AdProtection::new(
        configuration.ad_protection.base_url.clone(),
        configuration.ad_protection.token.clone(),
    );

    let ad_protection = Arc::new(ad_protection);
    let configuration = Arc::new(configuration);

    Command::repl(bot, move |bot: Bot, msg: Message, cmd: Command| {
        let adp = ad_protection.clone();
        let config = configuration.clone();

        async move {
            if msg.chat.id.0 != config.chat.id {
                log::error!("Chat Id {} doesn't match!", msg.chat.id);

                return Ok(())
            }

            match cmd {
                Command::Ping => {
                    log::info!("Received ping: {:?}", msg.chat.id);

                    bot.send_message(msg.chat.id, "pong").await?
                }
                Command::IsTrackingServiceEnabled => {
                    log::info!("Checking if tracking service is enabled...");

                    let status = if adp.is_enabled() { "enabled" } else { "disabled" };

                    bot.send_message(msg.chat.id, status).await?
                }
                Command::DisableTrackingService { duration } => {
                    log::info!("Received DisableAdBlock command with duration: {:?}", duration);

                    if let Param::SingleNumber(seconds) = duration {
                        adp.disable(Duration::from_secs(seconds.into()));
                    } else {
                        adp.disable(Duration::from_secs(10));
                    }

                    bot.send_message(msg.chat.id, "ok").await?
                }
                _ => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?
            };

            Ok(())
        }
    }).await;
}
