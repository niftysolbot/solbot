use reqwest::{Error, Response};
use super::solanart_stats_response::SolanartResponse;

pub async fn handle_solanart(collection_name: String) -> (f64, String) {
    match tokio::spawn(get_solanart_json(collection_name.to_owned())).await.unwrap() {
        Ok(solanart_stats_response) => {
            // Handle json failure
            match solanart_stats_response.json::<SolanartResponse>().await {
                Ok(json_parsed_response) => return (json_parsed_response.floor_price as f64, "".to_string()),
                Err(json_error) => {
                    println!("Problem calling Solanart api json: {:?}", json_error);
                    return (0.0 as f64, "Could not get response from Solanart".to_string());
                }
            }
        }
        Err(error) => {
            println!("Problem calling Solanart api: {:?}", error);
            return (0.0 as f64, "Could not get response from Solanart".to_string());
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