# homebot

A Telegram bot that'll be used for some home automation and to learn Rust

## Platforms

Although it should work on all Rust supported platforms, the pipeline builds against the following platforms only:

* amd64
* arm64
* armv6

## Configuration

|                             |                                                              |
| --------------------------- | ------------------------------------------------------------ |
| **bot.token**               | api token telegram provides                                  |
| **chat.id**                 | telegram group the bot is limited with                       |
| **pihole.base_url**         | base URL of pihole API                                       |
| **pihole.token**            | api token pihole provides                                    |
| **pihole.default_duration** | default duration (in seconds) to pause ad-blocking on pihole |

### Example

```
bot:
  token: abc123

chat:
  id: 12345678

pihole:
  base_url: 192.168.1.2
  token: api_token_for_pihole
  default_duration: 600
```

## Run

```
$ docker run -d \
    --name homebot \
    --restart unless-stopped \
    -v ~/.config/homebot:/etc/homebot \
    -e APP_CONFIG=/etc/homebot \
    -e RUST_LOG=info \
    ghcr.io/tunix/homebot
```
