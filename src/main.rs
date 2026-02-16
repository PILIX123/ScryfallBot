mod models;
use std::env;

use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
use models::cards;
use regex::Regex;
use serde::Deserialize;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

const SCRYFALL_API: &'static str = "https://api.scryfall.com";
const CARDS_SEARCH: &'static str = "/cards/search";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

struct CardSearch {
    pub q: String,
    pub unique: Option<String>,
    pub order: Option<String>,
    pub dir: Option<String>,
    pub include_extras: Option<bool>,
    pub include_multilingual: Option<bool>,
    pub include_variations: Option<bool>,
    pub page: Option<i32>,
    pub format: Option<String>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let re = Regex::new(r"\[\[(.*)\]\]").unwrap();
        let Some(caps) = re.captures(&msg.content) else {
            return;
        };
        if let Err(why) = msg
            .channel_id
            .say(&ctx.http, format!("CardName: {}", &caps[1]))
            .await
        {
            println!("Error sending message: {why:?}");
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn fetch_json(url: hyper::Uri) {}

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

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
