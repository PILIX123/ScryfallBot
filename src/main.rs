pub mod models;
use std::env;

use http::{HeaderMap, HeaderValue, Uri};
use regex::Regex;
use serenity::Client;
use serenity::all::GatewayIntents;
use serenity::all::{Context, EventHandler, Message, Ready};
use serenity::async_trait;
use serenity::prelude::TypeMapKey;

use crate::models::cards::Card;
use crate::models::queries::{self, ReturnList};
struct Handler;
struct ReqwestClient;

impl TypeMapKey for ReqwestClient {
    type Value = reqwest::Client;
}

const SCRYFALL_API: &'static str = "https://api.scryfall.com";
const CARDS_SEARCH: &'static str = "/cards/search";

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let data = ctx.data.read().await;
        let Some(http_c) = data.get::<ReqwestClient>() else {
            return;
        };
        let http_client = http_c.clone();
        let re = Regex::new(r"\[\[(.*)\]\]").unwrap();
        let Some(caps) = re.captures(&msg.content) else {
            return;
        };
        let mut card: Card = Card::default();
        if let Err(err) =
            fetch_json(http_client, String::from(caps[1].to_string()), &mut card).await
        {
            println!("{}", err);
            panic!();
        }

        if let Err(why) = msg
            .channel_id
            .say(&ctx.http, format!("{} costs {}", card.name, card.cmc))
            .await
        {
            println!("Error sending message: {why:?}");
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn fetch_json(
    http_client: reqwest::Client,
    card: String,
    return_card: &mut Card,
) -> Result<(), reqwest::Error> {
    let card_search = queries::CardSearch {
        q: card,
        unique: None,
        order: None,
        dir: None,
        include_extras: None,
        include_multilingual: None,
        include_variations: None,
        page: None,
        format: None,
    };

    let config: serde_qs::Config = serde_qs::Config::new().use_form_encoding(true);
    let search: String = config.serialize_string(&card_search).unwrap();
    let url: String = format!("{}{}?{}", SCRYFALL_API, CARDS_SEARCH, search);

    let res = http_client.get(url).send().await?.error_for_status()?;
    let content = res.text().await.unwrap();
    println!("{}", content);
    let t = match serde_json::from_str::<ReturnList<Card>>(&content) {
        Ok(t) => t,
        Err(e) => {
            println!("Parse error: {e}");
            panic!("");
        }
    };

    *return_card = t.data.into_iter().next().unwrap();
    return Ok(());
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
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        let mut headers = HeaderMap::new();
        headers.append(http::header::ACCEPT, HeaderValue::from_str("*/*").unwrap());
        //TODO: Make Guards
        headers.append(
            http::header::USER_AGENT,
            HeaderValue::from_str("PILIXScryfallBot/0.1").unwrap(),
        );
        //TODO: Make Guards
        let http_cli = reqwest::Client::builder()
            .http2_prior_knowledge()
            .default_headers(headers)
            .build()
            .unwrap();
        data.insert::<ReqwestClient>(http_cli);
    }

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
