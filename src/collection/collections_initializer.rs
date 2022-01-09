use std::borrow::Borrow;
use std::collections::{HashMap};
use csv::WriterBuilder;
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
                let new_url = url.split_at(url.len() - 1);
                return String::from(new_url.0);
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

    println!("strip_if_present1: {}", strip_backslash_if_present(String::from("https://www.spacekitties.xyz/")));
    println!("strip_if_present2: {}", strip_backslash_if_present(String::from("https://www.spacekitties.xyz")));
    println!("strip_if_present2: {}", strip_backslash_if_present(String::from(".")));
    println!("strip_if_present2: {}", strip_backslash_if_present(String::from("")));
    println!("strip_if_present2: {}", strip_backslash_if_present(String::from("hellooo//")));

    for (magic_eden_name, magic_eden_collection) in magic_eden {
        let name = magic_eden_name.clone();
        let mut website = magic_eden_collection.website.clone();
        let mut twitter = magic_eden_collection.twitter.clone();
        let mut discord = magic_eden_collection.discord.clone();
        let mut slug = magic_eden_collection.slug.clone();
        match solanart.get(magic_eden_name.clone().as_str()) {
            Some(pfp_collection) => {
                slug.insert(SOLANART.parse().unwrap(), pfp_collection.slug.get(SOLANART).unwrap().to_string());
                website = if website.is_none() && pfp_collection.website.is_some() { pfp_collection.website.clone() } else { website };
                twitter = if twitter.is_none() && pfp_collection.twitter.is_some() { pfp_collection.twitter.clone() } else { twitter };
                discord = if discord.is_none() && pfp_collection.discord.is_some() { pfp_collection.discord.clone() } else { discord };
            },
            None => for (_, solanart_collection) in solanart.clone() {
                match_on_attributes(SOLANART, &website, &twitter, &discord, &mut slug, solanart_collection)
            }
        }
        match digital_eyes.get(magic_eden_name.clone().as_str()) {
            Some(pfp_collection) => {
                slug.insert(DIGITAL_EYES.parse().unwrap(), pfp_collection.slug.get(DIGITAL_EYES).unwrap().to_string());
                website = if website.is_none() && pfp_collection.website.is_some() { pfp_collection.website.clone() } else { website };
                twitter = if twitter.is_none() && pfp_collection.twitter.is_some() { pfp_collection.twitter.clone() } else { twitter };
                discord = if discord.is_none() && pfp_collection.discord.is_some() { pfp_collection.discord.clone() } else { discord };
            },
            None => for (_, digital_eyes_collection) in digital_eyes.clone() {
                match_on_attributes(DIGITAL_EYES, &website, &twitter, &discord, &mut slug, digital_eyes_collection)
            }
        }
        match alpha_art.get(magic_eden_name.clone().as_str()) {
            Some(pfp_collection) => {
                slug.insert(ALPHA_ART.parse().unwrap(), pfp_collection.slug.get(ALPHA_ART).unwrap().to_string());
                website = if website.is_none() && pfp_collection.website.is_some() { pfp_collection.website.clone() } else { website };
                twitter = if twitter.is_none() && pfp_collection.twitter.is_some() { pfp_collection.twitter.clone() } else { twitter };
                discord = if discord.is_none() && pfp_collection.discord.is_some() { pfp_collection.discord.clone() } else { discord };
            },
            None => for (_, alpha_art_collection) in alpha_art.clone() {
                match_on_attributes(ALPHA_ART, &website, &twitter, &discord, &mut slug, alpha_art_collection)
            }
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
    write_combined_collections_to_csv(&mut pfp_collections_combined);


    pfp_collections_combined
}

fn write_combined_collections_to_csv(pfp_collections_combined: &mut HashMap<String, PfpCollection>) {
    let mut wtr = WriterBuilder::new().from_path("combined_collections.csv").unwrap();
    wtr.write_record(&[
        "name",
        "website",
        "twitter",
        "discord",
        "magic_eden_slug",
        "solanart_slug",
        "digital_eys_slug",
        "alpha_art_slug",
    ]).unwrap();
    for (_, collection) in pfp_collections_combined {
        wtr.write_record(&[
            collection.to_owned().name,
            collection.to_owned().website.unwrap_or(String::from("")),
            collection.to_owned().twitter.unwrap_or(String::from("")),
            collection.to_owned().discord.unwrap_or(String::from("")),
            collection.to_owned().slug.get(MAGIC_EDEN).unwrap_or(&String::from("")).to_string(),
            collection.to_owned().slug.get(SOLANART).unwrap_or(&String::from("")).to_string(),
            collection.to_owned().slug.get(DIGITAL_EYES).unwrap_or(&String::from("")).to_string(),
            collection.to_owned().slug.get(ALPHA_ART).unwrap_or(&String::from("")).to_string()
        ]).unwrap();
    }
}

fn match_on_attributes(col_name: &str, website: &Option<String>, twitter: &Option<String>, discord: &Option<String>, slug: &mut HashMap<String, String>, pfp_collection: PfpCollection) {
    match pfp_collection.website {
        Some(mut sol_website_name) => match &website {
            Some(source_website_name) =>
                if sol_website_name.clone().len() > 1 && strip_backslash_if_present(sol_website_name.clone()).eq(&strip_backslash_if_present(source_website_name.clone())) {
                    println!("Website match found: source_website_name: {}, sol_website_name: {}, collection: {}", source_website_name, sol_website_name, col_name);
                    slug.insert(col_name.parse().unwrap(), pfp_collection.slug.get(col_name).unwrap().to_string());
                },
            _ => {}
        },
        _ => {}
    }
    match pfp_collection.twitter {
        Some(mut sol_twitter_name) => match &twitter {
            Some(source_twitter_name) =>
                if sol_twitter_name.clone().len() > 1 && strip_backslash_if_present(sol_twitter_name.clone()).eq(&strip_backslash_if_present(source_twitter_name.clone())) {
                    println!("Twitter match found: source_twitter_name: {}, sol_twitter_name: {}, collection: {}", source_twitter_name, sol_twitter_name.clone(), col_name.clone());
                    slug.insert(col_name.parse().unwrap(), pfp_collection.slug.get(col_name).unwrap().to_string());
                },
            _ => {}
        },
        _ => {}
    }
    match pfp_collection.discord {
        Some(mut sol_discord_name) => match &discord {
            Some(source_discord_name) =>
                if sol_discord_name.clone().len() > 1 && strip_backslash_if_present(sol_discord_name.clone()).eq(&strip_backslash_if_present(source_discord_name.clone())) {
                    println!("Discord match found: source_discord_name: {}, sol_discord_name: {}, collection: {}", source_discord_name, sol_discord_name.clone(), col_name.clone());
                    slug.insert(col_name.parse().unwrap(), pfp_collection.slug.get(col_name).unwrap().to_string());
                },
            _ => {}
        },
        _ => {}
    }
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

        let collection = PfpCollection{
            name: alpha_art_collection.title.to_lowercase(),
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