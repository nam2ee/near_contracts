use std::clone;
use near_sdk::base64::alphabet::IMAP_MUTF7;
// Find all our documentation at https://docs.near.org
use near_sdk::{log, near, AccountId};
use crate::club::token::Token;
use crate::club::verify_nft::Verify_nft;

#[near(serializers = [borsh, json])]
pub struct Message{ 
    sender: Profile,
    content: String,
    image_url: String,
}

#[near(serializers = [borsh, json])]
pub struct Article { 
    owner: Profile,
    content: String,
    image_url: Vec<String>,
}

#[near(serializers = [borsh, json])]
pub struct Profile{
    account: AccountId,
    name: String,
    own_nft: Verify_nft, //nft로 변경
    own_token: Token, //Token으로 변경
}


impl Message{
    pub fn Create_Message( sender: Profile, content: String, image_url: String) -> Self{
        Message{
            sender:sender,
            content:content,
            image_url:image_url,
        }
    }
}
