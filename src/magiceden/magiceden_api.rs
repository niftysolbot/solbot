use std::collections::{HashMap};
use reqwest::{Error, Response};
use super::super::collection::all_collections_handling::{initialize_pfp_collection_from_magic_eden, PfpCollection};
use super::magiceden_all_collection_response::MagicEdenAllCollectionsResponse;
use super::magiceden_stats_response::MagicEdenResponse;

pub async fn handle_magiceden(collection_name: String) -> String {
    return match tokio::spawn(get_magic_eden_json(collection_name.to_owned())).await.unwrap() {
        Ok(magiceden_stats_response) => {
            // Handle json failure
            match magiceden_stats_response.json::<MagicEdenResponse>().await {
                Ok(json_parsed_response) => (format!("Magic Eden: {} SOL", json_parsed_response.results.floor_price as f64 / 1000000000 as f64)),
                Err(json_error) => {
                    println!("Problem calling Magic Eden api (json parse): {:?}", json_error);
                    String::from("Magic Eden: Could not get response from Magic Eden")
                }
            }
        }
        Err(error) => {
            println!("Problem calling Magic Eden api: {:?}", error);
            String::from("Magic Eden: Could not get response from Magic Eden")
        }
    };
}

async fn get_magic_eden_json(collection_name: String) -> Result<Response, Error> {
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
        .send().await;

    return response;
}

pub async fn handle_magic_eden_all_collections() -> HashMap<String, PfpCollection> {
    return match tokio::spawn(get_all_magic_eden_collections_json()).await.unwrap() {
        Ok(magic_eden_response) => {
            // Handle json failure
            match magic_eden_response.json::<MagicEdenAllCollectionsResponse>().await {
                Ok(json_parsed_response) => {
                    initialize_pfp_collection_from_magic_eden(json_parsed_response).await
                },
                Err(json_error) => {
                    println!("Problem calling Magic Eden all collections api json: {:?}", json_error);
                    panic!("Error getting magic eden all collections");
                }
            }
        }
        Err(error) => {
            println!("Problem calling Magic Eden all collections api: {:?}", error);
            panic!("Error getting magic eden all collections");
        }
    };
}

async fn get_all_magic_eden_collections_json() -> Result<Response, Error> {
    // Build the client using the builder pattern
    let client = reqwest::Client::new();

    // Alternative to get all collections:
    // https://qzlsklfacc.medianetwork.cloud/query_volume_all

    // Perform the actual execution of the network request
    let response = client
        .get("https://api-mainnet.magiceden.io/all_collections_with_escrow_data")
        .header("Accept", "application/json, text/plain, */*")
        .header("Referer", "https://magiceden.io/")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.93 Safari/537.36")
        .send().await;

    return response;
}