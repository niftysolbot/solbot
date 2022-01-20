mod collection;
mod marketplace;

use std::collections::HashMap;
use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::futures::future;
use tokio::join;

use marketplace::alpha_art::alpha_art_api::{AlphaArt};
use marketplace::marketplace::MarketplaceCollection;
use collection::all_collections_handling::{check_if_collection_exists_or_give_suggestions, PfpCollectionEntry};
use collection::collections_initializer::{ALPHA_ART, combine_pfp_collections, DIGITAL_EYES};
use crate::collection::collections_initializer::SOLANART;
use crate::marketplace::digital_eyes::digitaleyes_api::DigitalEyes;
use crate::marketplace::magiceden::magiceden_api::MagicEden;
use crate::marketplace::solanart::solanart_api::Solanart;


struct Bot {
    pfp_collections: HashMap<String, PfpCollectionEntry>,
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

            let discord_response_message: String;
            let (pfp_collection_option, suggestions) = check_if_collection_exists_or_give_suggestions(&self.pfp_collections, &*collection_name).await;
            let mut alpha_art: AlphaArt = MarketplaceCollection::new(String::from("ALPHA_ART"));
            let mut digital_eyes: DigitalEyes = MarketplaceCollection::new(String::from("DIGITAL_EYES"));
            let mut magic_eden: MagicEden = MarketplaceCollection::new(String::from("MAGIC_EDEN"));
            let mut solanart: Solanart = MarketplaceCollection::new(String::from("SOLANART"));

            match pfp_collection_option {
                Some(pfp_collection_entry) => {
                    let (sol, mag, dig, alph) = join!(
                        solanart.get_floor_from_api( pfp_collection_entry),
                        magic_eden.get_floor_from_api( pfp_collection_entry),
                        digital_eyes.get_floor_from_api( pfp_collection_entry),
                        alpha_art.get_floor_from_api( pfp_collection_entry),
                    );
                    let tuple = (sol, mag, dig, alph);
                    discord_response_message = construct_response_message(&tuple);
                }
                None => {
                    discord_response_message = construct_suggestions_message(suggestions);
                }
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

    let mut alpha_art: AlphaArt = MarketplaceCollection::new(String::from("ALPHA_ART"));
    let mut digital_eyes: DigitalEyes = MarketplaceCollection::new(String::from("DIGITAL_EYES"));
    let mut magic_eden: MagicEden = MarketplaceCollection::new(String::from("MAGIC_EDEN"));
    let mut solanart: Solanart = MarketplaceCollection::new(String::from("SOLANART"));

    let tuple = future::join4(
        magic_eden.initialize_pfp_collections(),
        solanart.initialize_pfp_collections(),
        digital_eyes.initialize_pfp_collections(),
        alpha_art.initialize_pfp_collections()
    ).await;


    let pfp_collections = combine_pfp_collections(tuple.0, &tuple.1, &tuple.2, &tuple.3, (SOLANART, DIGITAL_EYES, ALPHA_ART)).await;
    let mut pfp_collections_updated = combine_pfp_collections(pfp_collections, &tuple.1, &tuple.2, &tuple.3, (SOLANART, DIGITAL_EYES, ALPHA_ART)).await;

    // Manually add degen ape for alpha art since they provide no links
    manually_add_slug_to_pfp_collections(&mut pfp_collections_updated, String::from("degenerate ape academy"), String::from("ALPHA_ART"), String::from("dape"));


    let bot = Bot {
        pfp_collections: pfp_collections_updated,
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

fn manually_add_slug_to_pfp_collections(pfp_collections_updated: &mut HashMap<String, PfpCollectionEntry>, master_collection_name_key: String, marketplace: String, slug: String) {
    let mut collection_to_modify = pfp_collections_updated.get(master_collection_name_key.as_str()).unwrap().clone();
    let mut slug_to_modify = collection_to_modify.slug;
    slug_to_modify.insert(marketplace, slug.parse().unwrap());
    collection_to_modify.slug = slug_to_modify;
    pfp_collections_updated.insert(master_collection_name_key, collection_to_modify);
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