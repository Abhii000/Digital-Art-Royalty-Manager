#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, Address};

#[contracttype]
#[derive(Clone)]
pub struct Artwork {
    pub artwork_id: u64,
    pub creator: Address,
    pub title: String,
    pub royalty_percentage: u32,  // Royalty as a percentage of the sale price
    pub created_at: u64,
}

#[contracttype]
pub enum ArtworkKey {
    Artwork(u64),
}

#[contract]
pub struct DigitalArtRoyaltyManager;

#[contractimpl]
impl DigitalArtRoyaltyManager {
    
    pub fn create_artwork(
        env: Env,
        creator: Address,
        title: String,
        royalty_percentage: u32
    ) -> u64 {
        // Authenticate the creator
        creator.require_auth();

        // Validate royalty percentage
        if royalty_percentage > 100 {
            log!(&env, "Invalid royalty percentage");
            panic!("Royalty percentage must be between 0 and 100");
        }

        // Generate a new artwork ID
        let artwork_id: u64 = env.storage().instance().get(&Symbol::short("ARTWORK_COUNT")).unwrap_or(0) + 1;

        // Create the artwork
        let artwork = Artwork {
            artwork_id,
            creator: creator.clone(),
            title,
            royalty_percentage,
            created_at: env.ledger().timestamp(),
        };

        // Store the artwork
        env.storage().instance().set(&ArtworkKey::Artwork(artwork_id), &artwork);
        
        // Update artwork count
        env.storage().instance().set(&Symbol::short("ARTWORK_COUNT"), &(artwork_id));
        
        log!(&env, "Artwork created with ID: {}", artwork_id);
        
        artwork_id
    }

    pub fn transfer_artwork(
        env: Env,
        buyer: Address,
        artwork_id: u64,
        sale_price: u64
    ) -> bool {
        // Authenticate the buyer
        buyer.require_auth();
        
        // Get the artwork
        let mut artwork = Self::get_artwork(env.clone(), artwork_id);

        // Validate artwork exists
        if artwork.artwork_id == 0 {
            log!(&env, "Artwork does not exist");
            return false;
        }

        // Calculate royalty to be paid to the creator
        let royalty_amount = (sale_price * artwork.royalty_percentage as u64) / 100;

        // Simulate transferring the sale price to the buyer and royalty to the creator
        log!(&env, "Buyer: {} purchased the artwork: {} for {}. Creator receives royalty: {}", buyer, artwork.title, sale_price, royalty_amount);
        
        // Update the artwork transfer timestamp
        artwork.created_at = env.ledger().timestamp();

        // Store updated artwork
        env.storage().instance().set(&ArtworkKey::Artwork(artwork_id), &artwork);

        true
    }

pub fn get_artwork(env: Env, artwork_id: u64) -> Artwork {
    let key = ArtworkKey::Artwork(artwork_id);
    
    env.storage().instance().get(&key).unwrap_or(Artwork {
        artwork_id: 0,
        creator: Address::from_str(&env,"GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"), // Dummy address for example
        title: String::from_str(&env, ""),
        royalty_percentage: 0,
        created_at: 0,
    })
}
}
