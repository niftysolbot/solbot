use urlencoding::encode;
use reqwest::{Error, Response};
use crate::{MarketplaceCollection, PfpCollectionEntry};
use std::collections::{HashMap};
use serenity::{
    async_trait,
};
use crate::marketplace::digital_eyes::digital_eyes_all_collection_response::DigitalEyesAllCollectionResponse;
use crate::marketplace::digital_eyes::digitaleyes_stats_response::DigitalEyesResponse;

pub struct DigitalEyes {
    name: String,
}

impl DigitalEyes {
    async fn call_single_collection_api(&mut self, collection_name: String) -> Result<Response, Error> {
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

    pub async fn call_all_collections_api(&mut self) -> DigitalEyesAllCollectionResponse {
        return match self.make_all_collections_api_call().await {
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

    async fn make_all_collections_api_call(&mut self) -> Result<Response, Error> {
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

}

#[async_trait]
impl MarketplaceCollection for DigitalEyes {
    fn new(name: String) -> DigitalEyes {
        DigitalEyes { name }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    async fn get_floor_from_api(&mut self, pfp_collection: &PfpCollectionEntry) -> String {
        return match pfp_collection.slug.get("DIGITAL_EYES") { // check if there exists an api slug mapping for Solanart
            None => String::from(""),
            Some(collection_name) => {
                match self.call_single_collection_api(encode(&collection_name.to_owned())).await {
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

    async fn initialize_pfp_collections(&mut self) -> HashMap<String, PfpCollectionEntry> {
        let digital_eyes_response = self.call_all_collections_api().await;
        let mut pfp_collections: HashMap<String, PfpCollectionEntry> = HashMap::new();
        for digital_eyes_collection in digital_eyes_response {
            let mut slug: HashMap<String, String> = HashMap::new();
            slug.insert("DIGITAL_EYES".parse().unwrap(), digital_eyes_collection.name.clone());

            let collection = PfpCollectionEntry {
                name: String::from(digital_eyes_collection.name.to_lowercase().trim()),
                slug,
                website: digital_eyes_collection.website,
                twitter: None,
                discord: None,
                suggestions: Vec::new()
            };

            pfp_collections.insert(collection.name.to_lowercase(), collection);
        }
        println!("Digital Eyes Pfp collection size: {}", pfp_collections.len());
        pfp_collections
    }
}