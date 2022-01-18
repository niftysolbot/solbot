use serenity::{
    async_trait,
};
use std::collections::{HashMap};
use crate::collection::all_collections_handling::PfpCollectionEntry;

#[async_trait]
pub trait MarketplaceCollection {
    // Associated function signature; `Self` refers to the implementor type.
    fn new(name: String) -> Self;

    // Method signatures; these will return a string.
    fn name(&self) -> String;

    // Traits can provide default method definitions.
    async fn get_floor_from_api(&mut self, pfp_collection: &PfpCollectionEntry) -> String {
        println!("Un-implemented get_floor_from_api: marketplace: {}, pfp_collection: {}", self.name(), pfp_collection.name);
        String::from("Unimplemented")
    }

    async fn initialize_pfp_collections(&mut self) -> HashMap<String, PfpCollectionEntry>;
}