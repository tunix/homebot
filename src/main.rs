use std::{time::Duration, sync::Arc};

use dotenv::dotenv;
use teloxide::{prelude::*, utils::command::BotCommands, types::User};

use ad_protection::AdProtection;
use command::{Command, Param};

mod ad_protection;
mod command;
mod configuration;

const DEFAULT_DURATION_TO_DISABLE_AD_PROTECTION: u64 = 600; // 10 minutes

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init_timed();
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

        log::debug!("Received msg: {:?}", msg);

        async move {
            if msg.chat.id.0 != config.chat.id {
                log::error!("Chat Id {} doesn't match!", msg.chat.id);

                return Ok(())
            }

            extract_user(&msg)
                .map(|user|
                    log::info!(
                        "Received message from user id: {}, username: {}, first_name: {}, last_name: {}",
                        user.id,
                        user.username.unwrap_or("".to_string()),
                        user.first_name,
                        user.last_name.unwrap_or("".to_string())
                    )
                );

            match cmd {
                Command::Ping => {
                    log::info!("Received ping: {:?}", msg.chat.id);

                    bot.send_message(msg.chat.id, "pong").await?
                }
                Command::IsAdProtectionEnabled => {
                    log::info!("Checking if tracking service is enabled...");

                    match adp.is_enabled().await {
                        Ok(is_enabled) => {
                            let status = if is_enabled { "enabled" } else { "disabled" };

                            bot.send_message(msg.chat.id, status).await?
                        }
                        Err(e) => {
                            log::error!("{}", e);

                            bot.send_message(msg.chat.id, "Error occurred. Please try again later.").await?
                        }
                    }
                }
                Command::DisableAdProtection { duration } => {
                    log::info!("Received DisableAdBlock command with duration: {:?}", duration);

                    let result: Result<bool, reqwest::Error>;

                    if let Param::SingleNumber(minutes) = duration {
                        let seconds = minutes * 60;

                        result = adp.disable(Duration::from_secs(seconds.into())).await;
                    } else {
                        result = adp.disable(Duration::from_secs(DEFAULT_DURATION_TO_DISABLE_AD_PROTECTION)).await;
                    }

                    match result {
                        Ok(_) => bot.send_message(msg.chat.id, "ok").await?,
                        Err(e) => {
                            log::error!("{}", e);

                            bot.send_message(msg.chat.id, "Error occurred. Please try again later.").await?
                        }
                    }
                }
                _ => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?
            };

            Ok(())
        }
    }).await;
}

fn extract_user(msg: &Message) -> Option<User> {
    match &msg.kind {
        teloxide::types::MessageKind::Common(mc) => {
            mc.from.clone()
        }
        _ => None
    }
}
