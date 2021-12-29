use reqwest::{Error, Response};
use super::digitaleyes_stats_response::DigitalEyesResponse;


pub async fn handle_digitaleyes(collection_name: String) -> String {
    return match tokio::spawn(get_digitaleyes_json(collection_name.to_owned())).await.unwrap() {
        Ok(digitaleyes_stats_response) => {
            // Handle json failure
            match digitaleyes_stats_response.json::<DigitalEyesResponse>().await {
                Ok(json_parsed_response) => (format!("Digital Eyes: {} SOL", json_parsed_response.price_floor as f64 / 1000000000 as f64)),
                Err(json_error) => {
                    println!("Problem calling digitaleyes api json: {:?}", json_error);
                    String::from("Digital Eyes: Could not get response from Digitaleyes")
                }
            }
        }
        Err(error) => {
            println!("Problem calling digitaleyes api: {:?}", error);
            String::from("Digital Eyes: Could not get response from Digitaleyes")
        }
    };
}

async fn get_digitaleyes_json(collection_name: String) -> Result<Response, Error> {
    // Build the client using the builder pattern
    let client = reqwest::Client::new();

    println!("Collection name: {}", collection_name);

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