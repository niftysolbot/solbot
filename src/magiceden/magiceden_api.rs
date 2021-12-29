use reqwest::{Error, Response};
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