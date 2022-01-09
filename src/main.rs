mod solanart;
mod digital_eyes;
mod magiceden;
mod alpha_art;
mod collection;

use std::collections::HashMap;
use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::futures::future;

use alpha_art::alpha_art_api::handle_alpha_art;
use digital_eyes::digitaleyes_api::handle_digitaleyes;
use magiceden::magiceden_api::handle_magiceden;
use solanart::solanart_api::handle_solanart;
use collection::all_collections_handling::check_if_collection_exists_or_give_suggestions;
use collection::all_collections_handling::PfpCollection;
use collection::all_collections_handling::{populate_alphaart, populate_digitaleyes, populate_magiceden, populate_solanart};
use collection::collections_initializer::combine_pfp_collections_base_magic_eden;
use crate::collection::collections_initializer::{initialize_pfp_collection_from_alpha_art, initialize_pfp_collection_from_digital_eyes, initialize_pfp_collection_from_magic_eden, initialize_pfp_collection_from_solanart};


struct Bot {
    pfp_collections: HashMap<String, PfpCollection>,
}

#[async_trait]
impl EventHandler for Bot {
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
            //let split_input_string_tokens: Vec<&str> = msg.content.split(" ").collect();
            let collection_name = msg.content.get(7..).unwrap().to_lowercase();

            let (does_collection_exist, suggestions) = check_if_collection_exists_or_give_suggestions(&self.pfp_collections, &*collection_name).await;
            let discord_response_message: String;
            if !does_collection_exist {
                discord_response_message = construct_suggestions_message(suggestions);
            } else {
                let tuple = future::join4(
                    populate_solanart( &collection_name, &self.pfp_collections),
                    populate_magiceden( &collection_name, &self.pfp_collections),
                    populate_digitaleyes( &collection_name, &self.pfp_collections),
                    populate_alphaart( &collection_name, &self.pfp_collections),
                ).await;
                discord_response_message = construct_response_message(&tuple);
            }

            // Send final response to discord
            if let Err(why) = msg.channel_id.say(&ctx.http, discord_response_message).await {
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

    let tuple = future::join4(
        initialize_pfp_collection_from_magic_eden(),
        initialize_pfp_collection_from_solanart(),
        initialize_pfp_collection_from_digital_eyes(),
        initialize_pfp_collection_from_alpha_art()
    ).await;


    let pfp_collections = combine_pfp_collections_base_magic_eden(tuple.0, tuple.1, tuple.2, tuple.3).await;


    let bot = Bot {
        pfp_collections,
    };

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token).event_handler(bot).await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

fn construct_suggestions_message(suggestions: Vec<&str>) -> String {
    let mut collection_not_found_msg = String::from("No collections found");
    if suggestions.len() > 0 {
        println!("No collections found. Suggestions: {:?}", suggestions);
        collection_not_found_msg.push_str("\nDid you mean:\n");
        for sug in suggestions {
            collection_not_found_msg.push_str(&format!("- {}\n", sug));
        }
    }
    collection_not_found_msg
}

fn construct_response_message(tuple: &(String, String, String, String)) -> String {
    let mut floor_price_message = String::from("");
    if !tuple.0.is_empty() { floor_price_message.push_str(&format!("{}\n", tuple.0).to_string()) };
    if !tuple.1.is_empty() { floor_price_message.push_str(&format!("{}\n", tuple.1).to_string()) };
    if !tuple.2.is_empty() { floor_price_message.push_str(&format!("{}\n", tuple.2).to_string()) };
    if !tuple.3.is_empty() { floor_price_message.push_str(&format!("{}", tuple.3).to_string()) };
    floor_price_message
}