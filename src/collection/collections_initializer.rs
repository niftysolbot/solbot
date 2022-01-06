use std::collections::{HashMap};
use crate::PfpCollection;
use crate::alpha_art::alpha_art_all_collection_response::AlphaArtAllCollectionResponse;
use crate::alpha_art::alpha_art_api::alpha_art_process_all_collections_api;
use crate::digital_eyes::digital_eyes_all_collection_response::DigitalEyesAllCollectionResponse;
use crate::digital_eyes::digitaleyes_api::digital_eyes_process_all_collections_api;
use crate::magiceden::magiceden_all_collection_response::MagicEdenAllCollectionsResponse;
use crate::magiceden::magiceden_api::magic_eden_process_all_collections_api;
use crate::solanart::solanart_all_collection_response::SolanartAllCollectionResponse;
use crate::solanart::solanart_api::solanart_process_all_collections_api;

const MAGIC_EDEN: &str = "MAGIC_EDEN";
const SOLANART: &str = "SOLANART";
const DIGITAL_EYES: &str = "DIGITAL_EYES";
const ALPHA_ART: &str = "ALPHA_ART";

pub async fn combine_pfp_collections(magic_eden: HashMap<String, PfpCollection>,
                                     solanart: HashMap<String, PfpCollection>,
                                     digital_eyes: HashMap<String, PfpCollection>,
                                     alpha_art: HashMap<String, PfpCollection>) -> HashMap<String, PfpCollection> {
    let mut pfp_collections_combined: HashMap<String, PfpCollection> = HashMap::new();


    for (magic_eden_name, magic_eden_collection) in magic_eden {
        //pfp_collections_combined.insert(magic_eden_name.clone(), PfpCollection::new(magic_eden_collection));
        let name = magic_eden_name.clone();
        let mut slug = magic_eden_collection.slug.clone();
        // let twitter: String;
        // match magic_eden_collection.twitter {
        //     Some(the_twitter) => twitter = the_twitter,
        //     _ => {}
        // }
        match solanart.get(magic_eden_name.clone().as_str()) {
            Some(pfp_collection) => {
                slug.insert(SOLANART.parse().unwrap(), pfp_collection.slug.get(SOLANART).unwrap().to_string());
            },
            _ => {}
            // None => { // Try to match by website, twitter, or discord
            //     for (solanart_name, solanart_collection) in solanart {
            //         let sol_website_name: &str;
            //         if solanart_collection.website.chars().last().unwrap() == "/" {
            //             sol_website_name = solanart_collection.website.pop();
            //         }
            //         else {
            //             sol_website_name = solanart_collection.website;
            //         }
            //         if solanart_collection.website == magic_eden_collection.website {
            //
            //         }
            //     }
            // }
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
            website: magic_eden_collection.website,
            twitter: magic_eden_collection.twitter,
            discord: magic_eden_collection.discord,
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