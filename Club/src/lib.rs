use std::collections::HashMap;
use std::hash::Hash;

use near_sdk::{log, near};
use near_sdk::{env, near_bindgen, AccountId, Promise, PanicOnDefault};
pub mod club;

use club::{verify_nft, Chat, Plaza};
use club::verify_nft::Verify_nft;
use club::content::Profile;

#[near(serializers = [borsh, json])]
pub struct Club {
    name: String,
    verify_nft: Vec<Verify_nft>, 
    plaza: Plaza,
    chat: Chat,
    profiles: HashMap<AccountId, Profile>,
}

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    main: HashMap<usize, Club>,
}

// Define the default, which automatically initializes the contract

/*
impl Default for Contract {
    fn default() -> Self {
        
    }
}
 */

 


// Implement the contract structure
#[near]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    pub fn send_chat(&mut self, club_numb: usize, accountid: AccountId, content: String, image_url: String ) {
        match self.get_profile(&club_numb, accountid) {
            Some(x) => {
                self.main.get(&club_numb).and_then(|club| club.chat.send_message(x, content, image_url))
            },
            None => None
            
        }
    }

    pub fn make_profile(){

    }

    pub fn get_profile(&self, club_numb: &usize, accountid: AccountId) -> Profile {
        let x = self.main.
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn make_club(&mut self, name: String, verify_nft: Vec<Verify_nft>) {
        let plaza = Plaza::new();
        let chat = Chat::new();

    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

}
