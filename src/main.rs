pub mod models;
use std::env;

use http::{HeaderMap, HeaderValue};
use regex::Regex;
use serenity::Client;
use serenity::all::GatewayIntents;
use serenity::all::{Context, EventHandler, Http, Message, Ready};
use serenity::async_trait;
use serenity::prelude::TypeMapKey;

use crate::models::cards::Card;
use crate::models::queries::{self};
use crate::models::symbols::Symbols;
struct Handler;
struct ReqwestClient;

impl TypeMapKey for ReqwestClient {
    type Value = reqwest::Client;
}

struct SerdeQsConfig;

impl TypeMapKey for SerdeQsConfig {
    type Value = serde_qs::Config;
}

const SCRYFALL_API: &'static str = "https://api.scryfall.com";
const CARDS_NAMED: &'static str = "/cards/named";

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        let data = ctx.data.read().await;
        let Some(http_c) = data.get::<ReqwestClient>() else {
            return;
        };
        let Some(conf) = data.get::<SerdeQsConfig>() else {
            return;
        };
        let http_client = http_c.clone();
        let re = Regex::new(r"\[\[(.*)\]\]").unwrap();
        let Some(caps) = re.captures(&msg.content) else {
            return;
        };
        let card: Card =
            match fetch_json(http_client, conf, String::from(caps[1].to_string())).await {
                Ok(res) => {
                    let content = res.text().await.unwrap();
                    //TODO: Deal with double sided cards
                    let c = serde_json::from_str::<Card>(&content).unwrap_or_else(|error| {
                        println!("Parse error: {error}");
                        panic!("");
                    });
                    c
                }
                Err(err) => match err.status() {
                    Some(a) if a.is_client_error() => {
                        match a.as_u16() {
                            404 => {
                                send_message(
                                    msg,
                                    &ctx.http,
                                    String::from("No card with this name exist"),
                                )
                                .await
                            }
                            _ => {
                                send_message(
                                    msg,
                                    &ctx.http,
                                    String::from("Sounds like a problem for the bot maintainer"),
                                )
                                .await
                            }
                        }

                        return;
                    }
                    Some(_a) => {
                        send_message(
                            msg,
                            &ctx.http,
                            String::from("There was an error plz try again later"),
                        )
                        .await;
                        return;
                    }
                    None => return,
                },
            };

        send_message(
            msg,
            &ctx.http,
            format!(
                "{} costs {} and reads \n\"{}\"",
                card.name,
                parse_symbols(card.mana_cost),
                parse_symbols(card.oracle_text),
            ),
        )
        .await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn send_message(msg: Message, http_ctx: &Http, message: String) {
    if let Err(why) = msg.channel_id.say(http_ctx, message).await {
        println!("Error sending message: {why:?}");
    }
}
async fn fetch_json(
    http_client: reqwest::Client,
    config: &serde_qs::Config,
    card: String,
) -> Result<reqwest::Response, reqwest::Error> {
    let card_search = queries::CardNamed {
        name: queries::CardName::Fuzzy(card),
        set: None,
        format: None,
        face: None,
        version: None,
        pretty: None,
    };

    let search: String = config.serialize_string(&card_search).unwrap();
    let url: String = format!("{}{}?{}", SCRYFALL_API, CARDS_NAMED, search);

    let res = http_client.get(url).send().await?.error_for_status()?;
    return Ok(res);
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    #[cfg(debug_assertions)]
    {
        let _ = loadenv::load();
    }
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let config: serde_qs::Config = serde_qs::Config::new();
        let mut data = client.data.write().await;
        let mut headers = HeaderMap::new();
        headers.append(http::header::ACCEPT, HeaderValue::from_str("*/*").unwrap());
        headers.append(
            http::header::USER_AGENT,
            HeaderValue::from_str("PILIXScryfallBot/0.1").unwrap(),
        );
        let http_cli = reqwest::Client::builder()
            .http2_prior_knowledge()
            .default_headers(headers)
            .build();

        match http_cli {
            Ok(cli) => data.insert::<ReqwestClient>(cli),
            Err(err) => panic!("Couldnt make the Http clien {:?}", err),
        }
        data.insert::<SerdeQsConfig>(config);
    }

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

fn parse_symbols(symbol_string: Option<String>) -> String {
    let oracle = match symbol_string {
        Some(text) => text,
        None => return String::from(""),
    };

    let re = Regex::new(r"(\{[\w/∞½]*\})").unwrap();

    re.replace_all(&oracle, |caps: &regex::Captures| {
        match Symbols::try_from(&caps[0]) {
            Ok(symbol) => symbol.value().to_string(),
            Err(_) => caps[0].to_string(),
        }
    })
    .to_string()
}
