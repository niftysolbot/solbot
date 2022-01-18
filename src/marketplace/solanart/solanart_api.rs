use reqwest::{Error, Response};
use crate::marketplace::solanart::solanart_all_collection_response::SolanartAllCollectionResponse;
use crate::marketplace::solanart::solanart_stats_response::SolanartResponse;
use crate::PfpCollectionEntry;
use crate::marketplace::marketplace::MarketplaceCollection;
use std::collections::{HashMap};
use serenity::{
    async_trait,
};

pub struct Solanart {
    name: String,
}

impl Solanart {
    async fn call_single_collection_api(&mut self, collection_name: String) -> Result<Response, Error> {
        println!("Collection name solanart: {}", collection_name);

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

    pub async fn call_all_collections_api(&mut self) -> SolanartAllCollectionResponse {
        return match self.make_all_collections_api_call().await {
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

    async fn make_all_collections_api_call(&mut self) -> Result<Response, Error> {
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
}

#[async_trait]
impl MarketplaceCollection for Solanart {
    fn new(name: String) -> Solanart {
        Solanart { name }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    async fn get_floor_from_api(&mut self, pfp_collection: &PfpCollectionEntry) -> String {
        return match pfp_collection.slug.get("SOLANART") { // check if there exists an api slug mapping for Solanart
            None => String::from(""),
            Some(collection_name) => {
                match self.call_single_collection_api(collection_name.to_owned()).await {
                    Ok(solanart_stats_response) => {
                        // Handle json failure
                        match solanart_stats_response.json::<SolanartResponse>().await {
                            Ok(json_parsed_response) => (format!("Solanart: {} SOL", json_parsed_response.floor_price as f64)),
                            Err(json_error) => {
                                println!("Problem calling Solanart api json: {:?}", json_error);
                                String::from("")
                            }
                        }
                    }
                    Err(error) => {
                        println!("Problem calling Solanart api: {:?}", error);
                        String::from("Solanart: Could not get response from Solanart")
                    }
                }
            }
        }
    }

    async fn initialize_pfp_collections(&mut self) -> HashMap<String, PfpCollectionEntry> {
        let solanart_response = self.call_all_collections_api().await;
        let mut pfp_collections: HashMap<String, PfpCollectionEntry> = HashMap::new();
        for solanart_collection in solanart_response {
            let mut slug: HashMap<String, String> = HashMap::new();
            slug.insert("SOLANART".parse().unwrap(), solanart_collection.url);

            let collection = PfpCollectionEntry {
                name: String::from(solanart_collection.name.to_lowercase().trim()),
                slug,
                website: solanart_collection.website,
                twitter: solanart_collection.twitter,
                discord: solanart_collection.discord,
                suggestions: Vec::new()
            };
            match pfp_collections.get(&*collection.name.to_lowercase()) {
                Some(p) => println!("WARNING [Solanart]: collection already exists: {:?}", p),
                None => {
                    pfp_collections.insert(collection.name.to_lowercase(), collection);
                },
            }
        }
        println!("Solanart Pfp collection size: {}", pfp_collections.len());
        pfp_collections
    }

}