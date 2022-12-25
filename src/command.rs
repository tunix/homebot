use std::str::FromStr;

use teloxide::utils::command::BotCommands;

#[derive(Clone, Debug)]
pub enum Param {
    NoParam,
    SingleNumber(u8),
}

impl FromStr for Param {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(number) = s.parse::<u8>().ok() {
            return Ok(Param::SingleNumber(number));
        }

        Ok(Param::NoParam)
    }
}

#[derive(BotCommands, Clone)]
#[command(description = "These commands are supported:")]
pub enum Command {
    #[command(rename_rule = "lowercase", description = "off")]
    Start,
    #[command(rename_rule = "lowercase", description = "Display this text.")]
    Help,
    #[command(rename_rule = "lowercase", description = "Ping & Pong")]
    Ping,
    #[command(rename_rule = "snake_case", description = "Checks whether the tracking service blocking is enabled or not.")]
    IsTrackingServiceEnabled,
    #[command(rename_rule = "snake_case", description = "Disables tracking services for the given duration.")]
    DisableTrackingService { duration: Param },
}
