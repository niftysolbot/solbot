use urlencoding::encode;
use reqwest::{Error, Response};
use super::alpha_art_stats_response::AlphaArtResponse;
use serenity::{
    async_trait,
};
use crate::marketplace::alpha_art::alpha_art_all_collection_response::AlphaArtAllCollectionResponse;
use crate::PfpCollectionEntry;
use std::collections::{HashMap};
use crate::marketplace::marketplace::MarketplaceCollection;


pub struct AlphaArt {
    name: String,
}

impl AlphaArt {

    async fn call_single_collection_api(&mut self, collection_name: String) -> Result<Response, Error> {
        println!("Collection name alpha art: {}", collection_name);

        // Build the client using the builder pattern
        let client = reqwest::Client::new();

        // Perform the actual execution of the network request
        let response = client
            .get(format!("https://apis.alpha.art/api/v1/collection/{}", collection_name))
            .header("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
            .header("accept-language", "en-US,en;q=0.9")
            .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.93 Safari/537.36")
            .send().await;

        return response;
    }

    pub async fn call_all_collections_api(&mut self) -> AlphaArtAllCollectionResponse {
        return match self.make_all_collections_api_call().await {
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

    async fn make_all_collections_api_call(&mut self) -> Result<Response, Error> {
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
}

#[async_trait]
impl MarketplaceCollection for AlphaArt {
    // `Self` is the implementor type: `AlphaArt`.
    fn new(name: String) -> AlphaArt {
        AlphaArt { name }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    async fn get_floor_from_api(&mut self, pfp_collection: &PfpCollectionEntry) -> String {
        return match pfp_collection.slug.get(&*self.name) { // check if there exists an api slug mapping
            None => String::from(""),
            Some(collection_name) => {
                match self.call_single_collection_api(encode(&collection_name.to_owned())).await {
                    Ok(alpha_art_stats_response) => {
                        // Handle json failure
                        match alpha_art_stats_response.json::<AlphaArtResponse>().await {
                            Ok(json_parsed_response) => (format!("Alpha Art: {} SOL\n", json_parsed_response.floor_price.parse::<i64>().unwrap() as f64 / 1000000000 as f64)),
                            Err(json_error) => {
                                println!("Problem calling alphaart api json: {:?}", json_error);
                                //String::from(format!("Alpha Art: Could not get response. Check https://alpha.art/collection/{}", encode(&collection_name.to_owned())))
                                String::from("")
                            }
                        }
                    }
                    Err(error) => {
                        println!("Problem calling alphaart api: {:?}", error);
                        String::from("Alpha Art: Could not get response from Alpha Art")
                    }
                }
            }
        }
    }

    async fn initialize_pfp_collections(&mut self) -> HashMap<String, PfpCollectionEntry> {
        let alpha_art_response = self.call_all_collections_api().await;
        let mut pfp_collections: HashMap<String, PfpCollectionEntry> = HashMap::new();
        for alpha_art_collection in alpha_art_response.collections {
            let mut slug: HashMap<String, String> = HashMap::new();
            slug.insert("ALPHA_ART".parse().unwrap(), alpha_art_collection.slug);


            let mut twitter: Option<String> = None;
            let mut discord: Option<String> = None;
            let mut website: Option<String> = None;
            match alpha_art_collection.links {
                Some(alpha_art_links) => {
                    for link in alpha_art_links {
                        if link.contains("twitter.com") {
                            twitter = Some(link);
                        }
                        else if link.contains("discord.gg") {
                            discord = Some(link);
                        }
                        else if !link.contains("instagram.com") && !link.contains("howrare.is") {
                            website = Some(link)
                        }
                    }
                }
                _ => {}
            }

            let collection = PfpCollectionEntry {
                name: String::from(alpha_art_collection.title.to_lowercase().trim()),
                slug,
                website,
                twitter,
                discord,
                suggestions: Vec::new()
            };
            match pfp_collections.get(&*collection.name.to_lowercase()) {
                Some(p) => println!("WARNING [Alpha Art]: collection already exists: {:?}", p),
                None => {
                    pfp_collections.insert(collection.name.to_lowercase(), collection);
                },
            }
        }
        println!("Alpha Art Pfp collection size: {}", pfp_collections.len());
        pfp_collections
    }

}

