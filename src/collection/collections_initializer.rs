use std::borrow::Borrow;
use std::collections::{HashMap};
use crate::PfpCollection;
use crate::alpha_art::alpha_art_api::alpha_art_process_all_collections_api;
use crate::digital_eyes::digitaleyes_api::digital_eyes_process_all_collections_api;
use crate::magiceden::magiceden_api::magic_eden_process_all_collections_api;
use crate::solanart::solanart_api::solanart_process_all_collections_api;

const MAGIC_EDEN: &str = "MAGIC_EDEN";
const SOLANART: &str = "SOLANART";
const DIGITAL_EYES: &str = "DIGITAL_EYES";
const ALPHA_ART: &str = "ALPHA_ART";

pub fn strip_backslash_if_present(mut url: String) -> String {
    match url.chars().last() {
        Some(u) => {
            if u == '/' {
                let new_url = String::from(url.pop().unwrap()).clone();
                return new_url.clone();
            }
        },
        _ => {}
    }
    return url.clone();
}

pub async fn combine_pfp_collections_base_magic_eden(magic_eden: HashMap<String, PfpCollection>,
                                                     solanart: HashMap<String, PfpCollection>,
                                                     digital_eyes: HashMap<String, PfpCollection>,
                                                     alpha_art: HashMap<String, PfpCollection>) -> HashMap<String, PfpCollection> {
    let mut pfp_collections_combined: HashMap<String, PfpCollection> = HashMap::new();

    for (magic_eden_name, magic_eden_collection) in magic_eden {
        let name = magic_eden_name.clone();
        let website = magic_eden_collection.website.clone();
        let twitter = magic_eden_collection.twitter.clone();
        let discord = magic_eden_collection.discord.clone();
        let mut slug = magic_eden_collection.slug.clone();
        match solanart.get(magic_eden_name.clone().as_str()) {
            Some(pfp_collection) => {
                slug.insert(SOLANART.parse().unwrap(), pfp_collection.slug.get(SOLANART).unwrap().to_string());
            },
            // _ => {}
            None => { // Try to match by website, twitter, or discord
                for (solanart_name, solanart_collection) in solanart.clone() {
                    match solanart_collection.website {
                        Some(mut sol_website_name) => {
                            match &website {
                                Some(source_website_name) => {
                                    if strip_backslash_if_present(sol_website_name) == strip_backslash_if_present(source_website_name.clone()) {
                                        slug.insert(SOLANART.parse().unwrap(), solanart_collection.slug.get(SOLANART).unwrap().to_string());
                                    }
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                    match solanart_collection.twitter {
                        Some(mut sol_twitter_name) => {
                            match &twitter {
                                Some(source_twitter_name) => {
                                    if strip_backslash_if_present(sol_twitter_name) == strip_backslash_if_present(source_twitter_name.clone()) {
                                        slug.insert(SOLANART.parse().unwrap(), solanart_collection.slug.get(SOLANART).unwrap().to_string());
                                    }
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
        match digital_eyes.get(magic_eden_name.clone().as_str()) {
            Some(pfp_collection) => {
                slug.insert(DIGITAL_EYES.parse().unwrap(), pfp_collection.slug.get(DIGITAL_EYES).unwrap().to_string());
            },
            _ => {}
        }
        match alpha_art.get(magic_eden_name.clone().as_str()) {
            Some(pfp_collection) => {
                slug.insert(ALPHA_ART.parse().unwrap(), pfp_collection.slug.get(ALPHA_ART).unwrap().to_string());
            },
            _ => {}
        }
        pfp_collections_combined.insert(name, PfpCollection {
            name: magic_eden_name.clone(),
            slug,
            website,
            twitter,
            discord,
            suggestions: vec![]
        });
    }
    pfp_collections_combined
}

pub async fn initialize_pfp_collection_from_digital_eyes() -> HashMap<String, PfpCollection> {
    let digital_eyes_response = digital_eyes_process_all_collections_api().await;
    let mut pfp_collections: HashMap<String, PfpCollection> = HashMap::new();
    for digital_eyes_collection in digital_eyes_response {
        let mut slug: HashMap<String, String> = HashMap::new();
        slug.insert(DIGITAL_EYES.parse().unwrap(), digital_eyes_collection.name.clone());

        let collection = PfpCollection{
            name: digital_eyes_collection.name.to_lowercase(),
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


pub async fn initialize_pfp_collection_from_solanart() -> HashMap<String, PfpCollection> {
    let solanart_response = solanart_process_all_collections_api().await;
    let mut pfp_collections: HashMap<String, PfpCollection> = HashMap::new();
    for solanart_collection in solanart_response {
        let mut slug: HashMap<String, String> = HashMap::new();
        slug.insert(SOLANART.parse().unwrap(), solanart_collection.url);

        let collection = PfpCollection{
            name: solanart_collection.name.to_lowercase(),
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



pub async fn initialize_pfp_collection_from_magic_eden() -> HashMap<String, PfpCollection> {
    let magic_eden_response = magic_eden_process_all_collections_api().await;
    let mut pfp_collections: HashMap<String, PfpCollection> = HashMap::new();
    for magic_eden_collection in magic_eden_response.collections {
        let mut slug: HashMap<String, String> = HashMap::new();
        slug.insert(MAGIC_EDEN.parse().unwrap(), magic_eden_collection.symbol);
        let collection = PfpCollection{
            name: magic_eden_collection.name.to_lowercase(),
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


pub async fn initialize_pfp_collection_from_alpha_art() -> HashMap<String, PfpCollection> {
    let alpha_art_response = alpha_art_process_all_collections_api().await;
    let mut pfp_collections: HashMap<String, PfpCollection> = HashMap::new();
    for alpha_art_collection in alpha_art_response.collections {
        let mut slug: HashMap<String, String> = HashMap::new();
        slug.insert(ALPHA_ART.parse().unwrap(), alpha_art_collection.slug);
        let collection = PfpCollection{
            name: alpha_art_collection.title.to_lowercase(),
            slug,
            website: None,
            twitter: None,
            discord: None,
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