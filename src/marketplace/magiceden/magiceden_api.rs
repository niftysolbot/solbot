use reqwest::{Error, Response};
use crate::PfpCollectionEntry;
use std::collections::{HashMap};
use crate::marketplace::marketplace::MarketplaceCollection;
use serenity::{
    async_trait,
};
use crate::marketplace::magiceden::magiceden_all_collection_response::MagicEdenAllCollectionsResponse;
use crate::marketplace::magiceden::magiceden_stats_response::MagicEdenResponse;

pub struct MagicEden {
    name: String,
}

impl MagicEden {
    async fn call_single_collection_api(&mut self, collection_name: String) -> Result<Response, Error> {
        println!("Collection name magic eden: {}", collection_name);

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

    pub async fn call_all_collections_api(&mut self) -> MagicEdenAllCollectionsResponse {
        return match self.make_all_collections_api_call().await {
            Ok(magic_eden_response) => {
                // Handle json failure
                match magic_eden_response.json::<MagicEdenAllCollectionsResponse>().await {
                    Ok(json_parsed_response) => {
                        json_parsed_response
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

    async fn make_all_collections_api_call(&mut self) -> Result<Response, Error> {
        // Build the client using the builder pattern
        let client = reqwest::Client::new();

        // Perform the actual execution of the network request
        let response = client
            .get("https://api-mainnet.magiceden.io/all_collections_with_escrow_data")
            .header("Accept", "application/json, text/plain, */*")
            .header("Referer", "https://magiceden.io/")
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.93 Safari/537.36")
            .send().await;

        return response;
    }
}

#[async_trait]
impl MarketplaceCollection for MagicEden {
    fn new(name: String) -> MagicEden {
        MagicEden { name }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    async fn get_floor_from_api(&mut self, pfp_collection: &PfpCollectionEntry) -> String {
        return match pfp_collection.slug.get("MAGIC_EDEN") { // check if there exists an api slug mapping for Solanart
            None => String::from(""),
            Some(collection_name) => {
                match self.call_single_collection_api(collection_name.to_owned()).await {
                    Ok(magiceden_stats_response) => {
                        // Handle json failure
                        match magiceden_stats_response.json::<MagicEdenResponse>().await {
                            Ok(json_parsed_response) => (format!("Magic Eden: {} SOL", json_parsed_response.results.floor_price as f64 / 1000000000 as f64)),
                            Err(json_error) => {
                                println!("Problem calling Magic Eden api (json parse): {:?}", json_error);
                                String::from("")
                            }
                        }
                    }
                    Err(error) => {
                        println!("Problem calling Magic Eden api: {:?}", error);
                        String::from("Magic Eden: Could not get response from Magic Eden")
                    }
                }
            }
        }
    }

    async fn initialize_pfp_collections(&mut self) -> HashMap<String, PfpCollectionEntry> {
        let magic_eden_response = self.call_all_collections_api().await;
        let mut pfp_collections: HashMap<String, PfpCollectionEntry> = HashMap::new();
        for magic_eden_collection in magic_eden_response.collections {
            let mut slug: HashMap<String, String> = HashMap::new();
            slug.insert("MAGIC_EDEN".parse().unwrap(), magic_eden_collection.symbol);
            let collection = PfpCollectionEntry {
                name: String::from(magic_eden_collection.name.to_lowercase().trim()),
                slug,
                website: magic_eden_collection.website,
                twitter: magic_eden_collection.twitter,
                discord: magic_eden_collection.discord,
                suggestions: Vec::new()
            };
            match pfp_collections.get(&*collection.name.to_lowercase()) {
                Some(p) => println!("WARNING [Magic Eden]: collection already exists: {:?}", p),
                None => {
                    pfp_collections.insert(collection.name.to_lowercase(), collection);
                },
            }
        }
        println!("Magic Eden Pfp collection size: {}", pfp_collections.len());
        pfp_collections
    }

}