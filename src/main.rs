mod solanart;
mod digital_eyes;
mod magiceden;

use std::env;
use std::collections::HashMap;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use digital_eyes::digitaleyes_api::handle_digitaleyes;
use solanart::solanart_api::handle_solanart;
use magiceden::magiceden_api::handle_magiceden;

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
        if msg.content.contains("!floor ") {
            let split_input_string_tokens: Vec<&str> = msg.content.split(" ").collect();
            let collection_name = split_input_string_tokens[1].to_string();
            let mut floor_prices_map = HashMap::new();

            if msg.content.contains("magiceden") {
                let (floor_price, error_message) = handle_magiceden(collection_name.to_owned()).await;
                if error_message.is_empty() {
                    floor_prices_map.insert(String::from("Magic Eden"), floor_price.to_string() + " SOL");
                }
                else {
                    floor_prices_map.insert(String::from("Magic Eden"), error_message);
                }
            }
            if msg.content.contains("solanart") {
                let (floor_price, error_message) = handle_solanart(collection_name.to_owned()).await;
                if error_message.is_empty() {
                    floor_prices_map.insert(String::from("Solanart"), floor_price.to_string() + " SOL");
                }
                else {
                    floor_prices_map.insert(String::from("Solanart"), error_message);
                }
            }
            if msg.content.contains("digitaleyes") {
                // Handle digitaleyes call
                let (floor_price, error_message) = handle_digitaleyes(collection_name.to_owned()).await;
                if error_message.is_empty() {
                    floor_prices_map.insert(String::from("Digital Eyes"), floor_price.to_string() + " SOL");
                }
                else {
                    floor_prices_map.insert(String::from("Digital Eyes"), error_message);
                }
            }
            let mut floor_price_message = String::from("Floor Prices:\n");

            for (marketplace, marketplace_floor_price) in floor_prices_map.iter() {
                floor_price_message.push_str(&*(marketplace.to_owned() + " : " + marketplace_floor_price + "\n"));
            }


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