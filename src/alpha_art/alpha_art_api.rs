use reqwest::{Error, Response};
use super::alpha_art_stats_response::AlphaArtResponse;


pub async fn handle_alpha_art(collection_name: String) -> String {
    return match tokio::spawn(get_alpha_art_json(collection_name.to_owned())).await.unwrap() {
        Ok(alpha_art_stats_response) => {
            // Handle json failure
            match alpha_art_stats_response.json::<AlphaArtResponse>().await {
                Ok(json_parsed_response) => (format!("Alpha Art: {} SOL\n", json_parsed_response.floor_price.parse::<i64>().unwrap() as f64 / 1000000000 as f64)),
                Err(json_error) => {
                    println!("Problem calling alphaart api json: {:?}", json_error);
                    String::from("Alpha Art: Could not get response from Alpha Art")
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
    // Build the client using the builder pattern
    let client = reqwest::Client::new();

    println!("Collection name: {}", collection_name);

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