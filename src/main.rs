mod solanart;
mod digital_eyes;
mod magiceden;
mod alpha_art;
mod all_collections_handling;

use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::futures::future;

use all_collections_handling::initialize_pfp_collection_from_magic_eden;
use all_collections_handling::initialize_pfp_collection_from_solanart;
use digital_eyes::digitaleyes_api::handle_digitaleyes;
use solanart::solanart_api::handle_solanart;
use magiceden::magiceden_api::handle_magiceden;
use magiceden::magiceden_api::handle_magic_eden_all_collections;
use alpha_art::alpha_art_api::handle_alpha_art;
use solanart::solanart_api::handle_solanart_all_collections;
use digital_eyes::digitaleyes_api::handle_digital_eyes_all_collections;
use all_collections_handling::initialize_pfp_collection_from_digital_eyes;
use all_collections_handling::combine_pfp_collections;

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

        if msg.content.len() > 7 && msg.content.get(0..7).unwrap() == ("!floor ") {
            let split_input_string_tokens: Vec<&str> = msg.content.split(" ").collect();
            let collection_name = split_input_string_tokens[1].to_string().to_lowercase(); //TODO: add ".to_lowercase" on the end

            let tuple = future::join4(
                populate_solanart(msg.content.clone(), split_input_string_tokens.len(), &collection_name),
                populate_magiceden(msg.content.clone(), split_input_string_tokens.len(), &collection_name),
                populate_digitaleyes(msg.content.clone(), split_input_string_tokens.len(), &collection_name),
                populate_alphaart(msg.content.clone(), split_input_string_tokens.len(), &collection_name),
            ).await;

            if let Err(why) = msg.channel_id.say(&ctx.http, construct_response_message(&tuple)).await {
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

    let solanart_all_collections = handle_solanart_all_collections().await;
    let magic_eden_all_collections = handle_magic_eden_all_collections().await;
    let digital_eyes_all_collections = handle_digital_eyes_all_collections().await;
    let pfp_collections_magic_eden = initialize_pfp_collection_from_magic_eden(magic_eden_all_collections).await;
    let pfp_collections_solanart = initialize_pfp_collection_from_solanart(solanart_all_collections).await;
    let pfp_collections_digital_eyes = initialize_pfp_collection_from_digital_eyes(digital_eyes_all_collections).await;
    let pfp_collections = combine_pfp_collections(pfp_collections_magic_eden, pfp_collections_solanart, pfp_collections_digital_eyes).await;
    match pfp_collections.get("degenerate ape academy") {
        Some(s) => {
            match s.slug.get("SOLANART") {
                Some(t) => {
                    println!("Turtles solanart: {}", t)
                },
                _ => {}
            }
            match s.slug.get("DIGITAL_EYES") {
                Some(t) => {
                    println!("Turtles digital eyes: {}", t)
                },
                _ => {}
            }
            match s.slug.get("MAGIC_EDEN") {
                Some(t) => {
                    println!("Turtles magic eden: {}", t)
                },
                _ => {}
            }
        },
        _ => {}
    }

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

async fn populate_digitaleyes(msg_content: String, token_len: usize, collection_name: &String) -> String {
    if msg_content.contains("digitaleyes") || token_len == 2 {
        return handle_digitaleyes(collection_name.to_owned()).await;
    }
    return String::from("");
}


async fn populate_solanart(msg_content: String, token_len: usize, collection_name: &String) -> String {
    if msg_content.contains("solanart") || token_len == 2 {
        println!("solanart");
        return handle_solanart(collection_name.to_owned()).await;
    }
    return String::from("");
}

async fn populate_magiceden(msg_content: String, token_len: usize, collection_name: &String) -> String {
    if msg_content.contains("magiceden") || token_len == 2 {
        println!("magiceden");
        return handle_magiceden(collection_name.to_owned()).await;
    }
    return String::from("");
}

async fn populate_alphaart(msg_content: String, token_len: usize, collection_name: &String) -> String {
    if msg_content.contains("alphaart") || token_len == 2 {
        println!("alphaart");
        return handle_alpha_art(collection_name.to_owned()).await;
    }
    return String::from("");
}

fn construct_response_message(tuple: &(String, String, String, String)) -> String {
    let mut floor_price_message = String::from("Floor Prices:\n");
    if !tuple.0.is_empty() { floor_price_message.push_str(&format!("{}\n", tuple.0).to_string()) };
    if !tuple.1.is_empty() { floor_price_message.push_str(&format!("{}\n", tuple.1).to_string()) };
    if !tuple.2.is_empty() { floor_price_message.push_str(&format!("{}\n", tuple.2).to_string()) };
    if !tuple.3.is_empty() { floor_price_message.push_str(&format!("{}", tuple.3).to_string()) };
    floor_price_message
}