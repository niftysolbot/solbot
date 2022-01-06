use urlencoding::encode;
use reqwest::{Error, Response};
use crate::alpha_art::alpha_art_all_collection_response::AlphaArtAllCollectionResponse;
use super::alpha_art_stats_response::AlphaArtResponse;


pub async fn handle_alpha_art(collection_name: String) -> String {
    return match tokio::spawn(get_alpha_art_json(encode(&collection_name.to_owned()))).await.unwrap() {
        Ok(alpha_art_stats_response) => {
            // Handle json failure
            match alpha_art_stats_response.json::<AlphaArtResponse>().await {
                Ok(json_parsed_response) => (format!("Alpha Art: {} SOL\n", json_parsed_response.floor_price.parse::<i64>().unwrap() as f64 / 1000000000 as f64)),
                Err(json_error) => {
                    println!("Problem calling alphaart api json: {:?}", json_error);
                    String::from(format!("Alpha Art: Could not get response. Check https://alpha.art/collection/{}", encode(&collection_name.to_owned())))
                }
            }
        }
        Err(error) => {
            println!("Problem calling alphaart api: {:?}", error);
            String::from("Alpha Art: Could not get response from Alpha Art")
        }
    };
}

async fn get_alpha_art_json(collection_name: String) -> Result<Response, Error> {
    println!("Collection name alpha art: {}", collection_name);

    // Build the client using the builder pattern
    let client = reqwest::Client::new();

    // To get all collections:
    // https://us-central1-digitaleyes-prod.cloudfunctions.net/collection-retriever

    // Perform the actual execution of the network request
    let response = client
        .get(format!("https://apis.alpha.art/api/v1/collection/{}", collection_name))
        .header("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
        .header("accept-language", "en-US,en;q=0.9")
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.93 Safari/537.36")
        .send().await;

    return response;
}


pub async fn alpha_art_process_all_collections_api() -> AlphaArtAllCollectionResponse {
    return match get_all_alpha_art_collections_json().await {
        Ok(alpha_art_response) => {
            // Handle json failure
            match alpha_art_response.json::<AlphaArtAllCollectionResponse>().await {
                Ok(json_parsed_response) => {
                    json_parsed_response
                },
                Err(json_error) => {
                    println!("Problem calling Digital Eyes all collections api json: {:?}", json_error);
                    panic!("Error getting Digital Eyes all collections");
                }
            }
        }
        Err(error) => {
            println!("Problem calling Magic Eden all collections api: {:?}", error);
            panic!("Error getting Digital Eyes all collections");
        }
    }
}


// pub async fn handle_alpha_art_all_collections() -> HashMap<String, PfpCollection> {
//     return match tokio::spawn(get_all_alpha_art_collections_json()).await.unwrap() {
//         Ok(alpha_art_response) => {
//             // Handle json failure
//             match alpha_art_response.json::<AlphaArtAllCollectionResponse>().await {
//                 Ok(json_parsed_response) => {
//                     initialize_pfp_collection_from_alpha_art(json_parsed_response).await
//                 },
//                 Err(json_error) => {
//                     println!("Problem calling Alpha Art all collections api json: {:?}", json_error);
//                     panic!("Error getting Alpha Art all collections");
//                 }
//             }
//         }
//         Err(error) => {
//             println!("Problem calling Alpha Art all collections api: {:?}", error);
//             panic!("Error getting Alpha Art all collections");
//         }
//     };
// }

async fn get_all_alpha_art_collections_json() -> Result<Response, Error> {
    // Build the client using the builder pattern
    let client = reqwest::Client::new();

    // Alternative to get all collections:
    // https://qzlsklfacc.medianetwork.cloud/query_volume_all

    // Perform the actual execution of the network request
    let response = client
        .get("https://apis.alpha.art/api/v1/collections")
        .header("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
        .header("accept-language", "en-US,en;q=0.9")
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.93 Safari/537.36")
        .send().await;

    return response;
}

