mod magiceden_stats_response;
mod solanart_stats_response;
mod digital_eyes;

use std::env;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use digital_eyes::digitaleyes_stats_response::DigitalEyesResponse;
use digital_eyes::digitaleyes_api::handle_digitaleyes;
use magiceden_stats_response::MagicEdenResponse;
use solanart_stats_response::SolanartResponse;

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
            let mut floor_price = 0.0 as f64;
            let mut error_message = "".to_string();
            if msg.content.contains("magiceden") {
                //let marketplace = split_input_string_tokens[2].to_string();
                let magiceden_stats_response = tokio::spawn(get_magic_eden_json(collection_name.to_owned())).await.unwrap();
                floor_price = magiceden_stats_response.unwrap().results.floor_price as f64 / 1000000000 as f64;
            } else if msg.content.contains("solanart") {
                let solanart_stats_response = tokio::spawn(get_solanart_json(collection_name.to_owned())).await.unwrap();
                floor_price = solanart_stats_response.unwrap().floor_price as f64;
            } else if msg.content.contains("digitaleyes") {
                // Handle digitaleyes call
                let (floor_price_temp, error_message_temp) = handle_digitaleyes(collection_name).await;
                floor_price = floor_price_temp;
                error_message = error_message_temp;
            }
            let floor_price_message: String;
            if error_message.is_empty() {
                floor_price_message = "Floor price: ".to_owned() + &*floor_price.to_string() + " SOL";
            } else {
                floor_price_message = error_message.to_owned();
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


async fn get_magic_eden_json(collection_name: String) -> reqwest::Result<MagicEdenResponse> {
    // Build the client using the builder pattern
    let client = reqwest::Client::new();

    // All collections:
    // https://api-mainnet.magiceden.io/all_collections

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

async fn get_solanart_json(collection_name: String) -> reqwest::Result<SolanartResponse> {
    // Build the client using the builder pattern
    let client = reqwest::Client::new();

    // To get all collections:
    // https://qzlsklfacc.medianetwork.cloud/query_volume_all

    // Perform the actual execution of the network request
    let response = client
        .get(format!("https://qzlsklfacc.medianetwork.cloud/get_floor_price?collection={}", collection_name))
        .header("accept", "*/*")
        .header("origin", "https://solanart.io")
        .header("referer", "https://solanart.io/")
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.93 Safari/537.36")
        .send().await
        .unwrap();

    return response.json::<SolanartResponse>().await;
}