use std::{sync::Arc, time::Duration};

use dotenv::dotenv;
use teloxide::{prelude::*, types::User, utils::command::BotCommands};

use command::{Command, Param};
use pihole::Pihole;

mod command;
mod configuration;
mod pihole;

const DEFAULT_DURATION_TO_DISABLE_PIHOLE: u64 = 600; // 10 minutes

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init_timed();
    log::info!("Starting home bot...");

    let configuration = configuration::read_config();
    let bot = Bot::new(&configuration.bot.token);

    let pihole = Pihole::new(
        configuration.pihole.base_url.clone(),
        configuration.pihole.token.clone(),
    );

    let pihole = Arc::new(pihole);
    let configuration = Arc::new(configuration);

    Command::repl(bot, move |bot: Bot, msg: Message, cmd: Command| {
        let ph = pihole.clone();
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
                Command::IsPiholeEnabled => {
                    log::info!("Checking if pihole is enabled...");

                    match ph.is_enabled().await {
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
                Command::DisablePihole { duration } => {
                    log::info!("Received DisablePihole command with duration: {:?}", duration);

                    let dr: u64;
                    let result: Result<bool, reqwest::Error>;

                    if let Param::SingleNumber(minutes) = duration {
                        let seconds = minutes * 60;

                        dr = minutes.into();
                        result = ph.disable(Duration::from_secs(seconds.into())).await;
                    } else {
                        dr = DEFAULT_DURATION_TO_DISABLE_PIHOLE / 60;
                        result = ph.disable(Duration::from_secs(DEFAULT_DURATION_TO_DISABLE_PIHOLE)).await;
                    }

                    match result {
                        Ok(_) => {
                            let reply = format!("ok. disabled pihole for {dr} minutes.");

                            bot.send_message(msg.chat.id, reply).await?
                        },
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
        teloxide::types::MessageKind::Common(mc) => mc.from.clone(),
        _ => None,
    }
}
