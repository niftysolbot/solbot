use std::collections::{HashMap};
use csv::WriterBuilder;
use crate::{PfpCollectionEntry};

pub const MAGIC_EDEN: &str = "MAGIC_EDEN";
pub const SOLANART: &str = "SOLANART";
pub const DIGITAL_EYES: &str = "DIGITAL_EYES";
pub const ALPHA_ART: &str = "ALPHA_ART";

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

pub async fn combine_pfp_collections(master_collection: HashMap<String, PfpCollectionEntry>,
                                     first_collection: &HashMap<String, PfpCollectionEntry>,
                                     second_collection: &HashMap<String, PfpCollectionEntry>,
                                     third_collection: &HashMap<String, PfpCollectionEntry>,
                                     collection_order: (&str, &str, &str)) -> HashMap<String, PfpCollectionEntry> {
    let mut pfp_collections_combined: HashMap<String, PfpCollectionEntry> = HashMap::new();

    for (name_from_master_collection, master_pfp_collection) in master_collection {
        let name = name_from_master_collection.clone();
        let mut website = master_pfp_collection.website.clone();
        let mut twitter = master_pfp_collection.twitter.clone();
        let mut discord = master_pfp_collection.discord.clone();
        let mut slug = master_pfp_collection.slug.clone();
        match first_collection.get(name_from_master_collection.clone().as_str()) {
            Some(pfp_collection) => {
                slug.insert(collection_order.0.parse().unwrap(), pfp_collection.slug.get(collection_order.0).unwrap().to_string());
                website = if website.is_none() && pfp_collection.website.is_some() { pfp_collection.website.clone() } else { website };
                twitter = if twitter.is_none() && pfp_collection.twitter.is_some() { pfp_collection.twitter.clone() } else { twitter };
                discord = if discord.is_none() && pfp_collection.discord.is_some() { pfp_collection.discord.clone() } else { discord };
            },
            None => for (_, solanart_collection) in first_collection.clone() {
                match_on_attributes(name_from_master_collection.clone().as_str(), collection_order.0, &website, &twitter, &discord, &mut slug, solanart_collection)
            }
        }
        match second_collection.get(name_from_master_collection.clone().as_str()) {
            Some(pfp_collection) => {
                slug.insert(collection_order.1.parse().unwrap(), pfp_collection.slug.get(collection_order.1).unwrap().to_string());
                website = if website.is_none() && pfp_collection.website.is_some() { pfp_collection.website.clone() } else { website };
                twitter = if twitter.is_none() && pfp_collection.twitter.is_some() { pfp_collection.twitter.clone() } else { twitter };
                discord = if discord.is_none() && pfp_collection.discord.is_some() { pfp_collection.discord.clone() } else { discord };
            },
            None => for (_, digital_eyes_collection) in second_collection.clone() {
                match_on_attributes(name_from_master_collection.clone().as_str(),collection_order.1, &website, &twitter, &discord, &mut slug, digital_eyes_collection)
            }
        }
        match third_collection.get(name_from_master_collection.clone().as_str()) {
            Some(pfp_collection) => {
                slug.insert(collection_order.2.parse().unwrap(), pfp_collection.slug.get(collection_order.2).unwrap().to_string());
                website = if website.is_none() && pfp_collection.website.is_some() { pfp_collection.website.clone() } else { website };
                twitter = if twitter.is_none() && pfp_collection.twitter.is_some() { pfp_collection.twitter.clone() } else { twitter };
                discord = if discord.is_none() && pfp_collection.discord.is_some() { pfp_collection.discord.clone() } else { discord };
            },
            None => for (_, alpha_art_collection) in third_collection.clone() {
                match_on_attributes(name_from_master_collection.clone().as_str(),collection_order.2, &website, &twitter, &discord, &mut slug, alpha_art_collection)
            }
        }
        pfp_collections_combined.insert(name, PfpCollectionEntry {
            name: name_from_master_collection.clone(),
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

fn write_combined_collections_to_csv(pfp_collections_combined: &mut HashMap<String, PfpCollectionEntry>) {
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

fn match_on_attributes(name_from_master_collection: &str, marketplace_name: &str, website: &Option<String>, twitter: &Option<String>, discord: &Option<String>, slug: &mut HashMap<String, String>, pfp_collection: PfpCollectionEntry) {
    if remove_whitespace(name_from_master_collection).to_lowercase() == remove_whitespace(pfp_collection.name.as_str()).to_lowercase() {
        println!("removed_whitespace_check_true. master_col_name: {}, pfp_col_name: {}", name_from_master_collection, pfp_collection.name);
        slug.insert(marketplace_name.parse().unwrap(), pfp_collection.slug.get(marketplace_name).unwrap().to_string());
    }
    match pfp_collection.website {
        Some(mut sol_website_name) => match &website {
            Some(source_website_name) =>
                if sol_website_name.clone().len() > 1 && strip_backslash_if_present(sol_website_name.clone()).eq(&strip_backslash_if_present(source_website_name.clone())) {
                    println!("Website match found: source_website_name: {}, sol_website_name: {}, marketplace: {}", source_website_name, sol_website_name, marketplace_name);
                    slug.insert(marketplace_name.parse().unwrap(), pfp_collection.slug.get(marketplace_name).unwrap().to_string());
                },
            _ => {}
        },
        _ => {}
    }
    match pfp_collection.twitter {
        Some(mut sol_twitter_name) => match &twitter {
            Some(source_twitter_name) =>
                if sol_twitter_name.clone().len() > 1 && strip_backslash_if_present(sol_twitter_name.clone()).eq(&strip_backslash_if_present(source_twitter_name.clone())) {
                    println!("Twitter match found: source_twitter_name: {}, sol_twitter_name: {}, collection: {}", source_twitter_name, sol_twitter_name.clone(), marketplace_name.clone());
                    slug.insert(marketplace_name.parse().unwrap(), pfp_collection.slug.get(marketplace_name).unwrap().to_string());
                },
            _ => {}
        },
        _ => {}
    }
    match pfp_collection.discord {
        Some(mut sol_discord_name) => match &discord {
            Some(source_discord_name) =>
                if sol_discord_name.clone().len() > 1 && strip_backslash_if_present(sol_discord_name.clone()).eq(&strip_backslash_if_present(source_discord_name.clone())) {
                    println!("Discord match found: source_discord_name: {}, sol_discord_name: {}, collection: {}", source_discord_name, sol_discord_name.clone(), marketplace_name.clone());
                    slug.insert(marketplace_name.parse().unwrap(), pfp_collection.slug.get(marketplace_name).unwrap().to_string());
                },
            _ => {}
        },
        _ => {}
    }
}

fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<String>()
}