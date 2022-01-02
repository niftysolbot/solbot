use std::borrow::Borrow;
use std::collections::{HashMap};
use super::solanart::solanart_all_collection_response::SolanartAllCollectionResponse;
use super::magiceden::magiceden_all_collection_response::MagicEdenAllCollectionsResponse;
use super::digital_eyes::digital_eyes_all_collection_response::DigitalEyesAllCollectionResponse;
use super::alpha_art::alpha_art_all_collection_response::AlphaArtAllCollectionResponse;

const MAGIC_EDEN: &str = "MAGIC_EDEN";
const SOLANART: &str = "SOLANART";
const DIGITAL_EYES: &str = "DIGITAL_EYES";
const ALPHA_ART: &str = "ALPHA_ART";

#[derive(Debug)]
pub struct PfpCollection {
    // Solanart: name
    // Digital Eyes: name
    // Magic Eden: name
    // Alpha Art: title
    name: String,

    // Solanart: url
    // Digital Eyes: name
    // Magic Eden: symbol
    // Alpha Art: slug
    pub(crate) slug: HashMap<String, String>,

    // Solanart: website OR urlsoon (this could also be twitter, have to check)
    // Digital Eyes: website (this could also be twitter)
    // Magic Eden: website
    // Alpha Art: links (list)
    website: Option<String>,

    // Solanart: twitter
    // Digital Eyes: website (this could also be website or discord)
    // Magic Eden: twitter
    // Alpha Art: links (list)
    twitter: Option<String>,

    // Solanart: discord
    // Digital Eyes: website (this could also be website or twitter)
    // Magic Eden: discord
    // Alpha Art: links (list)
    discord: Option<String>,

    // These are suggestions to be able to map the collection from one marketplace
    // to another if no automatic match could be found
    suggestions: Vec<String>
}

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
            Some(s) => {
                let w = s.slug.get(SOLANART).unwrap();
                slug.insert(SOLANART.parse().unwrap(), w.to_string());
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
            Some(t) => {
                let w = t.slug.get(DIGITAL_EYES).unwrap();
                slug.insert(DIGITAL_EYES.parse().unwrap(), w.to_string());
            },
            _ => {}
        }
        match alpha_art.get(magic_eden_name.clone().as_str()) {
            Some(t) => {
                let w = t.slug.get(ALPHA_ART).unwrap();
                slug.insert(ALPHA_ART.parse().unwrap(), w.to_string());
            },
            _ => {}
        }
        pfp_collections_combined.insert(name, PfpCollection {
            name: magic_eden_name.clone(),
            slug,
            website: None,
            twitter: None,
            discord: None,
            suggestions: vec![]
        });
    }
    pfp_collections_combined
}



pub async fn initialize_pfp_collection_from_digital_eyes(digital_eyes_collections: DigitalEyesAllCollectionResponse) -> HashMap<String, PfpCollection> {
    let mut pfp_collections: HashMap<String, PfpCollection> = HashMap::new();
    for digital_eyes_collection in digital_eyes_collections {
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




pub async fn initialize_pfp_collection_from_solanart(solanart_collections: SolanartAllCollectionResponse) -> HashMap<String, PfpCollection> {
    let mut pfp_collections: HashMap<String, PfpCollection> = HashMap::new();
    for solanart_collection in solanart_collections {
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




pub async fn initialize_pfp_collection_from_magic_eden(magic_eden: MagicEdenAllCollectionsResponse) -> HashMap<String, PfpCollection> {
    let mut pfp_collections: HashMap<String, PfpCollection> = HashMap::new();
    for magic_eden_collection in magic_eden.collections {
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




pub async fn initialize_pfp_collection_from_alpha_art(alpha_art: AlphaArtAllCollectionResponse) -> HashMap<String, PfpCollection> {
    let mut pfp_collections: HashMap<String, PfpCollection> = HashMap::new();
    for alpha_art_collection in alpha_art.collections {
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