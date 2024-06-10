use std::clone;
use near_sdk::store::vec;
// Find all our documentation at https://docs.near.org
use near_sdk::{log, near, AccountId};
pub mod content;
pub mod verify_nft;
pub mod token;
use crate::club::content::Profile;


use content::Article;
use content::Message;

#[near(serializers = [borsh, json])]
pub struct Plaza {
    meta: Vec<Article>
}

#[near(serializers = [borsh, json])]
pub struct Chat {
    meta: Vec<Message>
}


impl Plaza {
    pub fn new() -> Self{
        Plaza{
            meta: vec![]
        }
    }
}

impl Chat {
    pub fn new() -> Self{
        Chat{
            meta: vec![]
        }
    }

    pub fn send_message(&mut self, profile: Profile ,content: String, image_url: String,) {
        self.meta.push(
            Message::Create_Message(profile, content, image_url)
        )

    }
}

