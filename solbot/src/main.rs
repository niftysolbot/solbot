mod magiceden_stats_response;

use std::env;
use std::collections::HashMap;
use reqwest::blocking::get;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::futures::TryFutureExt;
use crate::magiceden_stats_response::MagicEdenResponse;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!is this rugged" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Definitely yes.").await {
                println!("Error sending message: {:?}", why);
            }
        }
        // if msg.content == "!get me boryoku_dragonz magiceden json" {
        if msg.content.contains("!get me") {
            let split_input_string_tokens:Vec<&str>= msg.content.split(" ").collect();
            let magiceden_stats_response = tokio::spawn(get_magic_eden_json(split_input_string_tokens[2].to_string())).await.unwrap();
            let floor_price = magiceden_stats_response.unwrap().results.floor_price as f64 / 1000000000 as f64;
            let floor_price_message = "Floor price: ".to_owned() + &*floor_price.to_string();


            if let Err(why) = msg.channel_id.say(&ctx.http, floor_price_message).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token).event_handler(Handler).await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}


async fn get_magic_eden_json(collection_name: String) -> reqwest::Result<MagicEdenResponse> {
    // Build the client using the builder pattern
    let client = reqwest::Client::new();

    // Perform the actual execution of the network request
    let response = client
        .get(format!("https://api-mainnet.magiceden.io/rpc/getCollectionEscrowStats/{}", collection_name))
        .header("Accept", "application/json, text/plain, */*")
        .header("Referer", "https://magiceden.io/")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.93 Safari/537.36")
        .send().await
        .unwrap();

    return response.json::<MagicEdenResponse>().await;
}