use std::borrow::{Borrow};
use std::collections::{HashMap};

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

use crate::{handle_alpha_art, handle_digitaleyes, handle_magiceden, handle_solanart};

#[derive(Debug)]
pub struct PfpCollection {
    // Solanart: name
    // Digital Eyes: name
    // Magic Eden: name
    // Alpha Art: title
    pub name: String,

    // Solanart: url
    // Digital Eyes: name
    // Magic Eden: symbol
    // Alpha Art: slug
    pub slug: HashMap<String, String>,

    // Solanart: website OR urlsoon (this could also be twitter, have to check)
    // Digital Eyes: website (this could also be twitter)
    // Magic Eden: website
    // Alpha Art: links (list)
    pub website: Option<String>,

    // Solanart: twitter
    // Digital Eyes: website (this could also be website or discord)
    // Magic Eden: twitter
    // Alpha Art: links (list)
    pub twitter: Option<String>,

    // Solanart: discord
    // Digital Eyes: website (this could also be website or twitter)
    // Magic Eden: discord
    // Alpha Art: links (list)
    pub discord: Option<String>,

    // These are suggestions to be able to map the collection from one marketplace
    // to another if no automatic match could be found
    pub suggestions: Vec<String>
}

pub async fn populate_digitaleyes(collection_name: &String, pfp_collections: &HashMap<String, PfpCollection>) -> String {
    return match pfp_collections.get(collection_name) { // get the collection name from map of collections
        Some(pfp_col) => {
            match pfp_col.slug.get("DIGITAL_EYES") { // check if there exists an api slug mapping for Digital Eyes
                None => String::from(""),
                Some(dig_eyes_slug) => handle_digitaleyes(dig_eyes_slug.to_string()).await
            }
        }
        None => String::from("")
    }
}


pub async fn populate_solanart(collection_name: &String, pfp_collections: &HashMap<String, PfpCollection>) -> String {
    return match pfp_collections.get(collection_name) { // get the collection name from map of collections
        Some(pfp_col) => {
            match pfp_col.slug.get("SOLANART") { // check if there exists an api slug mapping for Solanart
                None => String::from(""),
                Some(solanart_slug) => handle_solanart(solanart_slug.to_string()).await
            }
        }
        None => String::from("")
    }
}

pub async fn populate_magiceden(collection_name: &String, pfp_collections: &HashMap<String, PfpCollection>) -> String {
    return match pfp_collections.get(collection_name) { // get the collection name from map of collections
        Some(pfp_col) => {
            match pfp_col.slug.get("MAGIC_EDEN") { // check if there exists an api slug mapping for Magic Eden
                None => String::from(""),
                Some(magic_eden_slug) => handle_magiceden(magic_eden_slug.to_string()).await
            }
        }
        None => String::from("")
    }
}

pub async fn populate_alphaart(collection_name: &String, pfp_collections: &HashMap<String, PfpCollection>) -> String {
    return match pfp_collections.get(collection_name) { // get the collection name from map of collections
        Some(pfp_col) => {
            match pfp_col.slug.get("ALPHA_ART") { // check if there exists an api slug mapping for Magic Eden
                None => String::from(""),
                Some(alpha_art_slug) => handle_alpha_art(alpha_art_slug.to_string()).await
            }
        }
        None => String::from("")
    }
}

pub async fn check_if_collection_exists_or_give_suggestions<'a>(all_collections_map: &'a HashMap<String, PfpCollection>, collection_name: &'a str) -> (bool, Vec<&'a str>) {
    return match all_collections_map.get(collection_name) { // get the collection name from map of collections
        Some(_) => {
            (true, vec![])
        }
        None => {
            let mut suggestions = vec![];
            let matcher = SkimMatcherV2::default();
            for (collection_from_map, _) in all_collections_map {
                if fuz_match(matcher.borrow(), collection_from_map.as_str(), collection_name) > 50 {
                    suggestions.push(collection_from_map.as_str())
                }
            }
            (false, suggestions)
        }
    }
}

fn fuz_match(matcher: &SkimMatcherV2, str1: &str, str2: &str) -> u64 {
    if str1.len() > str2.len() {
        if let Some((score, _)) = matcher.fuzzy_indices(str1, str2){
            println!("Score {},{} = {}", str1, str2, score);
            score as u64
        }
        else {
            0 as u64
        }
    }
    else {
        if let Some((score, _)) = matcher.fuzzy_indices(str2, str1){
            println!("Score {},{} = {}", str2, str1, score);
            score as u64
        }
        else {
            0 as u64
        }
    }
}