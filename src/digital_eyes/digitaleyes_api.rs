use urlencoding::encode;
use reqwest::{Error, Response};
use super::super::PfpCollectionEntry;
use super::digital_eyes_all_collection_response::DigitalEyesAllCollectionResponse;
use super::digitaleyes_stats_response::DigitalEyesResponse;


pub async fn handle_digitaleyes(pfp_collection: &PfpCollectionEntry) -> String {
    return match pfp_collection.slug.get("DIGITAL_EYES") { // check if there exists an api slug mapping for Solanart
        None => String::from(""),
        Some(collection_name) => {
            match tokio::spawn(get_digitaleyes_json(encode(&collection_name.to_owned()))).await.unwrap() {
                Ok(digitaleyes_stats_response) => {
                    // Handle json failure
                    match digitaleyes_stats_response.json::<DigitalEyesResponse>().await {
                        Ok(json_parsed_response) => (format!("Digital Eyes: {} SOL", json_parsed_response.price_floor as f64 / 1000000000 as f64)),
                        Err(json_error) => {
                            println!("Problem calling digitaleyes api json: {:?}", json_error);
                            //String::from(format!("Digital Eyes: Could not get response from Digitaleyes. Check https://digitaleyes.market/collections/{}", encode(&collection_name.to_owned())))
                            String::from("")
                        }
                    }
                }
                Err(error) => {
                    println!("Problem calling digitaleyes api: {:?}", error);
                    String::from("Digital Eyes: Could not get response from Digitaleyes")
                }
            }
        }
    }
}

async fn get_digitaleyes_json(collection_name: String) -> Result<Response, Error> {
    println!("Collection name digital eyes: {}", collection_name);
    // Build the client using the builder pattern
    let client = reqwest::Client::new();

    // To get all collections:
    // https://us-central1-digitaleyes-prod.cloudfunctions.net/collection-retriever

    // Perform the actual execution of the network request
    let response = client
        .get(format!("https://us-central1-digitaleyes-prod.cloudfunctions.net/offers-retriever?collection={}&price=asc", collection_name))
        .header("accept", "*/*")
        .header("accept-language", "en-US,en;q=0.9")
        .header("referer", "https://digitaleyes.market/")
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.93 Safari/537.36")
        .send().await;

    return response;
}

pub async fn digital_eyes_process_all_collections_api() -> DigitalEyesAllCollectionResponse {
    return match get_all_digital_eyes_collections_json().await {
        Ok(digital_eyes_response) => {
            // Handle json failure
            match digital_eyes_response.json::<DigitalEyesAllCollectionResponse>().await {
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
    };
}

async fn get_all_digital_eyes_collections_json() -> Result<Response, Error> {
    // Build the client using the builder pattern
    let client = reqwest::Client::new();

    // Alternative to get all collections:
    // https://qzlsklfacc.medianetwork.cloud/query_volume_all

    // Perform the actual execution of the network request
    let response = client
        .get("https://us-central1-digitaleyes-prod.cloudfunctions.net/collection-retriever")
        .header("accept", "*/*")
        .header("accept-language", "en-US,en;q=0.9")
        .header("referer", "https://digitaleyes.market/")
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.93 Safari/537.36")
        .send().await;

    return response;
}