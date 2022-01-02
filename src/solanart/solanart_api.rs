use reqwest::{Error, Response};
use super::solanart_stats_response::SolanartResponse;
use super::solanart_all_collection_response::SolanartAllCollectionResponse;

pub async fn handle_solanart(collection_name: String) -> String {
    return match tokio::spawn(get_solanart_json(collection_name.to_owned())).await.unwrap() {
        Ok(solanart_stats_response) => {
            // Handle json failure
            match solanart_stats_response.json::<SolanartResponse>().await {
                Ok(json_parsed_response) => (format!("Solanart: {} SOL", json_parsed_response.floor_price as f64)),
                Err(json_error) => {
                    println!("Problem calling Solanart api json: {:?}", json_error);
                    String::from("Solanart: Could not get response from Solanart")
                }
            }
        }
        Err(error) => {
            println!("Problem calling Solanart api: {:?}", error);
            String::from("Solanart: Could not get response from Solanart")
        }
    };
}

async fn get_solanart_json(collection_name: String) -> Result<Response, Error> {
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
        .send().await;
    return response;
}

pub async fn handle_solanart_all_collections() -> SolanartAllCollectionResponse {
    return match tokio::spawn(get_all_solanart_collections_json()).await.unwrap() {
        Ok(solanart_response) => {
            // Handle json failure
            match solanart_response.json::<SolanartAllCollectionResponse>().await {
                Ok(json_parsed_response) => {
                    json_parsed_response
                },
                Err(json_error) => {
                    println!("Problem calling Solanart all collections api json: {:?}", json_error);
                    panic!("Error getting Solanart all collections api json")
                }
            }
        }
        Err(error) => {
            println!("Problem calling Solanart all collections api: {:?}", error);
            panic!("Error getting Solanart all collections api json")
        }
    };
}

async fn get_all_solanart_collections_json() -> Result<Response, Error> {
    // Build the client using the builder pattern
    let client = reqwest::Client::new();

    // Alternative to get all collections:
    // https://qzlsklfacc.medianetwork.cloud/query_volume_all

    // Perform the actual execution of the network request
    let response = client
        .get("https://qzlsklfacc.medianetwork.cloud/get_collections")
        .header("accept", "*/*")
        .header("origin", "https://solanart.io")
        .header("referer", "https://solanart.io/")
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.93 Safari/537.36")
        .send().await;
    return response;
}